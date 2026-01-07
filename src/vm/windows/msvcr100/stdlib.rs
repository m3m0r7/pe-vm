//! Standard library function stubs for MSVCR100.dll.

use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";



// Conversion functions
define_stub_fn!(DLL, atoi_impl, 0);
define_stub_fn!(DLL, atoi_l_impl, 0);
define_stub_fn!(DLL, atol_impl, 0);
define_stub_fn!(DLL, atol_l_impl, 0);
define_stub_fn!(DLL, atof_impl, 0);
define_stub_fn!(DLL, atof_l_impl, 0);
define_stub_fn!(DLL, atoi64_impl, 0);
define_stub_fn!(DLL, atoi64_l_impl, 0);
define_stub_fn!(DLL, strtol_impl, 0);
define_stub_fn!(DLL, strtol_l_impl, 0);
define_stub_fn!(DLL, strtoul_impl, 0);
define_stub_fn!(DLL, strtoul_l_impl, 0);
define_stub_fn!(DLL, strtod_impl, 0);
define_stub_fn!(DLL, strtod_l_impl, 0);
define_stub_fn!(DLL, strtoll_impl, 0);
define_stub_fn!(DLL, strtoull_impl, 0);
define_stub_fn!(DLL, wtoi_impl, 0);
define_stub_fn!(DLL, wtoi_l_impl, 0);
define_stub_fn!(DLL, wtol_impl, 0);
define_stub_fn!(DLL, wtol_l_impl, 0);
define_stub_fn!(DLL, wtof_impl, 0);
define_stub_fn!(DLL, wtof_l_impl, 0);
define_stub_fn!(DLL, wtoi64_impl, 0);
define_stub_fn!(DLL, wtoi64_l_impl, 0);
define_stub_fn!(DLL, wcstol_impl, 0);
define_stub_fn!(DLL, wcstol_l_impl, 0);
define_stub_fn!(DLL, wcstoul_impl, 0);
define_stub_fn!(DLL, wcstoul_l_impl, 0);
define_stub_fn!(DLL, wcstod_impl, 0);
define_stub_fn!(DLL, wcstod_l_impl, 0);
define_stub_fn!(DLL, wcstoi64_impl, 0);
define_stub_fn!(DLL, wcstoi64_l_impl, 0);
define_stub_fn!(DLL, wcstoui64_impl, 0);
define_stub_fn!(DLL, wcstoui64_l_impl, 0);
define_stub_fn!(DLL, itoa_impl, 0);
define_stub_fn!(DLL, itoa_s_impl, 0);
define_stub_fn!(DLL, ltoa_impl, 0);
define_stub_fn!(DLL, ltoa_s_impl, 0);
define_stub_fn!(DLL, ultoa_impl, 0);
define_stub_fn!(DLL, ultoa_s_impl, 0);
define_stub_fn!(DLL, i64toa_impl, 0);
define_stub_fn!(DLL, i64toa_s_impl, 0);
define_stub_fn!(DLL, ui64toa_impl, 0);
define_stub_fn!(DLL, ui64toa_s_impl, 0);
define_stub_fn!(DLL, itow_impl, 0);
define_stub_fn!(DLL, itow_s_impl, 0);
define_stub_fn!(DLL, ltow_impl, 0);
define_stub_fn!(DLL, ltow_s_impl, 0);
define_stub_fn!(DLL, ultow_impl, 0);
define_stub_fn!(DLL, ultow_s_impl, 0);
define_stub_fn!(DLL, i64tow_impl, 0);
define_stub_fn!(DLL, i64tow_s_impl, 0);
define_stub_fn!(DLL, ui64tow_impl, 0);
define_stub_fn!(DLL, ui64tow_s_impl, 0);
define_stub_fn!(DLL, gcvt_impl, 0);
define_stub_fn!(DLL, gcvt_s_impl, 0);
define_stub_fn!(DLL, ecvt_impl, 0);
define_stub_fn!(DLL, ecvt_s_impl, 0);
define_stub_fn!(DLL, fcvt_impl, 0);
define_stub_fn!(DLL, fcvt_s_impl, 0);
define_stub_fn!(DLL, atodbl_impl, 0);
define_stub_fn!(DLL, atodbl_l_impl, 0);
define_stub_fn!(DLL, atoflt_impl, 0);
define_stub_fn!(DLL, atoflt_l_impl, 0);
define_stub_fn!(DLL, atoldbl_impl, 0);
define_stub_fn!(DLL, atoldbl_l_impl, 0);
define_stub_fn!(DLL, i10_output, 0);
define_stub_fn!(DLL, stringtold, 0);
define_stub_fn!(DLL, stringtold_l, 0);

