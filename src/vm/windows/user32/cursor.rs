//! User32 cursor-related stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("USER32.dll", "LoadCursorA", crate::vm::stdcall_args(2), load_cursor_a);
    vm.register_import_stdcall("USER32.dll", "LoadCursorW", crate::vm::stdcall_args(2), load_cursor_w);
}

fn load_cursor_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn load_cursor_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
