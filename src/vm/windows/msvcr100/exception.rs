//! Exception handling stubs for MSVCR100.dll.
#![allow(dead_code)]

use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";

// Exception handling
define_stub_fn!(DLL, cxx_throw_exception, 0);
define_stub_fn!(DLL, cxx_frame_handler, 0);
define_stub_fn!(DLL, cxx_frame_handler2, 0);
define_stub_fn!(DLL, cxx_frame_handler3, 0);
define_stub_fn!(DLL, cxx_call_unwind_dtor, 0);
define_stub_fn!(DLL, cxx_call_unwind_del_dtor, 0);
define_stub_fn!(DLL, cxx_call_unwind_std_del_dtor, 0);
define_stub_fn!(DLL, cxx_call_unwind_vec_dtor, 0);
define_stub_fn!(DLL, cxx_detect_rethrow, 0);
define_stub_fn!(DLL, cxx_exception_filter, 0);
define_stub_fn!(DLL, cxx_longjmp_unwind, 0);
define_stub_fn!(DLL, cxx_query_exception_size, 0);
define_stub_fn!(DLL, cxx_register_exception_object, 0);
define_stub_fn!(DLL, cxx_unregister_exception_object, 0);
define_stub_fn!(DLL, destruct_exception_object, 0);
define_stub_fn!(DLL, is_exception_object_to_be_destroyed, 0);
define_stub_fn!(DLL, frame_unwind_filter, 0);
define_stub_fn!(DLL, xcpt_filter, 0);
define_stub_fn!(DLL, cpp_xcpt_filter, 0);
define_stub_fn!(DLL, eh_prolog, 0);
define_stub_fn!(DLL, create_frame_info, 0);
define_stub_fn!(DLL, find_and_unlink_frame, 0);
define_stub_fn!(DLL, adjust_pointer, 0);
define_stub_fn!(DLL, build_catch_object, 0);
define_stub_fn!(DLL, build_catch_object_helper, 0);
define_stub_fn!(DLL, type_match, 0);
define_stub_fn!(DLL, nlg_dispatch2, 0);
define_stub_fn!(DLL, nlg_return, 0);
define_stub_fn!(DLL, nlg_return2, 0);
define_stub_fn!(DLL, abnormal_termination, 0);
define_stub_fn!(DLL, longjmp, 0);
define_stub_fn!(DLL, setjmp, 0);
define_stub_fn!(DLL, setjmp3, 0);
define_stub_fn!(DLL, rt_cast_to_void, 0);
define_stub_fn!(DLL, rt_dynamic_cast, 0);
define_stub_fn!(DLL, rt_typeid, 0);
define_stub_fn!(DLL, unwind_call, 0);

// Exception classes
define_stub_fn!(DLL, exception_ctor, 0);
define_stub_fn!(DLL, exception_ctor_str, 0);
define_stub_fn!(DLL, exception_ctor_copy, 0);
define_stub_fn!(DLL, exception_dtor, 0);
define_stub_fn!(DLL, exception_what, 0);
define_stub_fn!(DLL, exception_copy_str, 0);
define_stub_fn!(DLL, exception_assign, 0);
define_stub_fn!(DLL, bad_cast_ctor, 0);
define_stub_fn!(DLL, bad_cast_dtor, 0);
define_stub_fn!(DLL, bad_typeid_ctor, 0);
define_stub_fn!(DLL, bad_typeid_dtor, 0);
define_stub_fn!(DLL, non_rtti_object_ctor, 0);
define_stub_fn!(DLL, non_rtti_object_dtor, 0);
define_stub_fn!(DLL, type_info_dtor, 0);
define_stub_fn!(DLL, type_info_name, 0);
define_stub_fn!(DLL, type_info_raw_name, 0);
define_stub_fn!(DLL, type_info_eq, 0);
define_stub_fn!(DLL, type_info_ne, 0);
define_stub_fn!(DLL, type_info_before, 0);
define_stub_fn!(DLL, uncaught_exception, 0);

