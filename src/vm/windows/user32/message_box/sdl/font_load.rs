use crate::vm::Vm;
use fontdue::{Font, FontSettings};

pub(super) fn load_dialog_font(vm: &Vm) -> Option<Font> {
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

fn load_font_from_path(path: &str) -> Option<Font> {
    // Read TTF/OTF font bytes and parse them with fontdue.
    let path = std::path::Path::new(path);
    if !path.exists() {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    Font::from_bytes(bytes, FontSettings::default()).ok()
}

#[cfg(target_os = "macos")]
// macOS default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "/System/Library/Fonts/SFNS.ttf",
    "/System/Library/Fonts/SFNSText.ttf",
    "/System/Library/Fonts/Helvetica.ttc",
    "/System/Library/Fonts/Supplemental/Arial.ttf",
    "/Library/Fonts/Arial.ttf",
];

#[cfg(target_os = "windows")]
// Windows default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "C:\\Windows\\Fonts\\segoeui.ttf",
    "C:\\Windows\\Fonts\\Tahoma.ttf",
    "C:\\Windows\\Fonts\\arial.ttf",
];

#[cfg(target_os = "linux")]
// Linux default font search list.
const DEFAULT_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/opentype/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
];

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
// Fallback if the host OS doesn't match known lists.
const DEFAULT_FONT_PATHS: &[&str] = &[];
