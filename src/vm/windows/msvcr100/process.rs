//! Process/thread function stubs for MSVCR100.dll.

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

// Process functions
stub!(getpid_impl);
stub!(execl_impl);
stub!(execle_impl);
stub!(execlp_impl);
stub!(execlpe_impl);
stub!(execv_impl);
stub!(execve_impl);
stub!(execvp_impl);
stub!(execvpe_impl);
stub!(wexecl_impl);
stub!(wexecle_impl);
stub!(wexeclp_impl);
stub!(wexeclpe_impl);
stub!(wexecv_impl);
stub!(wexecve_impl);
stub!(wexecvp_impl);
stub!(wexecvpe_impl);
stub!(spawnl_impl);
stub!(spawnle_impl);
stub!(spawnlp_impl);
stub!(spawnlpe_impl);
stub!(spawnv_impl);
stub!(spawnve_impl);
stub!(spawnvp_impl);
stub!(spawnvpe_impl);
stub!(wspawnl_impl);
stub!(wspawnle_impl);
stub!(wspawnlp_impl);
stub!(wspawnlpe_impl);
stub!(wspawnv_impl);
stub!(wspawnve_impl);
stub!(wspawnvp_impl);
stub!(wspawnvpe_impl);
stub!(cwait_impl);
stub!(popen_impl);
stub!(wpopen_impl);
stub!(pclose_impl);
stub!(loaddll_impl);
stub!(unloaddll_impl);
stub!(getdllprocaddr_impl);
stub!(beep_impl);
stub!(sleep_impl);

// Thread functions
stub!(beginthread_impl);
stub!(beginthreadex_impl);
stub!(endthread_impl);
stub!(endthreadex_impl);
stub!(threadhandle_impl);
stub!(threadid_impl);

// Global variables
stub!(argc);
stub!(argv);
stub!(wargv);
stub!(environ);
stub!(wenviron);
stub!(pgmptr);
stub!(wpgmptr);
stub!(acmdln);
stub!(wcmdln);
stub!(initenv);
stub!(winitenv);
stub!(commode);
stub!(fmode);
stub!(osplatform);
stub!(osver);
stub!(winver);
stub!(winmajor);
stub!(winminor);
stub!(fls_getvalue);
stub!(fls_setvalue);
stub!(set_app_type);
stub!(set_flsgetvalue);
stub!(get_flsindex);
stub!(get_tlsindex);
stub!(getmainargs);
stub!(wgetmainargs);
stub!(configure_narrow_argv);
stub!(configure_wide_argv);
stub!(crt_rtc_init);
stub!(crt_rtc_initw);
stub!(report_gsfailure);
stub!(get_osplatform);
stub!(get_osver);
stub!(get_winver);
stub!(get_winmajor);
stub!(get_winminor);
stub!(unDName);
stub!(unDNameEx);
stub!(unDNameHelper);

