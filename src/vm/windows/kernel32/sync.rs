//! Kernel32 synchronization stubs.

use crate::register_func_stub;
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;

register_func_stub!(DLL_NAME, create_event_a, 1);
register_func_stub!(DLL_NAME, initialize_critical_section_and_spin_count, 1);
register_func_stub!(DLL_NAME, enter_critical_section, 0);
register_func_stub!(DLL_NAME, leave_critical_section, 0);
register_func_stub!(DLL_NAME, delete_critical_section, 0);
register_func_stub!(DLL_NAME, set_event, 1);

// Additional stubs
register_func_stub!(DLL_NAME, create_event_w, 1);
register_func_stub!(DLL_NAME, create_event_ex_a, 1);
register_func_stub!(DLL_NAME, create_mutex_a, 1);
register_func_stub!(DLL_NAME, create_mutex_w, 1);
register_func_stub!(DLL_NAME, create_mutex_ex_a, 1);
register_func_stub!(DLL_NAME, create_mutex_ex_w, 1);
register_func_stub!(DLL_NAME, open_mutex_a, 1);
register_func_stub!(DLL_NAME, open_mutex_w, 1);
register_func_stub!(DLL_NAME, release_mutex, 1);
register_func_stub!(DLL_NAME, create_semaphore_a, 1);
register_func_stub!(DLL_NAME, create_semaphore_w, 1);
register_func_stub!(DLL_NAME, create_semaphore_ex_a, 1);
register_func_stub!(DLL_NAME, create_semaphore_ex_w, 1);
register_func_stub!(DLL_NAME, open_semaphore_a, 1);
register_func_stub!(DLL_NAME, open_semaphore_w, 1);
register_func_stub!(DLL_NAME, release_semaphore, 1);
register_func_stub!(DLL_NAME, open_event_a, 1);
register_func_stub!(DLL_NAME, open_event_w, 1);
register_func_stub!(DLL_NAME, reset_event, 1);
register_func_stub!(DLL_NAME, pulse_event, 1);
register_func_stub!(DLL_NAME, initialize_critical_section, 0);
register_func_stub!(DLL_NAME, initialize_critical_section_ex, 1);
register_func_stub!(DLL_NAME, try_enter_critical_section, 1);
register_func_stub!(DLL_NAME, set_critical_section_spin_count, 0);
register_func_stub!(DLL_NAME, wait_for_multiple_objects, 0);
register_func_stub!(DLL_NAME, wait_for_multiple_objects_ex, 0);
register_func_stub!(DLL_NAME, wait_for_single_object_ex, 0);
register_func_stub!(DLL_NAME, signal_object_and_wait, 0);
register_func_stub!(DLL_NAME, sleep_ex, 0);

// SRW locks and condition variables
register_func_stub!(DLL_NAME, initialize_srw_lock, 0);
register_func_stub!(DLL_NAME, acquire_srw_lock_exclusive, 0);
register_func_stub!(DLL_NAME, acquire_srw_lock_shared, 0);
register_func_stub!(DLL_NAME, release_srw_lock_exclusive, 0);
register_func_stub!(DLL_NAME, release_srw_lock_shared, 0);
register_func_stub!(DLL_NAME, try_acquire_srw_lock_exclusive, 1);
register_func_stub!(DLL_NAME, try_acquire_srw_lock_shared, 1);
register_func_stub!(DLL_NAME, initialize_condition_variable, 0);
register_func_stub!(DLL_NAME, wake_condition_variable, 0);
register_func_stub!(DLL_NAME, wake_all_condition_variable, 0);
register_func_stub!(DLL_NAME, sleep_condition_variable_cs, 1);
register_func_stub!(DLL_NAME, sleep_condition_variable_srw, 1);