// Search and sort
define_stub_fn!(DLL, bsearch_impl, 0);
define_stub_fn!(DLL, bsearch_s_impl, 0);
define_stub_fn!(DLL, qsort_impl, 0);
define_stub_fn!(DLL, qsort_s_impl, 0);
define_stub_fn!(DLL, lfind_impl, 0);
define_stub_fn!(DLL, lfind_s_impl, 0);
define_stub_fn!(DLL, lsearch_impl, 0);
define_stub_fn!(DLL, lsearch_s_impl, 0);

// Random numbers
define_stub_fn!(DLL, rand_impl, 0);
define_stub_fn!(DLL, rand_s_impl, 0);
define_stub_fn!(DLL, srand_impl, 0);

// Environment
define_stub_fn!(DLL, getenv_impl, 0);
define_stub_fn!(DLL, getenv_s_impl, 0);
define_stub_fn!(DLL, wgetenv_impl, 0);
define_stub_fn!(DLL, wgetenv_s_impl, 0);
define_stub_fn!(DLL, putenv_impl, 0);
define_stub_fn!(DLL, putenv_s_impl, 0);
define_stub_fn!(DLL, wputenv_impl, 0);
define_stub_fn!(DLL, wputenv_s_impl, 0);
define_stub_fn!(DLL, dupenv_s_impl, 0);
define_stub_fn!(DLL, wdupenv_s_impl, 0);
define_stub_fn!(DLL, searchenv_impl, 0);
define_stub_fn!(DLL, searchenv_s_impl, 0);
define_stub_fn!(DLL, wsearchenv_impl, 0);
define_stub_fn!(DLL, wsearchenv_s_impl, 0);
define_stub_fn!(DLL, makepath_impl, 0);
define_stub_fn!(DLL, makepath_s_impl, 0);
define_stub_fn!(DLL, wmakepath_impl, 0);
define_stub_fn!(DLL, wmakepath_s_impl, 0);
define_stub_fn!(DLL, splitpath_impl, 0);
define_stub_fn!(DLL, splitpath_s_impl, 0);
define_stub_fn!(DLL, wsplitpath_impl, 0);
define_stub_fn!(DLL, wsplitpath_s_impl, 0);
define_stub_fn!(DLL, fullpath_impl, 0);
define_stub_fn!(DLL, wfullpath_impl, 0);

// Process control
define_stub_fn!(DLL, abort_impl, 0);
define_stub_fn!(DLL, exit_impl, 0);
define_stub_fn!(DLL, quick_exit_impl, 0);
define_stub_fn!(DLL, at_quick_exit_impl, 0);
define_stub_fn!(DLL, cexit_impl, 0);
define_stub_fn!(DLL, c_exit_impl, 0);
define_stub_fn!(DLL, atexit_impl, 0);
define_stub_fn!(DLL, system_impl, 0);
define_stub_fn!(DLL, wsystem_impl, 0);

// Byte swap
define_stub_fn!(DLL, byteswap_ushort, 0);
define_stub_fn!(DLL, byteswap_ulong, 0);
define_stub_fn!(DLL, byteswap_uint64, 0);
define_stub_fn!(DLL, rotl, 0);
define_stub_fn!(DLL, rotr, 0);
define_stub_fn!(DLL, rotl64, 0);
define_stub_fn!(DLL, rotr64, 0);
define_stub_fn!(DLL, lrotl, 0);
define_stub_fn!(DLL, lrotr, 0);

