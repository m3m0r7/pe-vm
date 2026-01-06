use font8x8::UnicodeFonts;

use super::bitmap_layout::BitmapLineParams;

pub(super) fn draw_lines_bitmap(
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

pub(super) fn draw_lines_centered_bitmap(
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
