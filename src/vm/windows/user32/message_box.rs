//! SDL-backed MessageBoxA implementation.

use crate::vm::{MessageBoxMode, Vm};

#[cfg(feature = "sdl2")]
use font8x8::UnicodeFonts;
#[cfg(feature = "sdl2")]
use fontdue::{Font, FontSettings, LineMetrics};

pub fn register(vm: &mut Vm) {
    // Expose MessageBoxA to guest imports as a stdcall host function.
    vm.register_import_stdcall("USER32.dll", "MessageBoxA", crate::vm::stdcall_args(4), message_box_a);
}

pub(crate) fn message_box_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Read Win32 MessageBoxA arguments from the guest stack.
    let text_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    let caption_ptr = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let _utype = vm.read_u32(stack_ptr.wrapping_add(16)).unwrap_or(0);
    let text = if text_ptr != 0 {
        vm.read_c_string(text_ptr).unwrap_or_default()
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        vm.read_c_string(caption_ptr).unwrap_or_default()
    } else {
        String::new()
    };
    // Dispatch based on the configured message box mode.
    match vm.message_box_mode() {
        MessageBoxMode::Stdout => {
            if caption.is_empty() {
                vm.write_stdout(&text);
                vm.write_stdout("\n");
            } else {
                vm.write_stdout(&format!("{}: {}\n", caption, text));
            }
        }
        MessageBoxMode::Dialog => {
            let _ = show_dialog(vm, &caption, &text);
        }
        MessageBoxMode::Silent => {}
    }
    1
}

fn show_dialog(vm: &Vm, caption: &str, text: &str) -> bool {
    // Prefer SDL rendering to match a real dialog window.
    try_sdl_dialog(vm, caption, text)
}