pub fn register(vm: &mut Vm) {
    // Process functions
    vm.register_import(DLL, "_getpid", getpid_impl);
    vm.register_import(DLL, "_execl", execl_impl);
    vm.register_import(DLL, "_execle", execle_impl);
    vm.register_import(DLL, "_execlp", execlp_impl);
    vm.register_import(DLL, "_execlpe", execlpe_impl);
    vm.register_import(DLL, "_execv", execv_impl);
    vm.register_import(DLL, "_execve", execve_impl);
    vm.register_import(DLL, "_execvp", execvp_impl);
    vm.register_import(DLL, "_execvpe", execvpe_impl);
    vm.register_import(DLL, "_wexecl", wexecl_impl);
    vm.register_import(DLL, "_wexecle", wexecle_impl);
    vm.register_import(DLL, "_wexeclp", wexeclp_impl);
    vm.register_import(DLL, "_wexeclpe", wexeclpe_impl);
    vm.register_import(DLL, "_wexecv", wexecv_impl);
    vm.register_import(DLL, "_wexecve", wexecve_impl);
    vm.register_import(DLL, "_wexecvp", wexecvp_impl);
    vm.register_import(DLL, "_wexecvpe", wexecvpe_impl);
    vm.register_import(DLL, "_spawnl", spawnl_impl);
    vm.register_import(DLL, "_spawnle", spawnle_impl);
    vm.register_import(DLL, "_spawnlp", spawnlp_impl);
    vm.register_import(DLL, "_spawnlpe", spawnlpe_impl);
    vm.register_import(DLL, "_spawnv", spawnv_impl);
    vm.register_import(DLL, "_spawnve", spawnve_impl);
    vm.register_import(DLL, "_spawnvp", spawnvp_impl);
    vm.register_import(DLL, "_spawnvpe", spawnvpe_impl);
    vm.register_import(DLL, "_wspawnl", wspawnl_impl);
    vm.register_import(DLL, "_wspawnle", wspawnle_impl);
    vm.register_import(DLL, "_wspawnlp", wspawnlp_impl);
    vm.register_import(DLL, "_wspawnlpe", wspawnlpe_impl);
    vm.register_import(DLL, "_wspawnv", wspawnv_impl);
    vm.register_import(DLL, "_wspawnve", wspawnve_impl);
    vm.register_import(DLL, "_wspawnvp", wspawnvp_impl);
    vm.register_import(DLL, "_wspawnvpe", wspawnvpe_impl);
    vm.register_import(DLL, "_cwait", cwait_impl);
    vm.register_import(DLL, "_popen", popen_impl);
    vm.register_import(DLL, "_wpopen", wpopen_impl);
    vm.register_import(DLL, "_pclose", pclose_impl);
    vm.register_import(DLL, "_loaddll", loaddll_impl);
    vm.register_import(DLL, "_unloaddll", unloaddll_impl);
    vm.register_import(DLL, "_getdllprocaddr", getdllprocaddr_impl);
    vm.register_import(DLL, "_beep", beep_impl);
    vm.register_import(DLL, "_sleep", sleep_impl);

    // Thread functions
    vm.register_import(DLL, "_beginthread", beginthread_impl);
    vm.register_import(DLL, "_beginthreadex", beginthreadex_impl);
    vm.register_import(DLL, "_endthread", endthread_impl);
    vm.register_import(DLL, "_endthreadex", endthreadex_impl);
    vm.register_import(DLL, "__threadhandle", threadhandle_impl);
    vm.register_import(DLL, "__threadid", threadid_impl);

    // Global variables
    vm.register_import(DLL, "__argc", argc);
    vm.register_import(DLL, "__argv", argv);
    vm.register_import(DLL, "__wargv", wargv);
    vm.register_import(DLL, "_environ", environ);
    vm.register_import(DLL, "_wenviron", wenviron);
    vm.register_import(DLL, "_pgmptr", pgmptr);
    vm.register_import(DLL, "_wpgmptr", wpgmptr);
    vm.register_import(DLL, "_acmdln", acmdln);
    vm.register_import(DLL, "_wcmdln", wcmdln);
    vm.register_import(DLL, "__initenv", initenv);
    vm.register_import(DLL, "__winitenv", winitenv);
    vm.register_import(DLL, "_commode", commode);
    vm.register_import(DLL, "_fmode", fmode);
    vm.register_import(DLL, "__p___argc", argc);
    vm.register_import(DLL, "__p___argv", argv);
    vm.register_import(DLL, "__p___wargv", wargv);
    vm.register_import(DLL, "__p__environ", environ);
    vm.register_import(DLL, "__p__wenviron", wenviron);
    vm.register_import(DLL, "__p__pgmptr", pgmptr);
    vm.register_import(DLL, "__p__wpgmptr", wpgmptr);
    vm.register_import(DLL, "__p__acmdln", acmdln);
    vm.register_import(DLL, "__p__wcmdln", wcmdln);
    vm.register_import(DLL, "__p___initenv", initenv);
    vm.register_import(DLL, "__p___winitenv", winitenv);
    vm.register_import(DLL, "__p__commode", commode);
    vm.register_import(DLL, "__p__fmode", fmode);
    vm.register_import(DLL, "__p___mb_cur_max", argc);
    vm.register_import(DLL, "___fls_getvalue@4", fls_getvalue);
    vm.register_import(DLL, "___fls_setvalue@8", fls_setvalue);
    vm.register_import(DLL, "__set_app_type", set_app_type);
    vm.register_import(DLL, "__set_flsgetvalue", set_flsgetvalue);
    vm.register_import(DLL, "__get_flsindex", get_flsindex);
    vm.register_import(DLL, "__get_tlsindex", get_tlsindex);
    vm.register_import(DLL, "__getmainargs", getmainargs);
    vm.register_import(DLL, "__wgetmainargs", wgetmainargs);
    vm.register_import(DLL, "_configure_narrow_argv", configure_narrow_argv);
    vm.register_import(DLL, "_configure_wide_argv", configure_wide_argv);
    vm.register_import(DLL, "_CRT_RTC_INIT", crt_rtc_init);
    vm.register_import(DLL, "_CRT_RTC_INITW", crt_rtc_initw);
    vm.register_import(DLL, "__report_gsfailure", report_gsfailure);
    vm.register_import(DLL, "_get_osplatform", get_osplatform);
    vm.register_import(DLL, "_get_osver", get_osver);
    vm.register_import(DLL, "_get_winver", get_winver);
    vm.register_import(DLL, "_get_winmajor", get_winmajor);
    vm.register_import(DLL, "_get_winminor", get_winminor);
    vm.register_import(DLL, "__unDName", unDName);
    vm.register_import(DLL, "__unDNameEx", unDNameEx);
    vm.register_import(DLL, "__unDNameHelper", unDNameHelper);
}
