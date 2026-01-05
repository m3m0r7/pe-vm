//! Kernel32 error stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetLastError", crate::vm::stdcall_args(0), get_last_error);
    vm.register_import_stdcall("KERNEL32.dll", "SetLastError", crate::vm::stdcall_args(1), set_last_error);
}

fn get_last_error(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.last_error()
}

fn set_last_error(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let value = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    vm.set_last_error(value);
    0
}
