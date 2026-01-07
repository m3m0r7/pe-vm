//! CRT initialization and utility stubs for MSVCR100.dll.

use crate::vm::windows::check_stub;
use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";



// CRT initialization
define_stub_fn!(DLL, dllonexit_impl, 0);
define_stub_fn!(DLL, encode_pointer, 0);
define_stub_fn!(DLL, decode_pointer, 0);
define_stub_fn!(DLL, encoded_null, 0);
define_stub_fn!(DLL, crt_atexit, 0);
define_stub_fn!(DLL, crt_at_quick_exit, 0);
define_stub_fn!(DLL, get_sbh_threshold, 0);
define_stub_fn!(DLL, set_sbh_threshold, 0);
define_stub_fn!(DLL, seh_longjmp_unwind, 0);
define_stub_fn!(DLL, seh_longjmp_unwind4, 0);
define_stub_fn!(DLL, local_unwind2, 0);
define_stub_fn!(DLL, local_unwind4, 0);
define_stub_fn!(DLL, global_unwind2, 0);
define_stub_fn!(DLL, except_handler2, 0);
define_stub_fn!(DLL, except_handler3, 0);
define_stub_fn!(DLL, except_handler4, 0);
define_stub_fn!(DLL, security_error_handler, 0);
define_stub_fn!(DLL, security_init_cookie, 0);
define_stub_fn!(DLL, crt_dbg_report_v, 0);
define_stub_fn!(DLL, crt_dbg_report_wv, 0);
define_stub_fn!(DLL, pxcptinfoptrs, 0);
define_stub_fn!(DLL, signal_impl, 0);
define_stub_fn!(DLL, raise_impl, 0);

pub fn register(vm: &mut Vm) {
    // CRT initialization
    vm.register_import(DLL, "__dllonexit", dllonexit_impl);
    vm.register_import(DLL, "_encoded_null", encoded_null);
    vm.register_import(DLL, "_encode_pointer", encode_pointer);
    vm.register_import(DLL, "_decode_pointer", decode_pointer);
    vm.register_import(DLL, "_crt_atexit", crt_atexit);
    vm.register_import(DLL, "_crt_at_quick_exit", crt_at_quick_exit);
    vm.register_import(DLL, "_get_sbh_threshold", get_sbh_threshold);
    vm.register_import(DLL, "_set_sbh_threshold", set_sbh_threshold);
    vm.register_import(DLL, "_seh_longjmp_unwind", seh_longjmp_unwind);
    vm.register_import(DLL, "_seh_longjmp_unwind4", seh_longjmp_unwind4);
    vm.register_import(DLL, "_local_unwind2", local_unwind2);
    vm.register_import(DLL, "_local_unwind4", local_unwind4);
    vm.register_import(DLL, "_global_unwind2", global_unwind2);
    vm.register_import(DLL, "_except_handler2", except_handler2);
    vm.register_import(DLL, "_except_handler3", except_handler3);
    vm.register_import(DLL, "_except_handler4", except_handler4);
    vm.register_import(DLL, "__security_error_handler", security_error_handler);
    vm.register_import(DLL, "__security_init_cookie", security_init_cookie);
    vm.register_import(DLL, "_CrtDbgReportV", crt_dbg_report_v);
    vm.register_import(DLL, "_CrtDbgReportWV", crt_dbg_report_wv);
    vm.register_import(DLL, "__pxcptinfoptrs", pxcptinfoptrs);
    vm.register_import(DLL, "signal", signal_impl);
    vm.register_import(DLL, "raise", raise_impl);
}
