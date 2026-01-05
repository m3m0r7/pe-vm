//! Kernel32 synchronization stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "CreateEventA", crate::vm::stdcall_args(4), create_event_a);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "InitializeCriticalSectionAndSpinCount",
        crate::vm::stdcall_args(2),
        initialize_critical_section_and_spin_count,
    );
    vm.register_import_stdcall("KERNEL32.dll", "EnterCriticalSection", crate::vm::stdcall_args(1), enter_critical_section);
    vm.register_import_stdcall("KERNEL32.dll", "LeaveCriticalSection", crate::vm::stdcall_args(1), leave_critical_section);
    vm.register_import_stdcall("KERNEL32.dll", "DeleteCriticalSection", crate::vm::stdcall_args(1), delete_critical_section);
    vm.register_import_stdcall("KERNEL32.dll", "SetEvent", crate::vm::stdcall_args(1), set_event);
    vm.register_import_stdcall("KERNEL32.dll", "WaitForSingleObject", crate::vm::stdcall_args(2), wait_for_single_object);
    vm.register_import_stdcall("KERNEL32.dll", "Sleep", crate::vm::stdcall_args(1), sleep);
}

fn create_event_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn initialize_critical_section_and_spin_count(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn enter_critical_section(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn leave_critical_section(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn delete_critical_section(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn set_event(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn wait_for_single_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn sleep(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