// Concurrency exception classes
define_stub_fn!(DLL, bad_target_ctor, 0);
define_stub_fn!(DLL, context_self_unblock_ctor, 0);
define_stub_fn!(DLL, context_unblock_unbalanced_ctor, 0);
define_stub_fn!(DLL, default_scheduler_exists_ctor, 0);
define_stub_fn!(DLL, improper_lock_ctor, 0);
define_stub_fn!(DLL, improper_scheduler_attach_ctor, 0);
define_stub_fn!(DLL, improper_scheduler_detach_ctor, 0);
define_stub_fn!(DLL, improper_scheduler_reference_ctor, 0);
define_stub_fn!(DLL, invalid_link_target_ctor, 0);
define_stub_fn!(DLL, invalid_multiple_scheduling_ctor, 0);
define_stub_fn!(DLL, invalid_operation_ctor, 0);
define_stub_fn!(DLL, invalid_oversubscribe_operation_ctor, 0);
define_stub_fn!(DLL, invalid_scheduler_policy_key_ctor, 0);
define_stub_fn!(DLL, invalid_scheduler_policy_thread_specification_ctor, 0);
define_stub_fn!(DLL, invalid_scheduler_policy_value_ctor, 0);
define_stub_fn!(DLL, message_not_found_ctor, 0);
define_stub_fn!(DLL, missing_wait_ctor, 0);
define_stub_fn!(DLL, nested_scheduler_missing_detach_ctor, 0);
define_stub_fn!(DLL, operation_timed_out_ctor, 0);
define_stub_fn!(DLL, scheduler_not_attached_ctor, 0);
define_stub_fn!(DLL, scheduler_resource_allocation_error_ctor, 0);
define_stub_fn!(DLL, task_canceled_ctor, 0);
define_stub_fn!(DLL, unsupported_os_ctor, 0);

