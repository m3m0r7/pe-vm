use crate::vm::Vm;
use fontdue::Font;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::time::Duration;

use super::common::apply_blank_icon;
use super::font_draw::{draw_centered_label, draw_lines_font};
use super::font_layout::{
    line_metrics_or_default, wrap_text_font, FontDialogLayout, FontLineParams, DIALOG_MAX_WIDTH,
    DIALOG_PAD_X,
};

pub(super) fn try_dialog_with_font(_vm: &Vm, font: &Font, text: &str) -> bool {
    let Ok(_sdl) = sdl2::init() else {
        return false;
    };
    let Ok(video) = _sdl.video() else {
        return false;
    };

    // Compute dialog layout from text metrics.
    let body_font_size = 14.0;
    let button_font_size = 13.0;
    let body_line_metrics = line_metrics_or_default(font, body_font_size);
    let button_line_metrics = line_metrics_or_default(font, button_font_size);
    let max_text_width = (DIALOG_MAX_WIDTH - DIALOG_PAD_X * 2) as f32;
    let body_lines = wrap_text_font(text, font, body_font_size, max_text_width);
    let layout = FontDialogLayout::new(&body_lines, body_line_metrics);
    let window_title = "pe_vm";

    let Ok(mut window) = video
        .window(window_title, layout.width as u32, layout.height as u32)
        .position_centered()
        .build()
    else {
        return false;
    };
    apply_blank_icon(&mut window);
    let Ok(mut canvas) = window.into_canvas().accelerated().present_vsync().build() else {
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

        draw_lines_font(&mut canvas, font, &body_lines, &body_params);

        canvas.set_draw_color(Color::RGB(230, 230, 230));
        let _ = canvas.fill_rect(button_rect);
        canvas.set_draw_color(Color::RGB(120, 120, 120));
        let _ = canvas.draw_rect(button_rect);

        draw_centered_label(
            &mut canvas,
            font,
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
