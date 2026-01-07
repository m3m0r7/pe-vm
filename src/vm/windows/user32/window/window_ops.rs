use crate::vm::Vm;
use crate::vm_args;

use super::constants::DUMMY_HWND;
use super::helpers::{write_c_string, write_rect};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "USER32.dll",
        "CreateWindowExA",
        crate::vm::stdcall_args(12),
        create_window_ex_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "DestroyWindow",
        crate::vm::stdcall_args(1),
        destroy_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "ShowWindow",
        crate::vm::stdcall_args(2),
        show_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "MoveWindow",
        crate::vm::stdcall_args(6),
        move_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetWindowPos",
        crate::vm::stdcall_args(7),
        set_window_pos,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetWindowRect",
        crate::vm::stdcall_args(2),
        get_window_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetClientRect",
        crate::vm::stdcall_args(2),
        get_client_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetWindowTextLengthA",
        crate::vm::stdcall_args(1),
        get_window_text_length_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetWindowTextA",
        crate::vm::stdcall_args(3),
        get_window_text_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetWindowTextA",
        crate::vm::stdcall_args(2),
        set_window_text_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetWindow",
        crate::vm::stdcall_args(2),
        get_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetParent",
        crate::vm::stdcall_args(1),
        get_parent,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetDesktopWindow",
        crate::vm::stdcall_args(0),
        get_desktop_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "IsWindow",
        crate::vm::stdcall_args(1),
        is_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "IsChild",
        crate::vm::stdcall_args(2),
        is_child,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetClassNameA",
        crate::vm::stdcall_args(3),
        get_class_name_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetWindowLongA",
        crate::vm::stdcall_args(2),
        get_window_long_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetWindowLongA",
        crate::vm::stdcall_args(3),
        set_window_long_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetParent",
        crate::vm::stdcall_args(2),
        set_parent,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetActiveWindow",
        crate::vm::stdcall_args(0),
        get_active_window,
    );
}

pub(super) fn create_window_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

pub(super) fn destroy_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn show_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn move_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn set_window_pos(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
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

pub(super) fn get_window_text_length_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_window_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, text_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if text_ptr != 0 && max_len > 0 {
        write_c_string(vm, text_ptr, "", max_len);
    }
    0
}

pub(super) fn set_window_text_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn get_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_parent(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_desktop_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

pub(super) fn is_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [hwnd] = vm_args!(vm, stack_ptr; u32);
    if hwnd == 0 { 0 } else { 1 }
}

pub(super) fn is_child(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_class_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

pub(super) fn get_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn set_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn set_parent(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_active_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
