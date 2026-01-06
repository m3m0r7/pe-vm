use crate::vm::Vm;

#[cfg(feature = "sdl2")]
mod bitmap_dialog;
#[cfg(feature = "sdl2")]
mod bitmap_draw;
#[cfg(feature = "sdl2")]
mod bitmap_layout;
#[cfg(feature = "sdl2")]
mod common;
#[cfg(feature = "sdl2")]
mod font_dialog;
#[cfg(feature = "sdl2")]
mod font_draw;
#[cfg(feature = "sdl2")]
mod font_layout;
#[cfg(feature = "sdl2")]
mod font_load;

#[cfg(feature = "sdl2")]
pub(super) fn try_dialog(vm: &Vm, _caption: &str, text: &str) -> bool {
    if let Some(font) = font_load::load_dialog_font(vm) {
        return font_dialog::try_dialog_with_font(vm, &font, text);
    }
    bitmap_dialog::try_dialog_bitmap(text)
}

#[cfg(not(feature = "sdl2"))]
pub(super) fn try_dialog(_vm: &Vm, _caption: &str, _text: &str) -> bool {
    false
}