#[cfg(feature = "sdl2")]
fn try_sdl_dialog(vm: &Vm, _caption: &str, text: &str) -> bool {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::mouse::{Cursor, SystemCursor};
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::BlendMode;
    use std::time::Duration;

    // Prefer a real font (config or OS default), fall back to bitmap if unavailable.
    let Some(font) = load_dialog_font(vm) else {
        return try_sdl_dialog_bitmap(text);
    };

    // Initialize SDL and window/canvas.
    let Ok(_sdl) = sdl2::init() else {
        return false;
    };
    let Ok(video) = _sdl.video() else {
        return false;
    };
    // Compute dialog layout from text metrics.
    let body_font_size = 14.0;
    let button_font_size = 13.0;
    let body_line_metrics = line_metrics_or_default(&font, body_font_size);
    let button_line_metrics = line_metrics_or_default(&font, button_font_size);
    let max_text_width = (DIALOG_MAX_WIDTH - DIALOG_PAD_X * 2) as f32;
    let body_lines = wrap_text_font(text, &font, body_font_size, max_text_width);
    let layout = FontDialogLayout::new(&body_lines, body_line_metrics, button_line_metrics);
    let window_title = "pe_vm";

    let Ok(mut window) = video
        .window(window_title, layout.width as u32, layout.height as u32)
        .position_centered()
        .build()
    else {
        return false;
    };
    apply_blank_icon(&mut window);
    let Ok(mut canvas) = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
    else {
        return false;
    };
    canvas.set_blend_mode(BlendMode::Blend);
    let Ok(mut event_pump) = _sdl.event_pump() else {
        return false;
    };

    // Swap to a hand cursor when the OK button is hovered.
    let hand_cursor = Cursor::from_system(SystemCursor::Hand).ok();
    let arrow_cursor = Cursor::from_system(SystemCursor::Arrow).ok();
    let mut cursor_is_hand = false;
    if let Some(arrow) = &arrow_cursor {
        arrow.set();
    }

    // Dialog palette.
    let bg = Color::RGB(245, 245, 245);
    let fg = Color::RGB(20, 20, 20);

    let button_rect = Rect::new(
        layout.button_x,
        layout.button_y,
        layout.button_w as u32,
        layout.button_h as u32,
    );
    let body_params = FontLineParams {
        x: layout.pad_x,
        y: layout.text_y,
        color: fg,
        size: body_font_size,
        line_metrics: body_line_metrics,
    };

    // Render loop: draw, update cursor, and handle input.
    loop {
        canvas.set_draw_color(bg);
        canvas.clear();

        draw_lines_font(&mut canvas, &font, &body_lines, &body_params);

        canvas.set_draw_color(Color::RGB(230, 230, 230));
        let _ = canvas.fill_rect(button_rect);
        canvas.set_draw_color(Color::RGB(120, 120, 120));
        let _ = canvas.draw_rect(button_rect);

        draw_centered_label(
            &mut canvas,
            &font,
            "OK",
            button_rect,
            fg,
            button_font_size,
            button_line_metrics,
        );

        canvas.present();

        if let (Some(hand), Some(arrow)) = (&hand_cursor, &arrow_cursor) {
            let mouse = event_pump.mouse_state();
            let hovered = mouse.x() >= button_rect.x()
                && mouse.x() < button_rect.x() + button_rect.width() as i32
                && mouse.y() >= button_rect.y()
                && mouse.y() < button_rect.y() + button_rect.height() as i32;
            if hovered != cursor_is_hand {
                cursor_is_hand = hovered;
                if cursor_is_hand {
                    hand.set();
                } else {
                    arrow.set();
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => return true,
                Event::MouseButtonDown { x, y, .. } => {
                    if x >= button_rect.x()
                        && x <= button_rect.x() + button_rect.width() as i32
                        && y >= button_rect.y()
                        && y <= button_rect.y() + button_rect.height() as i32
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}

#[cfg(not(feature = "sdl2"))]
fn try_sdl_dialog(_vm: &Vm, _caption: &str, _text: &str) -> bool {
    false
}

#[cfg(feature = "sdl2")]
// Layout constants for the font-rendered dialog.
const DIALOG_PAD_X: i32 = 16;
#[cfg(feature = "sdl2")]
const DIALOG_PAD_Y: i32 = 16;
#[cfg(feature = "sdl2")]
const DIALOG_GAP: i32 = 12;
#[cfg(feature = "sdl2")]
const DIALOG_BUTTON_W: i32 = 84;
#[cfg(feature = "sdl2")]
const DIALOG_BUTTON_H: i32 = 26;
#[cfg(feature = "sdl2")]
const DIALOG_MIN_WIDTH: i32 = 260;
#[cfg(feature = "sdl2")]
const DIALOG_MAX_WIDTH: i32 = 560;
#[cfg(feature = "sdl2")]
const DIALOG_MIN_HEIGHT: i32 = 140;

#[cfg(feature = "sdl2")]
fn line_metrics_or_default(font: &Font, size: f32) -> LineMetrics {
    // Use font line metrics when available; fall back to basic scaling.
    font.horizontal_line_metrics(size).unwrap_or(LineMetrics {
        ascent: size,
        descent: 0.0,
        line_gap: 0.0,
        new_line_size: size,
    })
}
#[cfg(feature = "sdl2")]
fn load_dialog_font(vm: &Vm) -> Option<Font> {
    // Use a configured font first, then try OS defaults.
    if let Some(path) = vm.config().font_path_opt() {
        if let Some(font) = load_font_from_path(path) {
            return Some(font);
        }
    }
    for path in DEFAULT_FONT_PATHS {
        if let Some(font) = load_font_from_path(path) {
            return Some(font);
        }
    }
    None
}

#[cfg(feature = "sdl2")]
fn load_font_from_path(path: &str) -> Option<Font> {
    // Read TTF/OTF font bytes and parse them with fontdue.
    let path = std::path::Path::new(path);
    if !path.exists() {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    Font::from_bytes(bytes, FontSettings::default()).ok()
}

#[cfg(feature = "sdl2")]
#[cfg(target_os = "macos")]
// macOS default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "/System/Library/Fonts/SFNS.ttf",
    "/System/Library/Fonts/SFNSText.ttf",
    "/System/Library/Fonts/Helvetica.ttc",
    "/System/Library/Fonts/Supplemental/Arial.ttf",
    "/Library/Fonts/Arial.ttf",
];

#[cfg(feature = "sdl2")]
#[cfg(target_os = "windows")]
// Windows default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "C:\\Windows\\Fonts\\segoeui.ttf",
    "C:\\Windows\\Fonts\\Tahoma.ttf",
    "C:\\Windows\\Fonts\\arial.ttf",
];

#[cfg(feature = "sdl2")]
#[cfg(target_os = "linux")]
// Linux default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/opentype/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
];

#[cfg(feature = "sdl2")]
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
// Fallback if the host OS doesn't match known lists.
const DEFAULT_FONT_PATHS: &[&str] = &[];

#[cfg(feature = "sdl2")]
// Line of text with a cached pixel width.
struct TextLine {
    text: String,
    width: f32,
}

#[cfg(feature = "sdl2")]
fn wrap_text_font(text: &str, font: &Font, size: f32, max_width: f32) -> Vec<TextLine> {
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

#[cfg(feature = "sdl2")]
fn measure_text_width(font: &Font, size: f32, text: &str) -> f32 {
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

#[cfg(feature = "sdl2")]
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

#[cfg(feature = "sdl2")]
struct FontDialogLayout {
    width: i32,
    height: i32,
    pad_x: i32,
    text_y: i32,
    button_x: i32,
    button_y: i32,
    button_w: i32,
    button_h: i32,
}

#[cfg(feature = "sdl2")]
impl FontDialogLayout {
    fn new(body_lines: &[TextLine], body_metrics: LineMetrics, _button_metrics: LineMetrics) -> Self {
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

#[cfg(feature = "sdl2")]
struct FontLineParams {
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    size: f32,
    line_metrics: LineMetrics,
}

#[cfg(feature = "sdl2")]
fn draw_lines_font(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &Font,
    lines: &[TextLine],
    params: &FontLineParams,
) {
    // Draw each line on an integer baseline to reduce jitter.
    let line_height = params.line_metrics.new_line_size;
    let ascent = params.line_metrics.ascent;
    for (idx, line) in lines.iter().enumerate() {
        let baseline_y = params.y as f32 + line_height * idx as f32 + ascent;
        let baseline_y = baseline_y.round() as i32;
        draw_text_font(
            canvas,
            font,
            &line.text,
            params.x as f32,
            baseline_y,
            params.color,
            params.size,
        );
    }
}

#[cfg(feature = "sdl2")]
fn draw_centered_label(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &Font,
    text: &str,
    rect: sdl2::rect::Rect,
    color: sdl2::pixels::Color,
    size: f32,
    line_metrics: LineMetrics,
) {
    // Center the label within the button rect.
    let text_width = measure_text_width(font, size, text);
    let line_height = line_metrics.new_line_size;
    let baseline_y = rect.y() as f32
        + (rect.height() as f32 - line_height) / 2.0
        + line_metrics.ascent;
    let baseline_y = baseline_y.round() as i32;
    let x = rect.x() as f32 + (rect.width() as f32 - text_width) / 2.0;
    draw_text_font(canvas, font, text, x, baseline_y, color, size);
}

#[cfg(feature = "sdl2")]
fn draw_text_font(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &Font,
    text: &str,
    x: f32,
    baseline_y: i32,
    color: sdl2::pixels::Color,
    size: f32,
) {
    // Rasterize glyphs directly; SDL2 is used only for the window/canvas.
    let mut x = x;
    let (r, g, b) = (color.r, color.g, color.b);
    let mut prev = None;
    for ch in text.chars() {
        if let Some(prev_ch) = prev {
            if let Some(kern) = font.horizontal_kern(prev_ch, ch, size) {
                x += kern;
            }
        }
        let (metrics, bitmap) = font.rasterize(ch, size);
        if metrics.width == 0 || metrics.height == 0 {
            x += metrics.advance_width;
            prev = Some(ch);
            continue;
        }
        let origin_x = x.round() as i32;
        let glyph_x = origin_x + metrics.xmin;
        let glyph_y = baseline_y - metrics.ymin - metrics.height as i32;
        let mut idx = 0usize;
        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let alpha = bitmap[idx];
                idx += 1;
                if alpha == 0 {
                    continue;
                }
                canvas.set_draw_color(sdl2::pixels::Color::RGBA(r, g, b, alpha));
                let rect = sdl2::rect::Rect::new(
                    glyph_x + col as i32,
                    glyph_y + row as i32,
                    1,
                    1,
                );
                let _ = canvas.fill_rect(rect);
            }
        }
        x += metrics.advance_width;
        prev = Some(ch);
    }
}

#[cfg(feature = "sdl2")]
fn apply_blank_icon(window: &mut sdl2::video::Window) {
    // Force an empty icon to avoid folder icons on some hosts.
    let Ok(mut surface) =
        sdl2::surface::Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::RGBA32)
    else {
        return;
    };
    let _ = surface.fill_rect(None, sdl2::pixels::Color::RGBA(0, 0, 0, 0));
    window.set_icon(surface);
}

#[cfg(feature = "sdl2")]
fn try_sdl_dialog_bitmap(text: &str) -> bool {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::mouse::{Cursor, SystemCursor};
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use std::time::Duration;

    let Ok(_sdl) = sdl2::init() else {
        return false;
    };
    let Ok(video) = _sdl.video() else {
        return false;
    };
    let body_lines = wrap_text_columns(text, 60);
    let layout = BitmapDialogLayout::new(&body_lines);
    let window_title = "pe_vm";

    let Ok(mut window) = video
        .window(window_title, layout.width as u32, layout.height as u32)
        .position_centered()
        .build()
    else {
        return false;
    };
    apply_blank_icon(&mut window);
    let Ok(mut canvas) = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
    else {
        return false;
    };
    let Ok(mut event_pump) = _sdl.event_pump() else {
        return false;
    };

    // Swap to a hand cursor when the OK button is hovered.
    let hand_cursor = Cursor::from_system(SystemCursor::Hand).ok();
    let arrow_cursor = Cursor::from_system(SystemCursor::Arrow).ok();
    let mut cursor_is_hand = false;
    if let Some(arrow) = &arrow_cursor {
        arrow.set();
    }

    // Dialog palette (bitmap fallback).
    let bg = Color::RGB(245, 245, 245);
    let fg = Color::RGB(20, 20, 20);

    let button_rect = Rect::new(
        layout.button_x,
        layout.button_y,
        layout.button_w as u32,
        layout.button_h as u32,
    );

    // Render loop: draw, update cursor, and handle input.
    loop {
        canvas.set_draw_color(bg);
        canvas.clear();

        draw_lines_bitmap(
            &mut canvas,
            &body_lines,
            layout.pad_x,
            layout.text_y,
            fg,
            layout.body_scale,
        );

        canvas.set_draw_color(Color::RGB(230, 230, 230));
        let _ = canvas.fill_rect(button_rect);
        canvas.set_draw_color(Color::RGB(120, 120, 120));
        let _ = canvas.draw_rect(button_rect);

        let ok_label = ["OK".to_string()];
        let ok_params = BitmapLineParams {
            x: button_rect.x(),
            y: button_rect.y(),
            width: button_rect.width() as i32,
            height: button_rect.height() as i32,
            color: fg,
            scale: layout.button_scale,
        };
        draw_lines_centered_bitmap(&mut canvas, &ok_label, &ok_params);

        canvas.present();

        if let (Some(hand), Some(arrow)) = (&hand_cursor, &arrow_cursor) {
            let mouse = event_pump.mouse_state();
            let hovered = mouse.x() >= button_rect.x()
                && mouse.x() < button_rect.x() + button_rect.width() as i32
                && mouse.y() >= button_rect.y()
                && mouse.y() < button_rect.y() + button_rect.height() as i32;
            if hovered != cursor_is_hand {
                cursor_is_hand = hovered;
                if cursor_is_hand {
                    hand.set();
                } else {
                    arrow.set();
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => return true,
                Event::MouseButtonDown { x, y, .. } => {
                    if x >= button_rect.x()
                        && x <= button_rect.x() + button_rect.width() as i32
                        && y >= button_rect.y()
                        && y <= button_rect.y() + button_rect.height() as i32
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}

#[cfg(feature = "sdl2")]
fn wrap_text_columns(text: &str, max_cols: usize) -> Vec<String> {
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

#[cfg(feature = "sdl2")]
// Layout container for bitmap fallback rendering.
struct BitmapDialogLayout {
    width: i32,
    height: i32,
    pad_x: i32,
    text_y: i32,
    button_x: i32,
    button_y: i32,
    button_w: i32,
    button_h: i32,
    body_scale: i32,
    button_scale: i32,
}

#[cfg(feature = "sdl2")]
impl BitmapDialogLayout {
    fn new(body_lines: &[String]) -> Self {
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

#[cfg(feature = "sdl2")]
fn draw_lines_bitmap(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    lines: &[String],
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    scale: i32,
) {
    // Draw bitmap text line-by-line.
    let mut y = y;
    let line_height = 8 * scale;
    for line in lines {
        draw_text_bitmap(canvas, line, x, y, color, scale);
        y += line_height;
    }
}

#[cfg(feature = "sdl2")]
struct BitmapLineParams {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: sdl2::pixels::Color,
    scale: i32,
}

#[cfg(feature = "sdl2")]
fn draw_lines_centered_bitmap(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    lines: &[String],
    params: &BitmapLineParams,
) {
    // Center bitmap text within the button.
    let line_height = 8 * params.scale;
    let total_height = line_height * lines.len() as i32;
    let mut cursor_y = params.y + (params.height - total_height) / 2;
    for line in lines {
        let line_width = line.chars().count() as i32 * 8 * params.scale;
        let cursor_x = params.x + (params.width - line_width) / 2;
        draw_text_bitmap(canvas, line, cursor_x, cursor_y, params.color, params.scale);
        cursor_y += line_height;
    }
}

#[cfg(feature = "sdl2")]
fn draw_text_bitmap(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    text: &str,
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    scale: i32,
) {
    // Render bitmap glyphs using the 8x8 font table.
    let mut x = x;
    canvas.set_draw_color(color);
    for ch in text.chars() {
        let glyph = font8x8::BASIC_FONTS
            .get(ch)
            .or_else(|| font8x8::BASIC_FONTS.get('?'));
        if let Some(rows) = glyph {
            for (row_idx, row) in rows.iter().enumerate() {
                for col in 0..8 {
                    if (row >> col) & 1 == 1 {
                        let px = x + col * scale;
                        let py = y + row_idx as i32 * scale;
                        let rect = sdl2::rect::Rect::new(px, py, scale as u32, scale as u32);
                        let _ = canvas.fill_rect(rect);
                    }
                }
            }
        }
        x += 8 * scale;
    }
}