// Miscellaneous
define_stub_fn!(DLL, swab_impl, 0);
define_stub_fn!(DLL, assert_impl, 0);
define_stub_fn!(DLL, wassert_impl, 0);
define_stub_fn!(DLL, amsg_exit, 0);
define_stub_fn!(DLL, invalid_parameter, 0);
define_stub_fn!(DLL, invalid_parameter_noinfo, 0);
define_stub_fn!(DLL, invalid_parameter_noinfo_noreturn, 0);
define_stub_fn!(DLL, invoke_watson, 0);
define_stub_fn!(DLL, errno_impl, 0);
define_stub_fn!(DLL, doserrno_impl, 0);
define_stub_fn!(DLL, set_errno_impl, 0);
define_stub_fn!(DLL, get_errno_impl, 0);
define_stub_fn!(DLL, set_doserrno_impl, 0);
define_stub_fn!(DLL, get_doserrno_impl, 0);
define_stub_fn!(DLL, strerror_impl, 0);
define_stub_fn!(DLL, sys_errlist, 0);
define_stub_fn!(DLL, sys_nerr, 0);

pub fn register(vm: &mut Vm) {
    // Integer conversion
    vm.register_import(DLL, "atoi", atoi_impl);
    vm.register_import(DLL, "_atoi_l", atoi_l_impl);
    vm.register_import(DLL, "atol", atol_impl);
    vm.register_import(DLL, "_atol_l", atol_l_impl);
    vm.register_import(DLL, "atof", atof_impl);
    vm.register_import(DLL, "_atof_l", atof_l_impl);
    vm.register_import(DLL, "_atoi64", atoi64_impl);
    vm.register_import(DLL, "_atoi64_l", atoi64_l_impl);
    vm.register_import(DLL, "strtol", strtol_impl);
    vm.register_import(DLL, "_strtol_l", strtol_l_impl);
    vm.register_import(DLL, "strtoul", strtoul_impl);
    vm.register_import(DLL, "_strtoul_l", strtoul_l_impl);
    vm.register_import(DLL, "strtod", strtod_impl);
    vm.register_import(DLL, "_strtod_l", strtod_l_impl);
    vm.register_import(DLL, "_strtoi64", strtoll_impl);
    vm.register_import(DLL, "_strtoi64_l", strtoll_impl);
    vm.register_import(DLL, "_strtoui64", strtoull_impl);
    vm.register_import(DLL, "_strtoui64_l", strtoull_impl);

    // Wide integer conversion
    vm.register_import(DLL, "_wtoi", wtoi_impl);
    vm.register_import(DLL, "_wtoi_l", wtoi_l_impl);
    vm.register_import(DLL, "_wtol", wtol_impl);
    vm.register_import(DLL, "_wtol_l", wtol_l_impl);
    vm.register_import(DLL, "_wtof", wtof_impl);
    vm.register_import(DLL, "_wtof_l", wtof_l_impl);
    vm.register_import(DLL, "_wtoi64", wtoi64_impl);
    vm.register_import(DLL, "_wtoi64_l", wtoi64_l_impl);
    vm.register_import(DLL, "wcstol", wcstol_impl);
    vm.register_import(DLL, "_wcstol_l", wcstol_l_impl);
    vm.register_import(DLL, "wcstoul", wcstoul_impl);
    vm.register_import(DLL, "_wcstoul_l", wcstoul_l_impl);
    vm.register_import(DLL, "wcstod", wcstod_impl);
    vm.register_import(DLL, "_wcstod_l", wcstod_l_impl);
    vm.register_import(DLL, "_wcstoi64", wcstoi64_impl);
    vm.register_import(DLL, "_wcstoi64_l", wcstoi64_l_impl);
    vm.register_import(DLL, "_wcstoui64", wcstoui64_impl);
    vm.register_import(DLL, "_wcstoui64_l", wcstoui64_l_impl);

    // Integer to string
    vm.register_import(DLL, "_itoa", itoa_impl);
    vm.register_import(DLL, "_itoa_s", itoa_s_impl);
    vm.register_import(DLL, "_ltoa", ltoa_impl);
    vm.register_import(DLL, "_ltoa_s", ltoa_s_impl);
    vm.register_import(DLL, "_ultoa", ultoa_impl);
    vm.register_import(DLL, "_ultoa_s", ultoa_s_impl);
    vm.register_import(DLL, "_i64toa", i64toa_impl);
    vm.register_import(DLL, "_i64toa_s", i64toa_s_impl);
    vm.register_import(DLL, "_ui64toa", ui64toa_impl);
    vm.register_import(DLL, "_ui64toa_s", ui64toa_s_impl);
    vm.register_import(DLL, "_itow", itow_impl);
    vm.register_import(DLL, "_itow_s", itow_s_impl);
    vm.register_import(DLL, "_ltow", ltow_impl);
    vm.register_import(DLL, "_ltow_s", ltow_s_impl);
    vm.register_import(DLL, "_ultow", ultow_impl);
    vm.register_import(DLL, "_ultow_s", ultow_s_impl);
    vm.register_import(DLL, "_i64tow", i64tow_impl);
    vm.register_import(DLL, "_i64tow_s", i64tow_s_impl);
    vm.register_import(DLL, "_ui64tow", ui64tow_impl);
    vm.register_import(DLL, "_ui64tow_s", ui64tow_s_impl);

    // Floating point to string
    vm.register_import(DLL, "_gcvt", gcvt_impl);
    vm.register_import(DLL, "_gcvt_s", gcvt_s_impl);
    vm.register_import(DLL, "_ecvt", ecvt_impl);
    vm.register_import(DLL, "_ecvt_s", ecvt_s_impl);
    vm.register_import(DLL, "_fcvt", fcvt_impl);
    vm.register_import(DLL, "_fcvt_s", fcvt_s_impl);
    vm.register_import(DLL, "_atodbl", atodbl_impl);
    vm.register_import(DLL, "_atodbl_l", atodbl_l_impl);
    vm.register_import(DLL, "_atoflt", atoflt_impl);
    vm.register_import(DLL, "_atoflt_l", atoflt_l_impl);
    vm.register_import(DLL, "_atoldbl", atoldbl_impl);
    vm.register_import(DLL, "_atoldbl_l", atoldbl_l_impl);
    vm.register_import(DLL, "$I10_OUTPUT", i10_output);
    vm.register_import(DLL, "__STRINGTOLD", stringtold);
    vm.register_import(DLL, "__STRINGTOLD_L", stringtold_l);

    // Search and sort
    vm.register_import(DLL, "bsearch", bsearch_impl);
    vm.register_import(DLL, "bsearch_s", bsearch_s_impl);
    vm.register_import(DLL, "qsort", qsort_impl);
    vm.register_import(DLL, "qsort_s", qsort_s_impl);
    vm.register_import(DLL, "_lfind", lfind_impl);
    vm.register_import(DLL, "_lfind_s", lfind_s_impl);
    vm.register_import(DLL, "_lsearch", lsearch_impl);
    vm.register_import(DLL, "_lsearch_s", lsearch_s_impl);

    // Random numbers
    vm.register_import(DLL, "rand", rand_impl);
    vm.register_import(DLL, "rand_s", rand_s_impl);
    vm.register_import(DLL, "srand", srand_impl);

    // Environment
    vm.register_import(DLL, "getenv", getenv_impl);
    vm.register_import(DLL, "getenv_s", getenv_s_impl);
    vm.register_import(DLL, "_wgetenv", wgetenv_impl);
    vm.register_import(DLL, "_wgetenv_s", wgetenv_s_impl);
    vm.register_import(DLL, "_putenv", putenv_impl);
    vm.register_import(DLL, "_putenv_s", putenv_s_impl);
    vm.register_import(DLL, "_wputenv", wputenv_impl);
    vm.register_import(DLL, "_wputenv_s", wputenv_s_impl);
    vm.register_import(DLL, "_dupenv_s", dupenv_s_impl);
    vm.register_import(DLL, "_wdupenv_s", wdupenv_s_impl);
    vm.register_import(DLL, "_searchenv", searchenv_impl);
    vm.register_import(DLL, "_searchenv_s", searchenv_s_impl);
    vm.register_import(DLL, "_wsearchenv", wsearchenv_impl);
    vm.register_import(DLL, "_wsearchenv_s", wsearchenv_s_impl);
    vm.register_import(DLL, "_makepath", makepath_impl);
    vm.register_import(DLL, "_makepath_s", makepath_s_impl);
    vm.register_import(DLL, "_wmakepath", wmakepath_impl);
    vm.register_import(DLL, "_wmakepath_s", wmakepath_s_impl);
    vm.register_import(DLL, "_splitpath", splitpath_impl);
    vm.register_import(DLL, "_splitpath_s", splitpath_s_impl);
    vm.register_import(DLL, "_wsplitpath", wsplitpath_impl);
    vm.register_import(DLL, "_wsplitpath_s", wsplitpath_s_impl);
    vm.register_import(DLL, "_fullpath", fullpath_impl);
    vm.register_import(DLL, "_wfullpath", wfullpath_impl);

    // Process control
    vm.register_import(DLL, "abort", abort_impl);
    vm.register_import(DLL, "exit", exit_impl);
    vm.register_import(DLL, "_exit", exit_impl);
    vm.register_import(DLL, "_Exit", exit_impl);
    vm.register_import(DLL, "quick_exit", quick_exit_impl);
    vm.register_import(DLL, "at_quick_exit", at_quick_exit_impl);
    vm.register_import(DLL, "_cexit", cexit_impl);
    vm.register_import(DLL, "_c_exit", c_exit_impl);
    vm.register_import(DLL, "atexit", atexit_impl);
    vm.register_import(DLL, "system", system_impl);
    vm.register_import(DLL, "_wsystem", wsystem_impl);

    // Byte swap / rotate
    vm.register_import(DLL, "_byteswap_ushort", byteswap_ushort);
    vm.register_import(DLL, "_byteswap_ulong", byteswap_ulong);
    vm.register_import(DLL, "_byteswap_uint64", byteswap_uint64);
    vm.register_import(DLL, "_rotl", rotl);
    vm.register_import(DLL, "_rotr", rotr);
    vm.register_import(DLL, "_rotl64", rotl64);
    vm.register_import(DLL, "_rotr64", rotr64);
    vm.register_import(DLL, "_lrotl", lrotl);
    vm.register_import(DLL, "_lrotr", lrotr);

    // Miscellaneous
    vm.register_import(DLL, "_swab", swab_impl);
    vm.register_import(DLL, "_assert", assert_impl);
    vm.register_import(DLL, "_wassert", wassert_impl);
    vm.register_import(DLL, "_amsg_exit", amsg_exit);
    vm.register_import(DLL, "_invalid_parameter", invalid_parameter);
    vm.register_import(DLL, "_invalid_parameter_noinfo", invalid_parameter_noinfo);
    vm.register_import(DLL, "_invalid_parameter_noinfo_noreturn", invalid_parameter_noinfo_noreturn);
    vm.register_import(DLL, "_invoke_watson", invoke_watson);
    vm.register_import(DLL, "_errno", errno_impl);
    vm.register_import(DLL, "__doserrno", doserrno_impl);
    vm.register_import(DLL, "_set_errno", set_errno_impl);
    vm.register_import(DLL, "_get_errno", get_errno_impl);
    vm.register_import(DLL, "_set_doserrno", set_doserrno_impl);
    vm.register_import(DLL, "_get_doserrno", get_doserrno_impl);
    vm.register_import(DLL, "__sys_errlist", sys_errlist);
    vm.register_import(DLL, "__sys_nerr", sys_nerr);
}
