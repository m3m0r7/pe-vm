//! Kernel32 pointer encoding stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

const POINTER_COOKIE: u32 = 0xA5A5_A5A5;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "EncodePointer", crate::vm::stdcall_args(1), encode_pointer);
    vm.register_import_stdcall(DLL_NAME, "DecodePointer", crate::vm::stdcall_args(1), decode_pointer);
}

fn encode_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [value] = vm_args!(vm, stack_ptr; u32);
    if value == 0 {
        POINTER_COOKIE
    } else {
        value
    }
}

fn decode_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [value] = vm_args!(vm, stack_ptr; u32);
    if value == POINTER_COOKIE {
        0
    } else {
        value
    }
}
