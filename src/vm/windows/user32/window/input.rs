use crate::define_stub_fn;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::helpers::write_point;

define_stub_fn!(DLL_NAME, set_cursor, 0);
define_stub_fn!(DLL_NAME, set_capture, 0);
define_stub_fn!(DLL_NAME, release_capture, 1);
define_stub_fn!(DLL_NAME, get_key_state, 0);
define_stub_fn!(DLL_NAME, get_focus, 0);
define_stub_fn!(DLL_NAME, set_focus, 0);

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetCursorPos", crate::vm::stdcall_args(1), get_cursor_pos);
    vm.register_import_stdcall(DLL_NAME, "SetCursor", crate::vm::stdcall_args(1), set_cursor);
    vm.register_import_stdcall(DLL_NAME, "SetCapture", crate::vm::stdcall_args(1), set_capture);
    vm.register_import_stdcall(DLL_NAME, "ReleaseCapture", crate::vm::stdcall_args(0), release_capture);
    vm.register_import_stdcall(DLL_NAME, "GetKeyState", crate::vm::stdcall_args(1), get_key_state);
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
