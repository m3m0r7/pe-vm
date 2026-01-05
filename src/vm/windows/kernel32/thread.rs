//! Kernel32 thread-related stubs.

use crate::vm::{Value, Vm};

const CREATE_SUSPENDED: u32 = 0x00000004;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "CreateThread", crate::vm::stdcall_args(6), create_thread);
    vm.register_import_stdcall("KERNEL32.dll", "GetCurrentThreadId", crate::vm::stdcall_args(0), get_current_thread_id);
    vm.register_import_stdcall("KERNEL32.dll", "ExitThread", crate::vm::stdcall_args(1), exit_thread);
}

pub(crate) fn create_thread(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let start = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let param = vm.read_u32(stack_ptr.wrapping_add(16)).unwrap_or(0);
    let flags = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let thread_id_ptr = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0);

    if thread_id_ptr != 0 {
        let _ = vm.write_u32(thread_id_ptr, 1);
    }

    if start == 0 {
        return 0;
    }

    if flags & CREATE_SUSPENDED == 0
        && vm
            .execute_at_with_stack(start, &[Value::U32(param)])
            .is_err()
    {
        return 0;
    }

    1
}

fn get_current_thread_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn exit_thread(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
