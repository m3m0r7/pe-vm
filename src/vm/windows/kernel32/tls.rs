//! Kernel32 TLS stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "TlsAlloc", crate::vm::stdcall_args(0), tls_alloc);
    vm.register_import_stdcall("KERNEL32.dll", "TlsGetValue", crate::vm::stdcall_args(1), tls_get_value);
    vm.register_import_stdcall("KERNEL32.dll", "TlsSetValue", crate::vm::stdcall_args(2), tls_set_value);
    vm.register_import_stdcall("KERNEL32.dll", "TlsFree", crate::vm::stdcall_args(1), tls_free);
}

fn tls_alloc(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.tls_alloc()
}

fn tls_get_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    vm.tls_get(index)
}

fn tls_set_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if vm.tls_set(index, value) {
        1
    } else {
        0
    }
}

fn tls_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if vm.tls_free(index) {
        1
    } else {
        0
    }
}
