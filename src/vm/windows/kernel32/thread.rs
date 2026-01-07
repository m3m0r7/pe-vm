//! Kernel32 thread-related stubs.

use crate::define_stub_fn;
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

define_stub_fn!(DLL_NAME, exit_thread, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateThread",
        crate::vm::stdcall_args(6),
        create_thread,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCurrentThreadId",
        crate::vm::stdcall_args(0),
        get_current_thread_id,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ExitThread",
        crate::vm::stdcall_args(1),
        exit_thread,
    );
}

pub(crate) fn create_thread(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_attrs, _stack_size, start, param, _flags, thread_id_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);

    if start == 0 {
        return 0;
    }

    let handle = vm.queue_thread(start, param);
    if thread_id_ptr != 0 {
        let _ = vm.write_u32(thread_id_ptr, handle);
    }
    handle
}

fn get_current_thread_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
