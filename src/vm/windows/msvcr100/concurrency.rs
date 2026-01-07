//! Concurrency Runtime stubs for MSVCR100.dll.

use crate::vm::windows::check_stub;
use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";

macro_rules! stub {
    ($name:ident) => {
        fn $name(vm: &mut Vm, _sp: u32) -> u32 {
            check_stub(vm, DLL, stringify!($name));
            0
        }
    };
}

// Concurrency Runtime classes and methods
stub!(concurrency_alloc);
stub!(concurrency_free);
stub!(context_block);
stub!(context_current_context);
stub!(context_id);
stub!(context_is_current_task_collection_canceling);
stub!(context_oversubscribe);
stub!(context_schedule_group_id);
stub!(context_virtual_processor_id);
stub!(context_yield);
stub!(current_scheduler_create);
stub!(current_scheduler_detach);
stub!(current_scheduler_get);
stub!(current_scheduler_get_number_of_virtual_processors);
stub!(current_scheduler_get_policy);
stub!(current_scheduler_id);
stub!(current_scheduler_create_schedule_group);
stub!(current_scheduler_register_shutdown_event);
stub!(current_scheduler_schedule_task);
stub!(scheduler_create);
stub!(scheduler_set_default_scheduler_policy);
stub!(scheduler_reset_default_scheduler_policy);
stub!(scheduler_policy_ctor);
stub!(scheduler_policy_dtor);
stub!(scheduler_policy_get_policy_value);
stub!(scheduler_policy_set_policy_value);
stub!(scheduler_policy_set_concurrency_limits);
stub!(critical_section_ctor);
stub!(critical_section_dtor);
stub!(critical_section_lock);
stub!(critical_section_try_lock);
stub!(critical_section_unlock);
stub!(critical_section_native_handle);
stub!(reader_writer_lock_ctor);
stub!(reader_writer_lock_dtor);
stub!(reader_writer_lock_lock);
stub!(reader_writer_lock_lock_read);
stub!(reader_writer_lock_try_lock);
stub!(reader_writer_lock_try_lock_read);
stub!(reader_writer_lock_unlock);
stub!(event_ctor);
stub!(event_dtor);
stub!(event_set);
stub!(event_reset);
stub!(event_wait);
stub!(event_wait_for_multiple);
stub!(spin_lock_ctor);
stub!(spin_lock_dtor);
stub!(spin_lock_lock);
stub!(spin_lock_try_lock);
stub!(spin_lock_unlock);
stub!(spin_wait_ctor);
stub!(spin_wait_spin_once);
stub!(spin_wait_reset);
stub!(task_collection_ctor);
stub!(task_collection_dtor);
stub!(task_collection_cancel);
stub!(task_collection_is_canceling);
stub!(task_collection_run_and_wait);
stub!(task_collection_schedule);
stub!(structured_task_collection_ctor);
stub!(structured_task_collection_dtor);
stub!(structured_task_collection_cancel);
stub!(structured_task_collection_is_canceling);
stub!(structured_task_collection_run_and_wait);
stub!(structured_task_collection_schedule);

