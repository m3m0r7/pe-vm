use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use super::bitmap_draw::{draw_lines_bitmap, draw_lines_centered_bitmap};
use super::bitmap_layout::{wrap_text_columns, BitmapDialogLayout, BitmapLineParams};
use super::common::apply_blank_icon;

pub(super) fn try_dialog_bitmap(text: &str) -> bool {
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
    let Ok(mut canvas) = window.into_canvas().accelerated().present_vsync().build() else {
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
