use fontdue::{Font, LineMetrics};

use super::font_layout::{measure_text_width, FontLineParams, TextLine};

pub(super) fn draw_lines_font(
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

pub(super) fn draw_centered_label(
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
    let baseline_y =
        rect.y() as f32 + (rect.height() as f32 - line_height) / 2.0 + line_metrics.ascent;
    let baseline_y = baseline_y.round() as i32;
    let x = rect.x() as f32 + (rect.width() as f32 - text_width) / 2.0;
    draw_text_font(canvas, font, text, x, baseline_y, color, size);
}

pub(super) fn draw_text_font(
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
                let rect = sdl2::rect::Rect::new(glyph_x + col as i32, glyph_y + row as i32, 1, 1);
                let _ = canvas.fill_rect(rect);
            }
        }
        x += metrics.advance_width;
        prev = Some(ch);
    }
}
