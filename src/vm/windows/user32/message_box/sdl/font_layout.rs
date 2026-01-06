use fontdue::{Font, LineMetrics};

// Layout constants for the font-rendered dialog.
pub(super) const DIALOG_PAD_X: i32 = 16;
const DIALOG_PAD_Y: i32 = 16;
const DIALOG_GAP: i32 = 12;
const DIALOG_BUTTON_W: i32 = 84;
const DIALOG_BUTTON_H: i32 = 26;
const DIALOG_MIN_WIDTH: i32 = 260;
pub(super) const DIALOG_MAX_WIDTH: i32 = 560;
const DIALOG_MIN_HEIGHT: i32 = 140;

// Line of text with a cached pixel width.
pub(super) struct TextLine {
    pub(super) text: String,
    pub(super) width: f32,
}

pub(super) struct FontDialogLayout {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) pad_x: i32,
    pub(super) text_y: i32,
    pub(super) button_x: i32,
    pub(super) button_y: i32,
    pub(super) button_w: i32,
    pub(super) button_h: i32,
}

impl FontDialogLayout {
    pub(super) fn new(body_lines: &[TextLine], body_metrics: LineMetrics) -> Self {
        // Size the dialog around text width and button height.
        let mut max_line_width = 0.0f32;
        for line in body_lines {
            if line.width > max_line_width {
                max_line_width = line.width;
            }
        }
        let body_line_height = body_metrics.new_line_size.ceil() as i32;
        let line_count = body_lines.len().max(1) as i32;
        let body_height = body_line_height * line_count;
        let mut width = max_line_width.ceil() as i32 + DIALOG_PAD_X * 2;
        let min_width = DIALOG_MIN_WIDTH.max(DIALOG_BUTTON_W + DIALOG_PAD_X * 2);
        if width < min_width {
            width = min_width;
        }
        if width > DIALOG_MAX_WIDTH {
            width = DIALOG_MAX_WIDTH;
        }
        let mut height = DIALOG_PAD_Y * 2 + body_height + DIALOG_GAP + DIALOG_BUTTON_H;
        if height < DIALOG_MIN_HEIGHT {
            height = DIALOG_MIN_HEIGHT;
        }
        let text_y = DIALOG_PAD_Y;
        let button_x = (width - DIALOG_BUTTON_W) / 2;
        let button_y = height - DIALOG_PAD_Y - DIALOG_BUTTON_H;
        Self {
            width,
            height,
            pad_x: DIALOG_PAD_X,
            text_y,
            button_x,
            button_y,
            button_w: DIALOG_BUTTON_W,
            button_h: DIALOG_BUTTON_H,
        }
    }
}

pub(super) struct FontLineParams {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) color: sdl2::pixels::Color,
    pub(super) size: f32,
    pub(super) line_metrics: LineMetrics,
}

pub(super) fn line_metrics_or_default(font: &Font, size: f32) -> LineMetrics {
    // Use font line metrics when available; fall back to basic scaling.
    font.horizontal_line_metrics(size).unwrap_or(LineMetrics {
        ascent: size,
        descent: 0.0,
        line_gap: 0.0,
        new_line_size: size,
    })
}

pub(super) fn wrap_text_font(text: &str, font: &Font, size: f32, max_width: f32) -> Vec<TextLine> {
    // Wrap using font metrics (with kerning) so width calculations match rendering.
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(TextLine {
                text: String::new(),
                width: 0.0,
            });
            continue;
        }
        let mut current = String::new();
        let mut current_width = 0.0;
        for word in paragraph.split_whitespace() {
            let word_width = measure_text_width(font, size, word);
            if word_width > max_width {
                if !current.is_empty() {
                    lines.push(TextLine {
                        text: current,
                        width: current_width,
                    });
                    current = String::new();
                    current_width = 0.0;
                }
                let mut chunks = split_word(font, size, word, max_width);
                if let Some(last) = chunks.pop() {
                    for chunk in chunks {
                        lines.push(chunk);
                    }
                    current = last.text;
                    current_width = last.width;
                }
                continue;
            }
            if current.is_empty() {
                current.push_str(word);
                current_width = word_width;
                continue;
            }
            let candidate = format!("{} {}", current, word);
            let candidate_width = measure_text_width(font, size, &candidate);
            if candidate_width <= max_width {
                current = candidate;
                current_width = candidate_width;
            } else {
                lines.push(TextLine {
                    text: current,
                    width: current_width,
                });
                current = word.to_string();
                current_width = word_width;
            }
        }
        if !current.is_empty() {
            lines.push(TextLine {
                text: current,
                width: current_width,
            });
        }
    }
    if lines.is_empty() {
        lines.push(TextLine {
            text: String::new(),
            width: 0.0,
        });
    }
    lines
}

pub(super) fn measure_text_width(font: &Font, size: f32, text: &str) -> f32 {
    // Measure with kerning so layout matches glyph placement.
    let mut width = 0.0;
    let mut prev = None;
    for ch in text.chars() {
        if let Some(prev_ch) = prev {
            if let Some(kern) = font.horizontal_kern(prev_ch, ch, size) {
                width += kern;
            }
        }
        let (metrics, _) = font.rasterize(ch, size);
        width += metrics.advance_width;
        prev = Some(ch);
    }
    width
}

fn split_word(font: &Font, size: f32, word: &str, max_width: f32) -> Vec<TextLine> {
    // Split long words by glyph width so they don't overflow the dialog.
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_width = 0.0;
    let mut prev = None;
    for ch in word.chars() {
        let mut next_width = current_width;
        if let Some(prev_ch) = prev {
            if let Some(kern) = font.horizontal_kern(prev_ch, ch, size) {
                next_width += kern;
            }
        }
        let (metrics, _) = font.rasterize(ch, size);
        next_width += metrics.advance_width;
        if !current.is_empty() && next_width > max_width {
            chunks.push(TextLine {
                text: current,
                width: current_width,
            });
            current = String::new();
            let (metrics, _) = font.rasterize(ch, size);
            current.push(ch);
            current_width = metrics.advance_width;
            prev = Some(ch);
            continue;
        }
        current.push(ch);
        current_width = next_width;
        prev = Some(ch);
    }
    if !current.is_empty() {
        chunks.push(TextLine {
            text: current,
            width: current_width,
        });
    }
    chunks
}
