//! Kernel32 exception-related stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SetUnhandledExceptionFilter",
        crate::vm::stdcall_args(1),
        set_unhandled_exception_filter,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "UnhandledExceptionFilter",
        crate::vm::stdcall_args(1),
        unhandled_exception_filter,
    );
    vm.register_import_stdcall("KERNEL32.dll", "RaiseException", crate::vm::stdcall_args(4), raise_exception);
    vm.register_import_stdcall("KERNEL32.dll", "RtlUnwind", crate::vm::stdcall_args(4), rtl_unwind);
}

fn set_unhandled_exception_filter(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filter = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let previous = vm.unhandled_exception_filter();
    vm.set_unhandled_exception_filter(filter);
    previous
}

fn unhandled_exception_filter(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
        eprintln!(
            "[pe_vm] UnhandledExceptionFilter at eip=0x{:08X}",
            _vm.eip()
        );
    }
    0
}

fn raise_exception(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn rtl_unwind(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