pub fn register(vm: &mut Vm) {
    // C++ exception handling
    vm.register_import(DLL, "_CxxThrowException", cxx_throw_exception);
    vm.register_import(DLL, "__CxxFrameHandler", cxx_frame_handler);
    vm.register_import(DLL, "__CxxFrameHandler2", cxx_frame_handler2);
    vm.register_import(DLL, "__CxxFrameHandler3", cxx_frame_handler3);
    vm.register_import(DLL, "__CxxCallUnwindDtor", cxx_call_unwind_dtor);
    vm.register_import(DLL, "__CxxCallUnwindDelDtor", cxx_call_unwind_del_dtor);
    vm.register_import(
        DLL,
        "__CxxCallUnwindStdDelDtor",
        cxx_call_unwind_std_del_dtor,
    );
    vm.register_import(DLL, "__CxxCallUnwindVecDtor", cxx_call_unwind_vec_dtor);
    vm.register_import(DLL, "__CxxDetectRethrow", cxx_detect_rethrow);
    vm.register_import(DLL, "__CxxExceptionFilter", cxx_exception_filter);
    vm.register_import(DLL, "__CxxLongjmpUnwind", cxx_longjmp_unwind);
    vm.register_import(DLL, "__CxxQueryExceptionSize", cxx_query_exception_size);
    vm.register_import(
        DLL,
        "__CxxRegisterExceptionObject",
        cxx_register_exception_object,
    );
    vm.register_import(
        DLL,
        "__CxxUnregisterExceptionObject",
        cxx_unregister_exception_object,
    );
    vm.register_import(DLL, "__DestructExceptionObject", destruct_exception_object);
    vm.register_import(
        DLL,
        "_IsExceptionObjectToBeDestroyed",
        is_exception_object_to_be_destroyed,
    );
    vm.register_import(DLL, "__FrameUnwindFilter", frame_unwind_filter);
    vm.register_import(DLL, "_XcptFilter", xcpt_filter);
    vm.register_import(DLL, "__CppXcptFilter", cpp_xcpt_filter);
    vm.register_import(DLL, "_EH_prolog", eh_prolog);
    vm.register_import(DLL, "_CreateFrameInfo", create_frame_info);
    vm.register_import(DLL, "_FindAndUnlinkFrame", find_and_unlink_frame);
    vm.register_import(DLL, "__AdjustPointer", adjust_pointer);
    vm.register_import(DLL, "__BuildCatchObject", build_catch_object);
    vm.register_import(DLL, "__BuildCatchObjectHelper", build_catch_object_helper);
    vm.register_import(DLL, "__TypeMatch", type_match);
    vm.register_import(DLL, "_NLG_Dispatch2", nlg_dispatch2);
    vm.register_import(DLL, "_NLG_Return", nlg_return);
    vm.register_import(DLL, "_NLG_Return2", nlg_return2);
    vm.register_import(DLL, "_abnormal_termination", abnormal_termination);
    vm.register_import(DLL, "longjmp", longjmp);
    vm.register_import(DLL, "_setjmp", setjmp);
    vm.register_import(DLL, "_setjmp3", setjmp3);
    vm.register_import(DLL, "__RTCastToVoid", rt_cast_to_void);
    vm.register_import(DLL, "__RTDynamicCast", rt_dynamic_cast);
    vm.register_import(DLL, "__RTtypeid", rt_typeid);

    // exception class
    vm.register_import(DLL, "??0exception@std@@QAE@XZ", exception_ctor);
    vm.register_import(DLL, "??0exception@std@@QAE@ABQBD@Z", exception_ctor_str);
    vm.register_import(DLL, "??0exception@std@@QAE@ABQBDH@Z", exception_ctor_str);
    vm.register_import(DLL, "??0exception@std@@QAE@ABV01@@Z", exception_ctor_copy);
    vm.register_import(DLL, "??1exception@std@@UAE@XZ", exception_dtor);
    vm.register_import(DLL, "?what@exception@std@@UBEPBDXZ", exception_what);
    vm.register_import(
        DLL,
        "?_Copy_str@exception@std@@AAEXPBD@Z",
        exception_copy_str,
    );
    vm.register_import(DLL, "??4exception@std@@QAEAAV01@ABV01@@Z", exception_assign);
    vm.register_import(DLL, "??_7exception@std@@6B@", exception_ctor);
    vm.register_import(DLL, "??_7exception@@6B@", exception_ctor);

    // bad_cast
    vm.register_import(DLL, "??0bad_cast@std@@QAE@PBD@Z", bad_cast_ctor);
    vm.register_import(DLL, "??0bad_cast@std@@AAE@PBQBD@Z", bad_cast_ctor);
    vm.register_import(DLL, "??0bad_cast@std@@QAE@ABV01@@Z", bad_cast_ctor);
    vm.register_import(DLL, "??1bad_cast@std@@UAE@XZ", bad_cast_dtor);
    vm.register_import(DLL, "??4bad_cast@std@@QAEAAV01@ABV01@@Z", bad_cast_ctor);
    vm.register_import(DLL, "??_Fbad_cast@std@@QAEXXZ", bad_cast_ctor);
    vm.register_import(DLL, "??_7bad_cast@std@@6B@", bad_cast_ctor);

    // bad_typeid
    vm.register_import(DLL, "??0bad_typeid@std@@QAE@PBD@Z", bad_typeid_ctor);
    vm.register_import(DLL, "??0bad_typeid@std@@QAE@ABV01@@Z", bad_typeid_ctor);
    vm.register_import(DLL, "??1bad_typeid@std@@UAE@XZ", bad_typeid_dtor);
    vm.register_import(DLL, "??4bad_typeid@std@@QAEAAV01@ABV01@@Z", bad_typeid_ctor);
    vm.register_import(DLL, "??_Fbad_typeid@std@@QAEXXZ", bad_typeid_ctor);
    vm.register_import(DLL, "??_7bad_typeid@std@@6B@", bad_typeid_ctor);

    // __non_rtti_object
    vm.register_import(
        DLL,
        "??0__non_rtti_object@std@@QAE@PBD@Z",
        non_rtti_object_ctor,
    );
    vm.register_import(
        DLL,
        "??0__non_rtti_object@std@@QAE@ABV01@@Z",
        non_rtti_object_ctor,
    );
    vm.register_import(
        DLL,
        "??1__non_rtti_object@std@@UAE@XZ",
        non_rtti_object_dtor,
    );
    vm.register_import(
        DLL,
        "??4__non_rtti_object@std@@QAEAAV01@ABV01@@Z",
        non_rtti_object_ctor,
    );
    vm.register_import(DLL, "??_7__non_rtti_object@std@@6B@", non_rtti_object_ctor);

    // type_info
    vm.register_import(DLL, "??1type_info@@UAE@XZ", type_info_dtor);
    vm.register_import(
        DLL,
        "?name@type_info@@QBEPBDPAU__type_info_node@@@Z",
        type_info_name,
    );
    vm.register_import(DLL, "?raw_name@type_info@@QBEPBDXZ", type_info_raw_name);
    vm.register_import(DLL, "??8type_info@@QBE_NABV0@@Z", type_info_eq);
    vm.register_import(DLL, "??9type_info@@QBE_NABV0@@Z", type_info_ne);
    vm.register_import(DLL, "?before@type_info@@QBEHABV1@@Z", type_info_before);
    vm.register_import(
        DLL,
        "?_Name_base@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z",
        type_info_name,
    );
    vm.register_import(
        DLL,
        "?_Name_base_internal@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z",
        type_info_name,
    );
    vm.register_import(DLL, "__clean_type_info_names_internal", type_info_dtor);
    vm.register_import(DLL, "__uncaught_exception", uncaught_exception);

    // Concurrency exceptions
    vm.register_import(DLL, "??0bad_target@Concurrency@@QAE@PBD@Z", bad_target_ctor);
    vm.register_import(DLL, "??0bad_target@Concurrency@@QAE@XZ", bad_target_ctor);
    vm.register_import(
        DLL,
        "??0context_self_unblock@Concurrency@@QAE@PBD@Z",
        context_self_unblock_ctor,
    );
    vm.register_import(
        DLL,
        "??0context_self_unblock@Concurrency@@QAE@XZ",
        context_self_unblock_ctor,
    );
    vm.register_import(
        DLL,
        "??0context_unblock_unbalanced@Concurrency@@QAE@PBD@Z",
        context_unblock_unbalanced_ctor,
    );
    vm.register_import(
        DLL,
        "??0context_unblock_unbalanced@Concurrency@@QAE@XZ",
        context_unblock_unbalanced_ctor,
    );
    vm.register_import(
        DLL,
        "??0default_scheduler_exists@Concurrency@@QAE@PBD@Z",
        default_scheduler_exists_ctor,
    );
    vm.register_import(
        DLL,
        "??0default_scheduler_exists@Concurrency@@QAE@XZ",
        default_scheduler_exists_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_lock@Concurrency@@QAE@PBD@Z",
        improper_lock_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_lock@Concurrency@@QAE@XZ",
        improper_lock_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_attach@Concurrency@@QAE@PBD@Z",
        improper_scheduler_attach_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_attach@Concurrency@@QAE@XZ",
        improper_scheduler_attach_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_detach@Concurrency@@QAE@PBD@Z",
        improper_scheduler_detach_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_detach@Concurrency@@QAE@XZ",
        improper_scheduler_detach_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_reference@Concurrency@@QAE@PBD@Z",
        improper_scheduler_reference_ctor,
    );
    vm.register_import(
        DLL,
        "??0improper_scheduler_reference@Concurrency@@QAE@XZ",
        improper_scheduler_reference_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_link_target@Concurrency@@QAE@PBD@Z",
        invalid_link_target_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_link_target@Concurrency@@QAE@XZ",
        invalid_link_target_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_multiple_scheduling@Concurrency@@QAE@PBD@Z",
        invalid_multiple_scheduling_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_multiple_scheduling@Concurrency@@QAE@XZ",
        invalid_multiple_scheduling_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_operation@Concurrency@@QAE@PBD@Z",
        invalid_operation_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_operation@Concurrency@@QAE@XZ",
        invalid_operation_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_oversubscribe_operation@Concurrency@@QAE@PBD@Z",
        invalid_oversubscribe_operation_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_oversubscribe_operation@Concurrency@@QAE@XZ",
        invalid_oversubscribe_operation_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_key@Concurrency@@QAE@PBD@Z",
        invalid_scheduler_policy_key_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_key@Concurrency@@QAE@XZ",
        invalid_scheduler_policy_key_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_thread_specification@Concurrency@@QAE@PBD@Z",
        invalid_scheduler_policy_thread_specification_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_thread_specification@Concurrency@@QAE@XZ",
        invalid_scheduler_policy_thread_specification_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_value@Concurrency@@QAE@PBD@Z",
        invalid_scheduler_policy_value_ctor,
    );
    vm.register_import(
        DLL,
        "??0invalid_scheduler_policy_value@Concurrency@@QAE@XZ",
        invalid_scheduler_policy_value_ctor,
    );
    vm.register_import(
        DLL,
        "??0message_not_found@Concurrency@@QAE@PBD@Z",
        message_not_found_ctor,
    );
    vm.register_import(
        DLL,
        "??0message_not_found@Concurrency@@QAE@XZ",
        message_not_found_ctor,
    );
    vm.register_import(
        DLL,
        "??0missing_wait@Concurrency@@QAE@PBD@Z",
        missing_wait_ctor,
    );
    vm.register_import(
        DLL,
        "??0missing_wait@Concurrency@@QAE@XZ",
        missing_wait_ctor,
    );
    vm.register_import(
        DLL,
        "??0nested_scheduler_missing_detach@Concurrency@@QAE@PBD@Z",
        nested_scheduler_missing_detach_ctor,
    );
    vm.register_import(
        DLL,
        "??0nested_scheduler_missing_detach@Concurrency@@QAE@XZ",
        nested_scheduler_missing_detach_ctor,
    );
    vm.register_import(
        DLL,
        "??0operation_timed_out@Concurrency@@QAE@PBD@Z",
        operation_timed_out_ctor,
    );
    vm.register_import(
        DLL,
        "??0operation_timed_out@Concurrency@@QAE@XZ",
        operation_timed_out_ctor,
    );
    vm.register_import(
        DLL,
        "??0scheduler_not_attached@Concurrency@@QAE@PBD@Z",
        scheduler_not_attached_ctor,
    );
    vm.register_import(
        DLL,
        "??0scheduler_not_attached@Concurrency@@QAE@XZ",
        scheduler_not_attached_ctor,
    );
    vm.register_import(
        DLL,
        "??0scheduler_resource_allocation_error@Concurrency@@QAE@J@Z",
        scheduler_resource_allocation_error_ctor,
    );
    vm.register_import(
        DLL,
        "??0scheduler_resource_allocation_error@Concurrency@@QAE@PBDJ@Z",
        scheduler_resource_allocation_error_ctor,
    );
    vm.register_import(
        DLL,
        "??0task_canceled@details@Concurrency@@QAE@PBD@Z",
        task_canceled_ctor,
    );
    vm.register_import(
        DLL,
        "??0task_canceled@details@Concurrency@@QAE@XZ",
        task_canceled_ctor,
    );
    vm.register_import(
        DLL,
        "??0unsupported_os@Concurrency@@QAE@PBD@Z",
        unsupported_os_ctor,
    );
    vm.register_import(
        DLL,
        "??0unsupported_os@Concurrency@@QAE@XZ",
        unsupported_os_ctor,
    );
}
