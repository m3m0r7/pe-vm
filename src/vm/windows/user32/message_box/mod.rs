//! SDL-backed MessageBoxA implementation.

use crate::vm::windows::user32::DLL_NAME;
use crate::vm::{MessageBoxMode, Vm};
use crate::vm_args;

mod sdl;

pub fn register(vm: &mut Vm) {
    // Expose MessageBoxA to guest imports as a stdcall host function.
    vm.register_import_stdcall(
        DLL_NAME,
        "MessageBoxA",
        crate::vm::stdcall_args(4),
        message_box_a,
    );
    // Wide-char variant used by some DLLs.
    vm.register_import_stdcall(
        DLL_NAME,
        "MessageBoxW",
        crate::vm::stdcall_args(4),
        message_box_w,
    );
}

pub(crate) fn message_box_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Read Win32 MessageBoxA arguments from the guest stack.
    let (_hwnd, text_ptr, caption_ptr, _utype) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
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

pub(crate) fn message_box_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Read Win32 MessageBoxW arguments from the guest stack.
    let (_hwnd, text_ptr, caption_ptr, _utype) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let text = if text_ptr != 0 {
        read_w_string(vm, text_ptr)
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        read_w_string(vm, caption_ptr)
    } else {
        String::new()
    };
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
    sdl::try_dialog(vm, caption, text)
}

fn read_w_string(vm: &Vm, ptr: u32) -> String {
    let mut units = Vec::new();
    let mut cursor = ptr;
    loop {
        let value = vm.read_u16(cursor).unwrap_or(0);
        if value == 0 {
            break;
        }
        units.push(value);
        cursor = cursor.wrapping_add(2);
    }
    String::from_utf16_lossy(&units)
}
