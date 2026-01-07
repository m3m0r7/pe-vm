//! Kernel32 thread-related stubs.

use crate::register_func_stub;
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;

register_func_stub!(DLL_NAME, exit_thread, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "CreateThread", crate::vm::stdcall_args(6), create_thread);
    vm.register_import_stdcall(DLL_NAME, "GetCurrentThreadId", crate::vm::stdcall_args(0), get_current_thread_id);
    vm.register_import_stdcall(DLL_NAME, "ExitThread", crate::vm::stdcall_args(1), exit_thread);
}

pub(crate) fn create_thread(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let start = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let param = vm.read_u32(stack_ptr.wrapping_add(16)).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let thread_id_ptr = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0);

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
