//! Kernel32 error stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetLastError", crate::vm::stdcall_args(0), get_last_error);
    vm.register_import_stdcall(DLL_NAME, "SetLastError", crate::vm::stdcall_args(1), set_last_error);
}

fn get_last_error(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.last_error()
}

fn set_last_error(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (value,) = vm_args!(vm, stack_ptr; u32);
    vm.set_last_error(value);
    0
}
