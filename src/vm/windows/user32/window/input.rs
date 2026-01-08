use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::helpers::write_point;

/// SetCursor - Sets the cursor shape (returns previous cursor, 0 if none)
fn set_cursor(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

/// SetCapture - Sets mouse capture to window (returns previous capture window)
fn set_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

/// ReleaseCapture - Releases mouse capture
fn release_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

/// GetKeyState - Gets key state (0 = not pressed)
fn get_key_state(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

/// GetFocus - Gets window with keyboard focus (0 = no focus)
fn get_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

/// SetFocus - Sets keyboard focus to window (returns previous focus window)
fn set_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCursorPos",
        crate::vm::stdcall_args(1),
        get_cursor_pos,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetCursor",
        crate::vm::stdcall_args(1),
        set_cursor,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetCapture",
        crate::vm::stdcall_args(1),
        set_capture,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ReleaseCapture",
        crate::vm::stdcall_args(0),
        release_capture,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetKeyState",
        crate::vm::stdcall_args(1),
        get_key_state,
    );
    vm.register_import_stdcall(DLL_NAME, "GetFocus", crate::vm::stdcall_args(0), get_focus);
    vm.register_import_stdcall(DLL_NAME, "SetFocus", crate::vm::stdcall_args(1), set_focus);
}

pub(super) fn get_cursor_pos(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (point_ptr,) = vm_args!(vm, stack_ptr; u32);
    if point_ptr != 0 {
        write_point(vm, point_ptr, 0, 0);
    }
    1
}
