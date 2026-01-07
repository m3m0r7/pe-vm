//! Standard library function stubs for MSVCR100.dll.

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

// Conversion functions
stub!(atoi_impl);
stub!(atoi_l_impl);
stub!(atol_impl);
stub!(atol_l_impl);
stub!(atof_impl);
stub!(atof_l_impl);
stub!(atoi64_impl);
stub!(atoi64_l_impl);
stub!(strtol_impl);
stub!(strtol_l_impl);
stub!(strtoul_impl);
stub!(strtoul_l_impl);
stub!(strtod_impl);
stub!(strtod_l_impl);
stub!(strtoll_impl);
stub!(strtoull_impl);
stub!(wtoi_impl);
stub!(wtoi_l_impl);
stub!(wtol_impl);
stub!(wtol_l_impl);
stub!(wtof_impl);
stub!(wtof_l_impl);
stub!(wtoi64_impl);
stub!(wtoi64_l_impl);
stub!(wcstol_impl);
stub!(wcstol_l_impl);
stub!(wcstoul_impl);
stub!(wcstoul_l_impl);
stub!(wcstod_impl);
stub!(wcstod_l_impl);
stub!(wcstoi64_impl);
stub!(wcstoi64_l_impl);
stub!(wcstoui64_impl);
stub!(wcstoui64_l_impl);
stub!(itoa_impl);
stub!(itoa_s_impl);
stub!(ltoa_impl);
stub!(ltoa_s_impl);
stub!(ultoa_impl);
stub!(ultoa_s_impl);
stub!(i64toa_impl);
stub!(i64toa_s_impl);
stub!(ui64toa_impl);
stub!(ui64toa_s_impl);
stub!(itow_impl);
stub!(itow_s_impl);
stub!(ltow_impl);
stub!(ltow_s_impl);
stub!(ultow_impl);
stub!(ultow_s_impl);
stub!(i64tow_impl);
stub!(i64tow_s_impl);
stub!(ui64tow_impl);
stub!(ui64tow_s_impl);
stub!(gcvt_impl);
stub!(gcvt_s_impl);
stub!(ecvt_impl);
stub!(ecvt_s_impl);
stub!(fcvt_impl);
stub!(fcvt_s_impl);
stub!(atodbl_impl);
stub!(atodbl_l_impl);
stub!(atoflt_impl);
stub!(atoflt_l_impl);
stub!(atoldbl_impl);
stub!(atoldbl_l_impl);
stub!(i10_output);
stub!(stringtold);
stub!(stringtold_l);

// Search and sort
stub!(bsearch_impl);
stub!(bsearch_s_impl);
stub!(qsort_impl);
stub!(qsort_s_impl);
stub!(lfind_impl);
stub!(lfind_s_impl);
stub!(lsearch_impl);
stub!(lsearch_s_impl);

// Random numbers
stub!(rand_impl);
stub!(rand_s_impl);
stub!(srand_impl);

// Environment
stub!(getenv_impl);
stub!(getenv_s_impl);
stub!(wgetenv_impl);
stub!(wgetenv_s_impl);
stub!(putenv_impl);
stub!(putenv_s_impl);
stub!(wputenv_impl);
stub!(wputenv_s_impl);
stub!(dupenv_s_impl);
stub!(wdupenv_s_impl);
stub!(searchenv_impl);
stub!(searchenv_s_impl);
stub!(wsearchenv_impl);
stub!(wsearchenv_s_impl);
stub!(makepath_impl);
stub!(makepath_s_impl);
stub!(wmakepath_impl);
stub!(wmakepath_s_impl);
stub!(splitpath_impl);
stub!(splitpath_s_impl);
stub!(wsplitpath_impl);
stub!(wsplitpath_s_impl);
stub!(fullpath_impl);
stub!(wfullpath_impl);

// Process control
stub!(abort_impl);
stub!(exit_impl);
stub!(quick_exit_impl);
stub!(at_quick_exit_impl);
stub!(cexit_impl);
stub!(c_exit_impl);
stub!(atexit_impl);
stub!(system_impl);
stub!(wsystem_impl);

// Byte swap
stub!(byteswap_ushort);
stub!(byteswap_ulong);
stub!(byteswap_uint64);
stub!(rotl);
stub!(rotr);
stub!(rotl64);
stub!(rotr64);
stub!(lrotl);
stub!(lrotr);

// Miscellaneous
stub!(swab_impl);
stub!(assert_impl);
stub!(wassert_impl);
stub!(amsg_exit);
stub!(invalid_parameter);
stub!(invalid_parameter_noinfo);
stub!(invalid_parameter_noinfo_noreturn);
stub!(invoke_watson);
stub!(errno_impl);
stub!(doserrno_impl);
stub!(set_errno_impl);
stub!(get_errno_impl);
stub!(set_doserrno_impl);
stub!(get_doserrno_impl);
stub!(strerror_impl);
stub!(sys_errlist);
stub!(sys_nerr);

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
