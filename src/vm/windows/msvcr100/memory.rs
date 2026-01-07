//! Memory management stubs for MSVCR100.dll.

use crate::vm::windows::check_stub;
use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";



// Memory allocation
define_stub_fn!(DLL, malloc_impl, 0);
define_stub_fn!(DLL, calloc_impl, 0);
define_stub_fn!(DLL, realloc_impl, 0);
define_stub_fn!(DLL, free_impl, 0);
define_stub_fn!(DLL, aligned_malloc, 0);
define_stub_fn!(DLL, aligned_realloc, 0);
define_stub_fn!(DLL, aligned_recalloc, 0);
define_stub_fn!(DLL, aligned_free, 0);
define_stub_fn!(DLL, aligned_msize, 0);
define_stub_fn!(DLL, aligned_offset_malloc, 0);
define_stub_fn!(DLL, aligned_offset_realloc, 0);
define_stub_fn!(DLL, aligned_offset_recalloc, 0);
define_stub_fn!(DLL, msize, 0);
define_stub_fn!(DLL, expand, 0);
define_stub_fn!(DLL, heapchk, 0);
define_stub_fn!(DLL, heapmin, 0);
define_stub_fn!(DLL, heapset, 0);
define_stub_fn!(DLL, heapwalk, 0);
define_stub_fn!(DLL, heapadd, 0);
define_stub_fn!(DLL, heapused, 0);
define_stub_fn!(DLL, recalloc, 0);
define_stub_fn!(DLL, query_new_handler, 0);
define_stub_fn!(DLL, query_new_mode, 0);
define_stub_fn!(DLL, set_new_handler, 0);
define_stub_fn!(DLL, set_new_mode, 0);
define_stub_fn!(DLL, crt_dbg_break, 0);
define_stub_fn!(DLL, crt_dbg_report, 0);
define_stub_fn!(DLL, crt_dbg_report_w, 0);
define_stub_fn!(DLL, crt_set_alloc_hook, 0);
define_stub_fn!(DLL, crt_set_break_alloc, 0);
define_stub_fn!(DLL, crt_set_dbg_flag, 0);
define_stub_fn!(DLL, crt_set_dump_client, 0);
define_stub_fn!(DLL, crt_set_report_file, 0);
define_stub_fn!(DLL, crt_set_report_hook, 0);
define_stub_fn!(DLL, crt_set_report_hook2, 0);
define_stub_fn!(DLL, crt_set_report_mode, 0);
define_stub_fn!(DLL, crt_mem_checkpoint, 0);
define_stub_fn!(DLL, crt_mem_difference, 0);
define_stub_fn!(DLL, crt_mem_dump_all_objects_since, 0);
define_stub_fn!(DLL, crt_mem_dump_statistics, 0);
define_stub_fn!(DLL, crt_is_valid_heap_pointer, 0);
define_stub_fn!(DLL, crt_is_valid_pointer, 0);
define_stub_fn!(DLL, crt_check_memory, 0);
define_stub_fn!(DLL, crt_dump_memory_leaks, 0);
define_stub_fn!(DLL, get_heap_handle, 0);
define_stub_fn!(DLL, sbh_heap_init, 0);
define_stub_fn!(DLL, new_operator, 0);
define_stub_fn!(DLL, new_operator_debug, 0);
define_stub_fn!(DLL, delete_operator, 0);
define_stub_fn!(DLL, new_array_operator, 0);
define_stub_fn!(DLL, new_array_operator_debug, 0);
define_stub_fn!(DLL, delete_array_operator, 0);
define_stub_fn!(DLL, nh_malloc, 0);
define_stub_fn!(DLL, nh_malloc_dbg, 0);
define_stub_fn!(DLL, heap_alloc, 0);
define_stub_fn!(DLL, heap_realloc, 0);
define_stub_fn!(DLL, heap_free, 0);
define_stub_fn!(DLL, locked_malloc, 0);
define_stub_fn!(DLL, locked_free, 0);
define_stub_fn!(DLL, purecall, 0);
define_stub_fn!(DLL, onexit, 0);
define_stub_fn!(DLL, initterm, 0);
define_stub_fn!(DLL, initterm_e, 0);

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
