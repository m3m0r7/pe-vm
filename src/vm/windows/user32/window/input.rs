use crate::vm::Vm;
use crate::vm_args;

use super::helpers::write_point;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "USER32.dll",
        "GetCursorPos",
        crate::vm::stdcall_args(1),
        get_cursor_pos,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetCursor",
        crate::vm::stdcall_args(1),
        set_cursor,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetCapture",
        crate::vm::stdcall_args(1),
        set_capture,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "ReleaseCapture",
        crate::vm::stdcall_args(0),
        release_capture,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetKeyState",
        crate::vm::stdcall_args(1),
        get_key_state,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetFocus",
        crate::vm::stdcall_args(0),
        get_focus,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetFocus",
        crate::vm::stdcall_args(1),
        set_focus,
    );
}

pub(super) fn get_cursor_pos(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [point_ptr] = vm_args!(vm, stack_ptr; u32);
    if point_ptr != 0 {
        write_point(vm, point_ptr, 0, 0);
    }
    1
}

pub(super) fn set_cursor(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn set_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn release_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn get_key_state(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn set_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
