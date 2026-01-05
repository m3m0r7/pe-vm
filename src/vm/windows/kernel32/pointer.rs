//! Kernel32 pointer encoding stubs.

use crate::vm::Vm;

const POINTER_COOKIE: u32 = 0xA5A5_A5A5;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "EncodePointer", crate::vm::stdcall_args(1), encode_pointer);
    vm.register_import_stdcall("KERNEL32.dll", "DecodePointer", crate::vm::stdcall_args(1), decode_pointer);
}

fn encode_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let value = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if value == 0 {
        POINTER_COOKIE
    } else {
        value
    }
}

fn decode_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let value = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if value == POINTER_COOKIE {
        0
    } else {
        value
    }
}
