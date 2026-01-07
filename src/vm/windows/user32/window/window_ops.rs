use crate::define_stub_fn;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::constants::DUMMY_HWND;
use super::helpers::{write_c_string, write_rect};

define_stub_fn!(DLL_NAME, destroy_window, 1);
define_stub_fn!(DLL_NAME, show_window, 1);
define_stub_fn!(DLL_NAME, move_window, 1);
define_stub_fn!(DLL_NAME, set_window_pos, 1);
define_stub_fn!(DLL_NAME, get_window_text_length_a, 0);
define_stub_fn!(DLL_NAME, set_window_text_a, 1);
define_stub_fn!(DLL_NAME, get_window, 0);
define_stub_fn!(DLL_NAME, get_parent, 0);
define_stub_fn!(DLL_NAME, is_child, 0);
define_stub_fn!(DLL_NAME, get_window_long_a, 0);
define_stub_fn!(DLL_NAME, set_window_long_a, 0);
define_stub_fn!(DLL_NAME, set_parent, 0);
define_stub_fn!(DLL_NAME, get_active_window, 0);

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "CreateWindowExA", crate::vm::stdcall_args(12), create_window_ex_a);
    vm.register_import_stdcall(DLL_NAME, "DestroyWindow", crate::vm::stdcall_args(1), destroy_window);
    vm.register_import_stdcall(DLL_NAME, "ShowWindow", crate::vm::stdcall_args(2), show_window);
    vm.register_import_stdcall(DLL_NAME, "MoveWindow", crate::vm::stdcall_args(6), move_window);
    vm.register_import_stdcall(DLL_NAME, "SetWindowPos", crate::vm::stdcall_args(7), set_window_pos);
    vm.register_import_stdcall(DLL_NAME, "GetWindowRect", crate::vm::stdcall_args(2), get_window_rect);
    vm.register_import_stdcall(DLL_NAME, "GetClientRect", crate::vm::stdcall_args(2), get_client_rect);
    vm.register_import_stdcall(DLL_NAME, "GetWindowTextLengthA", crate::vm::stdcall_args(1), get_window_text_length_a);
    vm.register_import_stdcall(DLL_NAME, "GetWindowTextA", crate::vm::stdcall_args(3), get_window_text_a);
    vm.register_import_stdcall(DLL_NAME, "SetWindowTextA", crate::vm::stdcall_args(2), set_window_text_a);
    vm.register_import_stdcall(DLL_NAME, "GetWindow", crate::vm::stdcall_args(2), get_window);
    vm.register_import_stdcall(DLL_NAME, "GetParent", crate::vm::stdcall_args(1), get_parent);
    vm.register_import_stdcall(DLL_NAME, "GetDesktopWindow", crate::vm::stdcall_args(0), get_desktop_window);
    vm.register_import_stdcall(DLL_NAME, "IsWindow", crate::vm::stdcall_args(1), is_window);
    vm.register_import_stdcall(DLL_NAME, "IsChild", crate::vm::stdcall_args(2), is_child);
    vm.register_import_stdcall(DLL_NAME, "GetClassNameA", crate::vm::stdcall_args(3), get_class_name_a);
    vm.register_import_stdcall(DLL_NAME, "GetWindowLongA", crate::vm::stdcall_args(2), get_window_long_a);
    vm.register_import_stdcall(DLL_NAME, "SetWindowLongA", crate::vm::stdcall_args(3), set_window_long_a);
    vm.register_import_stdcall(DLL_NAME, "SetParent", crate::vm::stdcall_args(2), set_parent);
    vm.register_import_stdcall(DLL_NAME, "GetActiveWindow", crate::vm::stdcall_args(0), get_active_window);
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
    if hwnd == 0 { 0 } else { 1 }
}

pub(super) fn get_class_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}