pub fn register(vm: &mut Vm) {
    // Concurrency::Alloc / Free
    vm.register_import(DLL, "?Alloc@Concurrency@@YAPAXI@Z", concurrency_alloc);
    vm.register_import(DLL, "?Free@Concurrency@@YAXPAX@Z", concurrency_free);

    // Context methods
    vm.register_import(DLL, "?Block@Context@Concurrency@@SAXXZ", context_block);
    vm.register_import(DLL, "?CurrentContext@Context@Concurrency@@SAPAV12@XZ", context_current_context);
    vm.register_import(DLL, "?Id@Context@Concurrency@@SAIXZ", context_id);
    vm.register_import(DLL, "?IsCurrentTaskCollectionCanceling@Context@Concurrency@@SA_NXZ", context_is_current_task_collection_canceling);
    vm.register_import(DLL, "?Oversubscribe@Context@Concurrency@@SAX_N@Z", context_oversubscribe);
    vm.register_import(DLL, "?ScheduleGroupId@Context@Concurrency@@SAIXZ", context_schedule_group_id);
    vm.register_import(DLL, "?VirtualProcessorId@Context@Concurrency@@SAIXZ", context_virtual_processor_id);
    vm.register_import(DLL, "?Yield@Context@Concurrency@@SAXXZ", context_yield);

    // CurrentScheduler methods
    vm.register_import(DLL, "?Create@CurrentScheduler@Concurrency@@SAXABVSchedulerPolicy@2@@Z", current_scheduler_create);
    vm.register_import(DLL, "?Detach@CurrentScheduler@Concurrency@@SAXXZ", current_scheduler_detach);
    vm.register_import(DLL, "?Get@CurrentScheduler@Concurrency@@SAPAVScheduler@2@XZ", current_scheduler_get);
    vm.register_import(DLL, "?GetNumberOfVirtualProcessors@CurrentScheduler@Concurrency@@SAIXZ", current_scheduler_get_number_of_virtual_processors);
    vm.register_import(DLL, "?GetPolicy@CurrentScheduler@Concurrency@@SA?AVSchedulerPolicy@2@XZ", current_scheduler_get_policy);
    vm.register_import(DLL, "?Id@CurrentScheduler@Concurrency@@SAIXZ", current_scheduler_id);
    vm.register_import(DLL, "?CreateScheduleGroup@CurrentScheduler@Concurrency@@SAPAVScheduleGroup@2@XZ", current_scheduler_create_schedule_group);
    vm.register_import(DLL, "?RegisterShutdownEvent@CurrentScheduler@Concurrency@@SAXPAX@Z", current_scheduler_register_shutdown_event);
    vm.register_import(DLL, "?ScheduleTask@CurrentScheduler@Concurrency@@SAXP6AXPAX@Z0@Z", current_scheduler_schedule_task);

    // Scheduler methods
    vm.register_import(DLL, "?Create@Scheduler@Concurrency@@SAPAV12@ABVSchedulerPolicy@2@@Z", scheduler_create);
    vm.register_import(DLL, "?SetDefaultSchedulerPolicy@Scheduler@Concurrency@@SAXABVSchedulerPolicy@2@@Z", scheduler_set_default_scheduler_policy);
    vm.register_import(DLL, "?ResetDefaultSchedulerPolicy@Scheduler@Concurrency@@SAXXZ", scheduler_reset_default_scheduler_policy);

    // SchedulerPolicy
    vm.register_import(DLL, "??0SchedulerPolicy@Concurrency@@QAE@XZ", scheduler_policy_ctor);
    vm.register_import(DLL, "??0SchedulerPolicy@Concurrency@@QAA@IZZ", scheduler_policy_ctor);
    vm.register_import(DLL, "??0SchedulerPolicy@Concurrency@@QAE@ABV01@@Z", scheduler_policy_ctor);
    vm.register_import(DLL, "??1SchedulerPolicy@Concurrency@@QAE@XZ", scheduler_policy_dtor);
    vm.register_import(DLL, "??4SchedulerPolicy@Concurrency@@QAEAAV01@ABV01@@Z", scheduler_policy_ctor);
    vm.register_import(DLL, "?GetPolicyValue@SchedulerPolicy@Concurrency@@QBEIW4PolicyElementKey@2@@Z", scheduler_policy_get_policy_value);
    vm.register_import(DLL, "?SetPolicyValue@SchedulerPolicy@Concurrency@@QAEIW4PolicyElementKey@2@I@Z", scheduler_policy_set_policy_value);
    vm.register_import(DLL, "?SetConcurrencyLimits@SchedulerPolicy@Concurrency@@QAEXII@Z", scheduler_policy_set_concurrency_limits);

    // critical_section
    vm.register_import(DLL, "??0critical_section@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "??1critical_section@Concurrency@@QAE@XZ", critical_section_dtor);
    vm.register_import(DLL, "?lock@critical_section@Concurrency@@QAEXXZ", critical_section_lock);
    vm.register_import(DLL, "?try_lock@critical_section@Concurrency@@QAE_NXZ", critical_section_try_lock);
    vm.register_import(DLL, "?unlock@critical_section@Concurrency@@QAEXXZ", critical_section_unlock);
    vm.register_import(DLL, "?native_handle@critical_section@Concurrency@@QAEAAV12@XZ", critical_section_native_handle);

    // reader_writer_lock
    vm.register_import(DLL, "??0reader_writer_lock@Concurrency@@QAE@XZ", reader_writer_lock_ctor);
    vm.register_import(DLL, "??1reader_writer_lock@Concurrency@@QAE@XZ", reader_writer_lock_dtor);
    vm.register_import(DLL, "?lock@reader_writer_lock@Concurrency@@QAEXXZ", reader_writer_lock_lock);
    vm.register_import(DLL, "?lock_read@reader_writer_lock@Concurrency@@QAEXXZ", reader_writer_lock_lock_read);
    vm.register_import(DLL, "?try_lock@reader_writer_lock@Concurrency@@QAE_NXZ", reader_writer_lock_try_lock);
    vm.register_import(DLL, "?try_lock_read@reader_writer_lock@Concurrency@@QAE_NXZ", reader_writer_lock_try_lock_read);
    vm.register_import(DLL, "?unlock@reader_writer_lock@Concurrency@@QAEXXZ", reader_writer_lock_unlock);

    // event
    vm.register_import(DLL, "??0event@Concurrency@@QAE@XZ", event_ctor);
    vm.register_import(DLL, "??1event@Concurrency@@QAE@XZ", event_dtor);
    vm.register_import(DLL, "?set@event@Concurrency@@QAEXXZ", event_set);
    vm.register_import(DLL, "?reset@event@Concurrency@@QAEXXZ", event_reset);
    vm.register_import(DLL, "?wait@event@Concurrency@@QAEII@Z", event_wait);
    vm.register_import(DLL, "?wait_for_multiple@event@Concurrency@@SAIPAPAV12@I_NI@Z", event_wait_for_multiple);

    // _SpinLock
    vm.register_import(DLL, "??0_SpinLock@details@Concurrency@@QAE@ACJ@Z", spin_lock_ctor);
    vm.register_import(DLL, "??1_SpinLock@details@Concurrency@@QAE@XZ", spin_lock_dtor);
    vm.register_import(DLL, "?_Acquire@_SpinLock@details@Concurrency@@QAEXXZ", spin_lock_lock);
    vm.register_import(DLL, "?_TryAcquire@_SpinLock@details@Concurrency@@QAE_NXZ", spin_lock_try_lock);
    vm.register_import(DLL, "?_Release@_SpinLock@details@Concurrency@@QAEXXZ", spin_lock_unlock);

    // _SpinWait
    vm.register_import(DLL, "??0?$_SpinWait@$00@details@Concurrency@@QAE@P6AXXZ@Z", spin_wait_ctor);
    vm.register_import(DLL, "??0?$_SpinWait@$0A@@details@Concurrency@@QAE@P6AXXZ@Z", spin_wait_ctor);
    vm.register_import(DLL, "??_F?$_SpinWait@$00@details@Concurrency@@QAEXXZ", spin_wait_ctor);
    vm.register_import(DLL, "??_F?$_SpinWait@$0A@@details@Concurrency@@QAEXXZ", spin_wait_ctor);
    vm.register_import(DLL, "?_SpinOnce@?$_SpinWait@$00@details@Concurrency@@QAE_NXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_SpinOnce@?$_SpinWait@$0A@@details@Concurrency@@QAE_NXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_Reset@?$_SpinWait@$00@details@Concurrency@@IAEXXZ", spin_wait_reset);
    vm.register_import(DLL, "?_Reset@?$_SpinWait@$0A@@details@Concurrency@@IAEXXZ", spin_wait_reset);
    vm.register_import(DLL, "?_DoYield@?$_SpinWait@$00@details@Concurrency@@IAEXXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_DoYield@?$_SpinWait@$0A@@details@Concurrency@@IAEXXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_NumberOfSpins@?$_SpinWait@$00@details@Concurrency@@IAEKXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_NumberOfSpins@?$_SpinWait@$0A@@details@Concurrency@@IAEKXZ", spin_wait_spin_once);
    vm.register_import(DLL, "?_SetSpinCount@?$_SpinWait@$00@details@Concurrency@@QAEXI@Z", spin_wait_reset);
    vm.register_import(DLL, "?_SetSpinCount@?$_SpinWait@$0A@@details@Concurrency@@QAEXI@Z", spin_wait_reset);
    vm.register_import(DLL, "??4?$_SpinWait@$00@details@Concurrency@@QAEAAV012@ABV012@@Z", spin_wait_ctor);
    vm.register_import(DLL, "??4?$_SpinWait@$0A@@details@Concurrency@@QAEAAV012@ABV012@@Z", spin_wait_ctor);

    // _TaskCollection
    vm.register_import(DLL, "??0_TaskCollection@details@Concurrency@@QAE@XZ", task_collection_ctor);
    vm.register_import(DLL, "??1_TaskCollection@details@Concurrency@@QAE@XZ", task_collection_dtor);
    vm.register_import(DLL, "?_Cancel@_TaskCollection@details@Concurrency@@QAEXXZ", task_collection_cancel);
    vm.register_import(DLL, "?_IsCanceling@_TaskCollection@details@Concurrency@@QAE_NXZ", task_collection_is_canceling);
    vm.register_import(DLL, "?_RunAndWait@_TaskCollection@details@Concurrency@@QAG?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z", task_collection_run_and_wait);
    vm.register_import(DLL, "?_Schedule@_TaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@@Z", task_collection_schedule);

    // _StructuredTaskCollection
    vm.register_import(DLL, "??0_StructuredTaskCollection@details@Concurrency@@QAE@XZ", structured_task_collection_ctor);
    vm.register_import(DLL, "??1_StructuredTaskCollection@details@Concurrency@@QAE@XZ", structured_task_collection_dtor);
    vm.register_import(DLL, "?_Abort@_StructuredTaskCollection@details@Concurrency@@AAEXXZ", structured_task_collection_cancel);
    vm.register_import(DLL, "?_Cancel@_StructuredTaskCollection@details@Concurrency@@QAEXXZ", structured_task_collection_cancel);
    vm.register_import(DLL, "?_IsCanceling@_StructuredTaskCollection@details@Concurrency@@QAE_NXZ", structured_task_collection_is_canceling);
    vm.register_import(DLL, "?_RunAndWait@_StructuredTaskCollection@details@Concurrency@@QAG?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z", structured_task_collection_run_and_wait);
    vm.register_import(DLL, "?_Schedule@_StructuredTaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@@Z", structured_task_collection_schedule);

    // Scoped locks
    vm.register_import(DLL, "??0scoped_lock@critical_section@Concurrency@@QAE@AAV12@@Z", critical_section_lock);
    vm.register_import(DLL, "??1scoped_lock@critical_section@Concurrency@@QAE@XZ", critical_section_unlock);
    vm.register_import(DLL, "??0scoped_lock@reader_writer_lock@Concurrency@@QAE@AAV12@@Z", reader_writer_lock_lock);
    vm.register_import(DLL, "??1scoped_lock@reader_writer_lock@Concurrency@@QAE@XZ", reader_writer_lock_unlock);
    vm.register_import(DLL, "??0scoped_lock_read@reader_writer_lock@Concurrency@@QAE@AAV12@@Z", reader_writer_lock_lock_read);
    vm.register_import(DLL, "??1scoped_lock_read@reader_writer_lock@Concurrency@@QAE@XZ", reader_writer_lock_unlock);

    // _NonReentrantBlockingLock
    vm.register_import(DLL, "??0_NonReentrantBlockingLock@details@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "??1_NonReentrantBlockingLock@details@Concurrency@@QAE@XZ", critical_section_dtor);
    vm.register_import(DLL, "?_Acquire@_NonReentrantBlockingLock@details@Concurrency@@QAEXXZ", critical_section_lock);
    vm.register_import(DLL, "?_Release@_NonReentrantBlockingLock@details@Concurrency@@QAEXXZ", critical_section_unlock);

    // _ReentrantBlockingLock
    vm.register_import(DLL, "??0_ReentrantBlockingLock@details@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "??1_ReentrantBlockingLock@details@Concurrency@@QAE@XZ", critical_section_dtor);
    vm.register_import(DLL, "?_Acquire@_ReentrantBlockingLock@details@Concurrency@@QAEXXZ", critical_section_lock);
    vm.register_import(DLL, "?_Release@_ReentrantBlockingLock@details@Concurrency@@QAEXXZ", critical_section_unlock);

    // _ReentrantLock
    vm.register_import(DLL, "??0_ReentrantLock@details@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "?_Acquire@_ReentrantLock@details@Concurrency@@QAEXXZ", critical_section_lock);
    vm.register_import(DLL, "?_Release@_ReentrantLock@details@Concurrency@@QAEXXZ", critical_section_unlock);

    // _NonReentrantPPLLock
    vm.register_import(DLL, "??0_NonReentrantPPLLock@details@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "?_Acquire@_NonReentrantPPLLock@details@Concurrency@@QAEXPAX@Z", critical_section_lock);
    vm.register_import(DLL, "?_Release@_NonReentrantPPLLock@details@Concurrency@@QAEXXZ", critical_section_unlock);
    vm.register_import(DLL, "??0_Scoped_lock@_NonReentrantPPLLock@details@Concurrency@@QAE@AAV123@@Z", critical_section_lock);
    vm.register_import(DLL, "??1_Scoped_lock@_NonReentrantPPLLock@details@Concurrency@@QAE@XZ", critical_section_unlock);

    // _ReentrantPPLLock
    vm.register_import(DLL, "??0_ReentrantPPLLock@details@Concurrency@@QAE@XZ", critical_section_ctor);
    vm.register_import(DLL, "?_Acquire@_ReentrantPPLLock@details@Concurrency@@QAEXPAX@Z", critical_section_lock);
    vm.register_import(DLL, "?_Release@_ReentrantPPLLock@details@Concurrency@@QAEXXZ", critical_section_unlock);
    vm.register_import(DLL, "??0_Scoped_lock@_ReentrantPPLLock@details@Concurrency@@QAE@AAV123@@Z", critical_section_lock);
    vm.register_import(DLL, "??1_Scoped_lock@_ReentrantPPLLock@details@Concurrency@@QAE@XZ", critical_section_unlock);

    // _ReaderWriterLock
    vm.register_import(DLL, "??0_ReaderWriterLock@details@Concurrency@@QAE@XZ", reader_writer_lock_ctor);
    vm.register_import(DLL, "?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ", reader_writer_lock_lock_read);
    vm.register_import(DLL, "?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ", reader_writer_lock_lock);
    vm.register_import(DLL, "?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ", reader_writer_lock_unlock);
    vm.register_import(DLL, "?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ", reader_writer_lock_unlock);

    // _Timer
    vm.register_import(DLL, "??0_Timer@details@Concurrency@@IAE@I_N@Z", event_ctor);
    vm.register_import(DLL, "??1_Timer@details@Concurrency@@IAE@XZ", event_dtor);
    vm.register_import(DLL, "?_Start@_Timer@details@Concurrency@@IAEXXZ", event_set);
    vm.register_import(DLL, "?_Stop@_Timer@details@Concurrency@@IAEXXZ", event_reset);

    // Utility functions
    vm.register_import(DLL, "?GetExecutionContextId@Concurrency@@YAIXZ", context_id);
    vm.register_import(DLL, "?GetSchedulerId@Concurrency@@YAIXZ", current_scheduler_id);
    vm.register_import(DLL, "?GetProcessorCount@Concurrency@@YAIXZ", current_scheduler_get_number_of_virtual_processors);
    vm.register_import(DLL, "?GetProcessorNodeCount@Concurrency@@YAIXZ", current_scheduler_get_number_of_virtual_processors);
    vm.register_import(DLL, "?GetOSVersion@Concurrency@@YA?AW4OSVersion@IResourceManager@1@XZ", context_id);
    vm.register_import(DLL, "?CreateResourceManager@Concurrency@@YAPAUIResourceManager@1@XZ", context_current_context);
    vm.register_import(DLL, "?GetSharedTimerQueue@details@Concurrency@@YAPAXXZ", context_current_context);
    vm.register_import(DLL, "?DisableTracing@Concurrency@@YAJXZ", context_id);
    vm.register_import(DLL, "?EnableTracing@Concurrency@@YAJXZ", context_id);
    vm.register_import(DLL, "?Log2@details@Concurrency@@YAKI@Z", context_id);
    vm.register_import(DLL, "?_CheckTaskCollection@_UnrealizedChore@details@Concurrency@@IAEXXZ", task_collection_is_canceling);

    // ConcRT debug/assert
    vm.register_import(DLL, "?_ConcRT_Assert@details@Concurrency@@YAXPBD0H@Z", context_block);
    vm.register_import(DLL, "?_ConcRT_CoreAssert@details@Concurrency@@YAXPBD0H@Z", context_block);
    vm.register_import(DLL, "?_ConcRT_DumpMessage@details@Concurrency@@YAXPB_WZZ", context_block);
    vm.register_import(DLL, "?_ConcRT_Trace@details@Concurrency@@YAXHPB_WZZ", context_block);
}
