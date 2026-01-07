//! Time function stubs for MSVCR100.dll.

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

// Time functions
stub!(time32_impl);
stub!(time64_impl);
stub!(clock_impl);
stub!(difftime32_impl);
stub!(difftime64_impl);
stub!(mktime32_impl);
stub!(mktime64_impl);
stub!(gmtime32_impl);
stub!(gmtime32_s_impl);
stub!(gmtime64_impl);
stub!(gmtime64_s_impl);
stub!(localtime32_impl);
stub!(localtime32_s_impl);
stub!(localtime64_impl);
stub!(localtime64_s_impl);
stub!(asctime_impl);
stub!(asctime_s_impl);
stub!(wasctime_impl);
stub!(wasctime_s_impl);
stub!(ctime32_impl);
stub!(ctime32_s_impl);
stub!(ctime64_impl);
stub!(ctime64_s_impl);
stub!(wctime32_impl);
stub!(wctime32_s_impl);
stub!(wctime64_impl);
stub!(wctime64_s_impl);
stub!(strdate_impl);
stub!(strdate_s_impl);
stub!(wstrdate_impl);
stub!(wstrdate_s_impl);
stub!(strtime_impl);
stub!(strtime_s_impl);
stub!(wstrtime_impl);
stub!(wstrtime_s_impl);
stub!(tzset_impl);
stub!(daylight);
stub!(dstbias);
stub!(timezone);
stub!(tzname);
stub!(utime32_impl);
stub!(utime64_impl);
stub!(wutime32_impl);
stub!(wutime64_impl);
stub!(futime32_impl);
stub!(futime64_impl);
stub!(ftime32_impl);
stub!(ftime32_s_impl);
stub!(ftime64_impl);
stub!(ftime64_s_impl);
stub!(get_daylight);
stub!(get_dstbias);
stub!(get_timezone);
stub!(get_tzname);

pub fn register(vm: &mut Vm) {
    // Time functions
    vm.register_import(DLL, "_time32", time32_impl);
    vm.register_import(DLL, "_time64", time64_impl);
    vm.register_import(DLL, "clock", clock_impl);
    vm.register_import(DLL, "_difftime32", difftime32_impl);
    vm.register_import(DLL, "_difftime64", difftime64_impl);
    vm.register_import(DLL, "_mktime32", mktime32_impl);
    vm.register_import(DLL, "_mktime64", mktime64_impl);
    vm.register_import(DLL, "_gmtime32", gmtime32_impl);
    vm.register_import(DLL, "_gmtime32_s", gmtime32_s_impl);
    vm.register_import(DLL, "_gmtime64", gmtime64_impl);
    vm.register_import(DLL, "_gmtime64_s", gmtime64_s_impl);
    vm.register_import(DLL, "_localtime32", localtime32_impl);
    vm.register_import(DLL, "_localtime32_s", localtime32_s_impl);
    vm.register_import(DLL, "_localtime64", localtime64_impl);
    vm.register_import(DLL, "_localtime64_s", localtime64_s_impl);
    vm.register_import(DLL, "asctime", asctime_impl);
    vm.register_import(DLL, "asctime_s", asctime_s_impl);
    vm.register_import(DLL, "_wasctime", wasctime_impl);
    vm.register_import(DLL, "_wasctime_s", wasctime_s_impl);
    vm.register_import(DLL, "_ctime32", ctime32_impl);
    vm.register_import(DLL, "_ctime32_s", ctime32_s_impl);
    vm.register_import(DLL, "_ctime64", ctime64_impl);
    vm.register_import(DLL, "_ctime64_s", ctime64_s_impl);
    vm.register_import(DLL, "_wctime32", wctime32_impl);
    vm.register_import(DLL, "_wctime32_s", wctime32_s_impl);
    vm.register_import(DLL, "_wctime64", wctime64_impl);
    vm.register_import(DLL, "_wctime64_s", wctime64_s_impl);
    vm.register_import(DLL, "_strdate", strdate_impl);
    vm.register_import(DLL, "_strdate_s", strdate_s_impl);
    vm.register_import(DLL, "_wstrdate", wstrdate_impl);
    vm.register_import(DLL, "_wstrdate_s", wstrdate_s_impl);
    vm.register_import(DLL, "_strtime", strtime_impl);
    vm.register_import(DLL, "_strtime_s", strtime_s_impl);
    vm.register_import(DLL, "_wstrtime", wstrtime_impl);
    vm.register_import(DLL, "_wstrtime_s", wstrtime_s_impl);
    vm.register_import(DLL, "_tzset", tzset_impl);
    vm.register_import(DLL, "__daylight", daylight);
    vm.register_import(DLL, "__dstbias", dstbias);
    vm.register_import(DLL, "__timezone", timezone);
    vm.register_import(DLL, "__tzname", tzname);
    vm.register_import(DLL, "__p__daylight", daylight);
    vm.register_import(DLL, "__p__dstbias", dstbias);
    vm.register_import(DLL, "__p__timezone", timezone);
    vm.register_import(DLL, "__p__tzname", tzname);
    vm.register_import(DLL, "_utime32", utime32_impl);
    vm.register_import(DLL, "_utime64", utime64_impl);
    vm.register_import(DLL, "_wutime32", wutime32_impl);
    vm.register_import(DLL, "_wutime64", wutime64_impl);
    vm.register_import(DLL, "_futime32", futime32_impl);
    vm.register_import(DLL, "_futime64", futime64_impl);
    vm.register_import(DLL, "_ftime32", ftime32_impl);
    vm.register_import(DLL, "_ftime32_s", ftime32_s_impl);
    vm.register_import(DLL, "_ftime64", ftime64_impl);
    vm.register_import(DLL, "_ftime64_s", ftime64_s_impl);
    vm.register_import(DLL, "_get_daylight", get_daylight);
    vm.register_import(DLL, "_get_dstbias", get_dstbias);
    vm.register_import(DLL, "_get_timezone", get_timezone);
    vm.register_import(DLL, "_get_tzname", get_tzname);
}
