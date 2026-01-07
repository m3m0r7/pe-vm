use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "CreateThreadpoolTimer",
        crate::vm::stdcall_args(3),
        create_threadpool_timer,
    );
    vm.register_import_any_stdcall(
        "SetThreadpoolTimer",
        crate::vm::stdcall_args(4),
        set_threadpool_timer,
    );
    vm.register_import_any_stdcall(
        "WaitForThreadpoolTimerCallbacks",
        crate::vm::stdcall_args(2),
        wait_for_threadpool_timer_callbacks,
    );
    vm.register_import_any_stdcall(
        "CloseThreadpoolTimer",
        crate::vm::stdcall_args(1),
        close_threadpool_timer,
    );
    vm.register_import_any_stdcall(
        "CreateThreadpoolWait",
        crate::vm::stdcall_args(3),
        create_threadpool_wait,
    );
    vm.register_import_any_stdcall(
        "SetThreadpoolWait",
        crate::vm::stdcall_args(3),
        set_threadpool_wait,
    );
    vm.register_import_any_stdcall(
        "CloseThreadpoolWait",
        crate::vm::stdcall_args(1),
        close_threadpool_wait,
    );
    vm.register_import_any_stdcall(
        "FreeLibraryWhenCallbackReturns",
        crate::vm::stdcall_args(2),
        free_library_when_callback_returns,
    );
}

fn create_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn wait_for_threadpool_timer_callbacks(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn close_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn create_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn close_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn free_library_when_callback_returns(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
