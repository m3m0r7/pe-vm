// Bitmap fallback layout helpers.

pub(super) fn wrap_text_columns(text: &str, max_cols: usize) -> Vec<String> {
    // Bitmap fallback wrapping by character columns.
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            let mut chunk = word.to_string();
            while chunk.chars().count() > max_cols {
                let split_at = chunk
                    .char_indices()
                    .nth(max_cols)
                    .map(|(idx, _)| idx)
                    .unwrap_or(chunk.len());
                let rest = chunk.split_off(split_at);
                if !current.is_empty() {
                    lines.push(current);
                    current = String::new();
                }
                lines.push(chunk);
                chunk = rest;
            }
            let next_len = current.chars().count() + 1 + chunk.chars().count();
            if current.is_empty() {
                current = chunk;
            } else if next_len <= max_cols {
                current.push(' ');
                current.push_str(&chunk);
            } else {
                lines.push(current);
                current = chunk;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

// Layout container for bitmap fallback rendering.
pub(super) struct BitmapDialogLayout {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) pad_x: i32,
    pub(super) text_y: i32,
    pub(super) button_x: i32,
    pub(super) button_y: i32,
    pub(super) button_w: i32,
    pub(super) button_h: i32,
    pub(super) body_scale: i32,
    pub(super) button_scale: i32,
}

impl BitmapDialogLayout {
    pub(super) fn new(body_lines: &[String]) -> Self {
        // Size the bitmap dialog from 8x8 glyph metrics.
        let pad_x = 12;
        let pad_y = 12;
        let body_scale = 1;
        let button_scale = 1;
        let font_w = 8;
        let font_h = 8;
        let gap = 8;
        let button_w = 72;
        let button_h = 24;

        let mut max_cols = 0usize;
        for line in body_lines {
            max_cols = max_cols.max(line.chars().count());
        }
        let text_width = max_cols as i32 * font_w * body_scale;
        let mut width = text_width + pad_x * 2 + 24;
        width = width.clamp(240, 640);

        let body_height = body_lines.len() as i32 * font_h * body_scale;
        let mut height = pad_y * 2 + body_height + gap + button_h;
        if height < 140 {
            height = 140;
        }

        let text_y = pad_y;
        let button_x = (width - button_w) / 2;
        let button_y = height - pad_y - button_h;

        Self {
            width,
            height,
            pad_x,
            text_y,
            button_x,
            button_y,
            button_w,
            button_h,
            body_scale,
            button_scale,
        }
    }
}

pub(super) struct BitmapLineParams {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) color: sdl2::pixels::Color,
    pub(super) scale: i32,
}