// Init once
register_func_stub!(DLL_NAME, init_once_begin_initialize, 1);
register_func_stub!(DLL_NAME, init_once_complete, 1);
register_func_stub!(DLL_NAME, init_once_execute_once, 1);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "CreateEventA", crate::vm::stdcall_args(4), create_event_a);
    vm.register_import_stdcall(DLL_NAME, "InitializeCriticalSectionAndSpinCount", crate::vm::stdcall_args(2), initialize_critical_section_and_spin_count);
    vm.register_import_stdcall(DLL_NAME, "EnterCriticalSection", crate::vm::stdcall_args(1), enter_critical_section);
    vm.register_import_stdcall(DLL_NAME, "LeaveCriticalSection", crate::vm::stdcall_args(1), leave_critical_section);
    vm.register_import_stdcall(DLL_NAME, "DeleteCriticalSection", crate::vm::stdcall_args(1), delete_critical_section);
    vm.register_import_stdcall(DLL_NAME, "SetEvent", crate::vm::stdcall_args(1), set_event);
    vm.register_import_stdcall(DLL_NAME, "WaitForSingleObject", crate::vm::stdcall_args(2), wait_for_single_object);
    vm.register_import_stdcall(DLL_NAME, "Sleep", crate::vm::stdcall_args(1), sleep);

    // Additional
    vm.register_import_stdcall(DLL_NAME, "CreateEventW", crate::vm::stdcall_args(4), create_event_w);
    vm.register_import_stdcall(DLL_NAME, "CreateEventExA", crate::vm::stdcall_args(4), create_event_ex_a);
    vm.register_import_stdcall(DLL_NAME, "CreateMutexA", crate::vm::stdcall_args(3), create_mutex_a);
    vm.register_import_stdcall(DLL_NAME, "CreateMutexW", crate::vm::stdcall_args(3), create_mutex_w);
    vm.register_import_stdcall(DLL_NAME, "CreateMutexExA", crate::vm::stdcall_args(4), create_mutex_ex_a);
    vm.register_import_stdcall(DLL_NAME, "CreateMutexExW", crate::vm::stdcall_args(4), create_mutex_ex_w);
    vm.register_import_stdcall(DLL_NAME, "OpenMutexA", crate::vm::stdcall_args(3), open_mutex_a);
    vm.register_import_stdcall(DLL_NAME, "OpenMutexW", crate::vm::stdcall_args(3), open_mutex_w);
    vm.register_import_stdcall(DLL_NAME, "ReleaseMutex", crate::vm::stdcall_args(1), release_mutex);
    vm.register_import_stdcall(DLL_NAME, "CreateSemaphoreA", crate::vm::stdcall_args(4), create_semaphore_a);
    vm.register_import_stdcall(DLL_NAME, "CreateSemaphoreW", crate::vm::stdcall_args(4), create_semaphore_w);
    vm.register_import_stdcall(DLL_NAME, "CreateSemaphoreExA", crate::vm::stdcall_args(6), create_semaphore_ex_a);
    vm.register_import_stdcall(DLL_NAME, "CreateSemaphoreExW", crate::vm::stdcall_args(6), create_semaphore_ex_w);
    vm.register_import_stdcall(DLL_NAME, "OpenSemaphoreA", crate::vm::stdcall_args(3), open_semaphore_a);
    vm.register_import_stdcall(DLL_NAME, "OpenSemaphoreW", crate::vm::stdcall_args(3), open_semaphore_w);
    vm.register_import_stdcall(DLL_NAME, "ReleaseSemaphore", crate::vm::stdcall_args(3), release_semaphore);
    vm.register_import_stdcall(DLL_NAME, "OpenEventA", crate::vm::stdcall_args(3), open_event_a);
    vm.register_import_stdcall(DLL_NAME, "OpenEventW", crate::vm::stdcall_args(3), open_event_w);
    vm.register_import_stdcall(DLL_NAME, "ResetEvent", crate::vm::stdcall_args(1), reset_event);
    vm.register_import_stdcall(DLL_NAME, "PulseEvent", crate::vm::stdcall_args(1), pulse_event);
    vm.register_import_stdcall(DLL_NAME, "InitializeCriticalSection", crate::vm::stdcall_args(1), initialize_critical_section);
    vm.register_import_stdcall(DLL_NAME, "InitializeCriticalSectionEx", crate::vm::stdcall_args(3), initialize_critical_section_ex);
    vm.register_import_stdcall(DLL_NAME, "TryEnterCriticalSection", crate::vm::stdcall_args(1), try_enter_critical_section);
    vm.register_import_stdcall(DLL_NAME, "SetCriticalSectionSpinCount", crate::vm::stdcall_args(2), set_critical_section_spin_count);
    vm.register_import_stdcall(DLL_NAME, "WaitForMultipleObjects", crate::vm::stdcall_args(4), wait_for_multiple_objects);
    vm.register_import_stdcall(DLL_NAME, "WaitForMultipleObjectsEx", crate::vm::stdcall_args(5), wait_for_multiple_objects_ex);
    vm.register_import_stdcall(DLL_NAME, "WaitForSingleObjectEx", crate::vm::stdcall_args(3), wait_for_single_object_ex);
    vm.register_import_stdcall(DLL_NAME, "SignalObjectAndWait", crate::vm::stdcall_args(4), signal_object_and_wait);
    vm.register_import_stdcall(DLL_NAME, "SleepEx", crate::vm::stdcall_args(2), sleep_ex);

    // SRW locks and condition variables
    vm.register_import_stdcall(DLL_NAME, "InitializeSRWLock", crate::vm::stdcall_args(1), initialize_srw_lock);
    vm.register_import_stdcall(DLL_NAME, "AcquireSRWLockExclusive", crate::vm::stdcall_args(1), acquire_srw_lock_exclusive);
    vm.register_import_stdcall(DLL_NAME, "AcquireSRWLockShared", crate::vm::stdcall_args(1), acquire_srw_lock_shared);
    vm.register_import_stdcall(DLL_NAME, "ReleaseSRWLockExclusive", crate::vm::stdcall_args(1), release_srw_lock_exclusive);
    vm.register_import_stdcall(DLL_NAME, "ReleaseSRWLockShared", crate::vm::stdcall_args(1), release_srw_lock_shared);
    vm.register_import_stdcall(DLL_NAME, "TryAcquireSRWLockExclusive", crate::vm::stdcall_args(1), try_acquire_srw_lock_exclusive);
    vm.register_import_stdcall(DLL_NAME, "TryAcquireSRWLockShared", crate::vm::stdcall_args(1), try_acquire_srw_lock_shared);
    vm.register_import_stdcall(DLL_NAME, "InitializeConditionVariable", crate::vm::stdcall_args(1), initialize_condition_variable);
    vm.register_import_stdcall(DLL_NAME, "WakeConditionVariable", crate::vm::stdcall_args(1), wake_condition_variable);
    vm.register_import_stdcall(DLL_NAME, "WakeAllConditionVariable", crate::vm::stdcall_args(1), wake_all_condition_variable);
    vm.register_import_stdcall(DLL_NAME, "SleepConditionVariableCS", crate::vm::stdcall_args(3), sleep_condition_variable_cs);
    vm.register_import_stdcall(DLL_NAME, "SleepConditionVariableSRW", crate::vm::stdcall_args(4), sleep_condition_variable_srw);

    // Init once
    vm.register_import_stdcall(DLL_NAME, "InitOnceBeginInitialize", crate::vm::stdcall_args(4), init_once_begin_initialize);
    vm.register_import_stdcall(DLL_NAME, "InitOnceComplete", crate::vm::stdcall_args(3), init_once_complete);
    vm.register_import_stdcall(DLL_NAME, "InitOnceExecuteOnce", crate::vm::stdcall_args(4), init_once_execute_once);
}

// These have actual implementation (run_pending_threads)
fn wait_for_single_object(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let _ = vm.run_pending_threads();
    0
}

fn sleep(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let _ = vm.run_pending_threads();
    0
}
