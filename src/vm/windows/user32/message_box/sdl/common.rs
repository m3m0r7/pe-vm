use sdl2::video::Window;

pub(super) fn apply_blank_icon(window: &mut Window) {
    // Force an empty icon to avoid folder icons on some hosts.
    let Ok(mut surface) =
        sdl2::surface::Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::RGBA32)
    else {
        return;
    };
    let _ = surface.fill_rect(None, sdl2::pixels::Color::RGBA(0, 0, 0, 0));
    window.set_icon(surface);
}
