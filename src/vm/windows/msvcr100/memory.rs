//! Memory management stubs for MSVCR100.dll.

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

// Memory allocation
stub!(malloc_impl);
stub!(calloc_impl);
stub!(realloc_impl);
stub!(free_impl);
stub!(aligned_malloc);
stub!(aligned_realloc);
stub!(aligned_recalloc);
stub!(aligned_free);
stub!(aligned_msize);
stub!(aligned_offset_malloc);
stub!(aligned_offset_realloc);
stub!(aligned_offset_recalloc);
stub!(msize);
stub!(expand);
stub!(heapchk);
stub!(heapmin);
stub!(heapset);
stub!(heapwalk);
stub!(heapadd);
stub!(heapused);
stub!(recalloc);
stub!(query_new_handler);
stub!(query_new_mode);
stub!(set_new_handler);
stub!(set_new_mode);
stub!(crt_dbg_break);
stub!(crt_dbg_report);
stub!(crt_dbg_report_w);
stub!(crt_set_alloc_hook);
stub!(crt_set_break_alloc);
stub!(crt_set_dbg_flag);
stub!(crt_set_dump_client);
stub!(crt_set_report_file);
stub!(crt_set_report_hook);
stub!(crt_set_report_hook2);
stub!(crt_set_report_mode);
stub!(crt_mem_checkpoint);
stub!(crt_mem_difference);
stub!(crt_mem_dump_all_objects_since);
stub!(crt_mem_dump_statistics);
stub!(crt_is_valid_heap_pointer);
stub!(crt_is_valid_pointer);
stub!(crt_check_memory);
stub!(crt_dump_memory_leaks);
stub!(get_heap_handle);
stub!(sbh_heap_init);
stub!(new_operator);
stub!(new_operator_debug);
stub!(delete_operator);
stub!(new_array_operator);
stub!(new_array_operator_debug);
stub!(delete_array_operator);
stub!(nh_malloc);
stub!(nh_malloc_dbg);
stub!(heap_alloc);
stub!(heap_realloc);
stub!(heap_free);
stub!(locked_malloc);
stub!(locked_free);
stub!(purecall);
stub!(onexit);
stub!(initterm);
stub!(initterm_e);

pub fn register(vm: &mut Vm) {
    // Standard C memory functions
    vm.register_import(DLL, "malloc", malloc_impl);
    vm.register_import(DLL, "calloc", calloc_impl);
    vm.register_import(DLL, "realloc", realloc_impl);
    vm.register_import(DLL, "free", free_impl);

    // Aligned memory
    vm.register_import(DLL, "_aligned_malloc", aligned_malloc);
    vm.register_import(DLL, "_aligned_realloc", aligned_realloc);
    vm.register_import(DLL, "_aligned_recalloc", aligned_recalloc);
    vm.register_import(DLL, "_aligned_free", aligned_free);
    vm.register_import(DLL, "_aligned_msize", aligned_msize);
    vm.register_import(DLL, "_aligned_offset_malloc", aligned_offset_malloc);
    vm.register_import(DLL, "_aligned_offset_realloc", aligned_offset_realloc);
    vm.register_import(DLL, "_aligned_offset_recalloc", aligned_offset_recalloc);

    // Heap functions
    vm.register_import(DLL, "_msize", msize);
    vm.register_import(DLL, "_expand", expand);
    vm.register_import(DLL, "_heapchk", heapchk);
    vm.register_import(DLL, "_heapmin", heapmin);
    vm.register_import(DLL, "_heapset", heapset);
    vm.register_import(DLL, "_heapwalk", heapwalk);
    vm.register_import(DLL, "_heapadd", heapadd);
    vm.register_import(DLL, "_heapused", heapused);
    vm.register_import(DLL, "_recalloc", recalloc);
    vm.register_import(DLL, "_get_heap_handle", get_heap_handle);
    vm.register_import(DLL, "__sbh_heap_init", sbh_heap_init);

    // New/delete handlers
    vm.register_import(DLL, "_query_new_handler", query_new_handler);
    vm.register_import(DLL, "_query_new_mode", query_new_mode);
    vm.register_import(DLL, "_set_new_handler", set_new_handler);
    vm.register_import(DLL, "_set_new_mode", set_new_mode);

    // Debug heap functions
    vm.register_import(DLL, "_CrtDbgBreak", crt_dbg_break);
    vm.register_import(DLL, "_CrtDbgReport", crt_dbg_report);
    vm.register_import(DLL, "_CrtDbgReportW", crt_dbg_report_w);
    vm.register_import(DLL, "_CrtSetAllocHook", crt_set_alloc_hook);
    vm.register_import(DLL, "_CrtSetBreakAlloc", crt_set_break_alloc);
    vm.register_import(DLL, "_CrtSetDbgFlag", crt_set_dbg_flag);
    vm.register_import(DLL, "_CrtSetDumpClient", crt_set_dump_client);
    vm.register_import(DLL, "_CrtSetReportFile", crt_set_report_file);
    vm.register_import(DLL, "_CrtSetReportHook", crt_set_report_hook);
    vm.register_import(DLL, "_CrtSetReportHook2", crt_set_report_hook2);
    vm.register_import(DLL, "_CrtSetReportMode", crt_set_report_mode);
    vm.register_import(DLL, "_CrtMemCheckpoint", crt_mem_checkpoint);
    vm.register_import(DLL, "_CrtMemDifference", crt_mem_difference);
    vm.register_import(DLL, "_CrtMemDumpAllObjectsSince", crt_mem_dump_all_objects_since);
    vm.register_import(DLL, "_CrtMemDumpStatistics", crt_mem_dump_statistics);
    vm.register_import(DLL, "_CrtIsValidHeapPointer", crt_is_valid_heap_pointer);
    vm.register_import(DLL, "_CrtIsValidPointer", crt_is_valid_pointer);
    vm.register_import(DLL, "_CrtCheckMemory", crt_check_memory);
    vm.register_import(DLL, "_CrtDumpMemoryLeaks", crt_dump_memory_leaks);

    // C++ operators new/delete
    vm.register_import(DLL, "??2@YAPAXI@Z", new_operator);
    vm.register_import(DLL, "??2@YAPAXIHPBDH@Z", new_operator_debug);
    vm.register_import(DLL, "??3@YAXPAX@Z", delete_operator);
    vm.register_import(DLL, "??_U@YAPAXI@Z", new_array_operator);
    vm.register_import(DLL, "??_U@YAPAXIHPBDH@Z", new_array_operator_debug);
    vm.register_import(DLL, "??_V@YAXPAX@Z", delete_array_operator);

    // Internal allocation
    vm.register_import(DLL, "_nh_malloc", nh_malloc);
    vm.register_import(DLL, "_nh_malloc_dbg", nh_malloc_dbg);
    vm.register_import(DLL, "_heap_alloc", heap_alloc);
    vm.register_import(DLL, "_heap_realloc", heap_realloc);
    vm.register_import(DLL, "_heap_free", heap_free);
    vm.register_import(DLL, "_callnewh", set_new_handler);
    vm.register_import(DLL, "_lock", locked_malloc);
    vm.register_import(DLL, "_unlock", locked_free);

    // Pure virtual call
    vm.register_import(DLL, "_purecall", purecall);

    // Initialization
    vm.register_import(DLL, "_onexit", onexit);
    vm.register_import(DLL, "_initterm", initterm);
    vm.register_import(DLL, "_initterm_e", initterm_e);
}
