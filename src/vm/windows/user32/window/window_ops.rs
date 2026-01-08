use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::constants::DUMMY_HWND;
use super::helpers::{write_c_string, write_rect};

/// DestroyWindow - Destroys the specified window
/// Returns: TRUE if succeeded, FALSE if failed
fn destroy_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd,) = vm_args!(vm, stack_ptr; u32);
    // In our VM, we don't track windows, so just return success if hwnd is valid
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// ShowWindow - Sets the specified window's show state
/// Returns: TRUE if window was previously visible, FALSE otherwise
fn show_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Return FALSE (window was not previously visible)
    0
}

/// MoveWindow - Changes position and dimensions of window
/// Returns: TRUE if succeeded, FALSE if failed
fn move_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _x, _y, _width, _height, _repaint) = vm_args!(vm, stack_ptr; u32, i32, i32, i32, i32, u32);
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// SetWindowPos - Changes size, position, and Z order of window
/// Returns: TRUE if succeeded, FALSE if failed
fn set_window_pos(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _insert_after, _x, _y, _cx, _cy, _flags) = vm_args!(vm, stack_ptr; u32, u32, i32, i32, i32, i32, u32);
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// GetWindowTextLengthA - Gets length of window title text
/// Returns: Length of text in characters (not including null terminator), or 0
fn get_window_text_length_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track window text, return 0 (no text)
    0
}

/// SetWindowTextA - Sets window title text
/// Returns: TRUE if succeeded, FALSE if failed
fn set_window_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _text_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// GetWindow - Gets handle to related window (owner, child, sibling, etc.)
/// Returns: Window handle, or NULL if no window exists
fn get_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _cmd) = vm_args!(vm, stack_ptr; u32, u32);
    // Return the same window handle if valid, simulating a simple hierarchy
    if hwnd != 0 {
        hwnd
    } else {
        DUMMY_HWND
    }
}

/// GetParent - Gets handle to parent window
/// Returns: Parent window handle, or NULL if no parent
fn get_parent(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd,) = vm_args!(vm, stack_ptr; u32);
    // Return desktop window as parent if hwnd is valid
    if hwnd != 0 {
        DUMMY_HWND
    } else {
        0
    }
}

/// IsChild - Tests whether window is child of specified parent
/// Returns: TRUE if child, FALSE otherwise
fn is_child(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track window hierarchy, return FALSE
    0
}

/// GetWindowLongA - Gets information about window
/// Returns: Requested value, or 0 on failure
fn get_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track window attributes, return 0
    0
}

/// SetWindowLongA - Changes attribute of window
/// Returns: Previous value, or 0 on failure
fn set_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track window attributes, return 0 (previous value)
    0
}

/// SetParent - Changes parent window of specified child window
/// Returns: Previous parent window handle, or NULL on failure
fn set_parent(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _new_parent) = vm_args!(vm, stack_ptr; u32, u32);
    // Return previous parent (desktop window) if child is valid
    if hwnd != 0 {
        DUMMY_HWND
    } else {
        0
    }
}

/// GetActiveWindow - Gets handle to active window attached to calling thread's message queue
/// Returns: Active window handle, or NULL if no active window
fn get_active_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Return a valid dummy window handle
    DUMMY_HWND
}

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateWindowExA",
        crate::vm::stdcall_args(12),
        create_window_ex_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "DestroyWindow",
        crate::vm::stdcall_args(1),
        destroy_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ShowWindow",
        crate::vm::stdcall_args(2),
        show_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "MoveWindow",
        crate::vm::stdcall_args(6),
        move_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetWindowPos",
        crate::vm::stdcall_args(7),
        set_window_pos,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetWindowRect",
        crate::vm::stdcall_args(2),
        get_window_rect,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetClientRect",
        crate::vm::stdcall_args(2),
        get_client_rect,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetWindowTextLengthA",
        crate::vm::stdcall_args(1),
        get_window_text_length_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetWindowTextA",
        crate::vm::stdcall_args(3),
        get_window_text_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetWindowTextA",
        crate::vm::stdcall_args(2),
        set_window_text_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetWindow",
        crate::vm::stdcall_args(2),
        get_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetParent",
        crate::vm::stdcall_args(1),
        get_parent,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetDesktopWindow",
        crate::vm::stdcall_args(0),
        get_desktop_window,
    );
    vm.register_import_stdcall(DLL_NAME, "IsWindow", crate::vm::stdcall_args(1), is_window);
    vm.register_import_stdcall(DLL_NAME, "IsChild", crate::vm::stdcall_args(2), is_child);
    vm.register_import_stdcall(
        DLL_NAME,
        "GetClassNameA",
        crate::vm::stdcall_args(3),
        get_class_name_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetWindowLongA",
        crate::vm::stdcall_args(2),
        get_window_long_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetWindowLongA",
        crate::vm::stdcall_args(3),
        set_window_long_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetParent",
        crate::vm::stdcall_args(2),
        set_parent,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetActiveWindow",
        crate::vm::stdcall_args(0),
        get_active_window,
    );
}

pub(super) fn create_window_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

pub(super) fn get_window_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, rect_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 640, 480);
    }
    1
}

pub(super) fn get_client_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, rect_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 640, 480);
    }
    1
}

pub(super) fn get_window_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, text_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if text_ptr != 0 && max_len > 0 {
        write_c_string(vm, text_ptr, "", max_len);
    }
    0
}

pub(super) fn get_desktop_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

pub(super) fn is_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd,) = vm_args!(vm, stack_ptr; u32);
    if hwnd == 0 {
        0
    } else {
        1
    }
}

pub(super) fn get_class_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}
