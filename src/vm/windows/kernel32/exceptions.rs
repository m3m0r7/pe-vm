//! Kernel32 exception-related stubs.

use crate::define_stub_fn;
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

define_stub_fn!(DLL_NAME, raise_exception, 0);
define_stub_fn!(DLL_NAME, rtl_unwind, 0);
define_stub_fn!(DLL_NAME, add_vectored_exception_handler, 1);
define_stub_fn!(DLL_NAME, remove_vectored_exception_handler, 1);
define_stub_fn!(DLL_NAME, add_vectored_continue_handler, 1);
define_stub_fn!(DLL_NAME, remove_vectored_continue_handler, 1);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "SetUnhandledExceptionFilter",
        crate::vm::stdcall_args(1),
        set_unhandled_exception_filter,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "UnhandledExceptionFilter",
        crate::vm::stdcall_args(1),
        unhandled_exception_filter,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RaiseException",
        crate::vm::stdcall_args(4),
        raise_exception,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RtlUnwind",
        crate::vm::stdcall_args(4),
        rtl_unwind,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "AddVectoredExceptionHandler",
        crate::vm::stdcall_args(2),
        add_vectored_exception_handler,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RemoveVectoredExceptionHandler",
        crate::vm::stdcall_args(1),
        remove_vectored_exception_handler,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "AddVectoredContinueHandler",
        crate::vm::stdcall_args(2),
        add_vectored_continue_handler,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RemoveVectoredContinueHandler",
        crate::vm::stdcall_args(1),
        remove_vectored_continue_handler,
    );
}

fn set_unhandled_exception_filter(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (filter,) = vm_args!(vm, stack_ptr; u32);
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
