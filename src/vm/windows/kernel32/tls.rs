//! Kernel32 TLS stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "TlsAlloc", crate::vm::stdcall_args(0), tls_alloc);
    vm.register_import_stdcall(DLL_NAME, "TlsGetValue", crate::vm::stdcall_args(1), tls_get_value);
    vm.register_import_stdcall(DLL_NAME, "TlsSetValue", crate::vm::stdcall_args(2), tls_set_value);
    vm.register_import_stdcall(DLL_NAME, "TlsFree", crate::vm::stdcall_args(1), tls_free);
}

fn tls_alloc(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.tls_alloc()
}

fn tls_get_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index,) = vm_args!(vm, stack_ptr; u32);
    vm.tls_get(index)
}

fn tls_set_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index, value) = vm_args!(vm, stack_ptr; u32, u32);
    if vm.tls_set(index, value) {
        1
    } else {
        0
    }
}

fn tls_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index,) = vm_args!(vm, stack_ptr; u32);
    if vm.tls_free(index) {
        1
    } else {
        0
    }
}
