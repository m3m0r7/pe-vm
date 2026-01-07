use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "InitializeCriticalSectionEx",
        crate::vm::stdcall_args(3),
        initialize_critical_section_ex,
    );
    vm.register_import_any_stdcall(
        "CreateEventExW",
        crate::vm::stdcall_args(4),
        create_event_ex_w,
    );
    vm.register_import_any_stdcall(
        "CreateSemaphoreExW",
        crate::vm::stdcall_args(6),
        create_semaphore_ex_w,
    );
    vm.register_import_any_stdcall(
        "SetThreadStackGuarantee",
        crate::vm::stdcall_args(1),
        set_thread_stack_guarantee,
    );
}

fn initialize_critical_section_ex(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_event_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_semaphore_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_thread_stack_guarantee(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
