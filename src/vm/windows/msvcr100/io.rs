//! Low-level I/O function stubs for MSVCR100.dll.

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

// Low-level file I/O
stub!(open_impl);
stub!(wopen_impl);
stub!(sopen_impl);
stub!(sopen_s_impl);
stub!(wsopen_impl);
stub!(wsopen_s_impl);
stub!(creat_impl);
stub!(wcreat_impl);
stub!(close_impl);
stub!(read_impl);
stub!(write_impl);
stub!(lseek_impl);
stub!(lseeki64_impl);
stub!(tell_impl);
stub!(telli64_impl);
stub!(dup_impl);
stub!(dup2_impl);
stub!(eof_impl);
stub!(commit_impl);
stub!(chsize_impl);
stub!(chsize_s_impl);
stub!(filelength_impl);
stub!(filelengthi64_impl);
stub!(locking_impl);
stub!(isatty_impl);
stub!(setmode_impl);
stub!(umask_impl);
stub!(umask_s_impl);
stub!(unlink_impl);
stub!(wunlink_impl);
stub!(access_impl);
stub!(access_s_impl);
stub!(waccess_impl);
stub!(waccess_s_impl);
stub!(chmod_impl);
stub!(wchmod_impl);
stub!(mktemp_impl);
stub!(mktemp_s_impl);
stub!(wmktemp_impl);
stub!(wmktemp_s_impl);
stub!(pipe_impl);
stub!(getcwd_impl);
stub!(wgetcwd_impl);
stub!(getdcwd_impl);
stub!(wgetdcwd_impl);
stub!(getdcwd_nolock_impl);
stub!(wgetdcwd_nolock_impl);
stub!(chdir_impl);
stub!(wchdir_impl);
stub!(mkdir_impl);
stub!(wmkdir_impl);
stub!(rmdir_impl);
stub!(wrmdir_impl);
stub!(getdrive_impl);
stub!(chdrive_impl);

// File find
stub!(findfirst32_impl);
stub!(findfirst32i64_impl);
stub!(findfirst64_impl);
stub!(findfirst64i32_impl);
stub!(findnext32_impl);
stub!(findnext32i64_impl);
stub!(findnext64_impl);
stub!(findnext64i32_impl);
stub!(findclose_impl);
stub!(wfindfirst32_impl);
stub!(wfindfirst32i64_impl);
stub!(wfindfirst64_impl);
stub!(wfindfirst64i32_impl);
stub!(wfindnext32_impl);
stub!(wfindnext32i64_impl);
stub!(wfindnext64_impl);
stub!(wfindnext64i32_impl);

// File stat
stub!(stat32_impl);
stub!(stat32i64_impl);
stub!(stat64_impl);
stub!(stat64i32_impl);
stub!(wstat32_impl);
stub!(wstat32i64_impl);
stub!(wstat64_impl);
stub!(wstat64i32_impl);
stub!(fstat32_impl);
stub!(fstat32i64_impl);
stub!(fstat64_impl);
stub!(fstat64i32_impl);

// Pioinfo
stub!(pioinfo);
stub!(badioinfo);

pub fn register(vm: &mut Vm) {
    // Low-level file I/O
    vm.register_import(DLL, "_open", open_impl);
    vm.register_import(DLL, "_wopen", wopen_impl);
    vm.register_import(DLL, "_sopen", sopen_impl);
    vm.register_import(DLL, "_sopen_s", sopen_s_impl);
    vm.register_import(DLL, "_wsopen", wsopen_impl);
    vm.register_import(DLL, "_wsopen_s", wsopen_s_impl);
    vm.register_import(DLL, "_creat", creat_impl);
    vm.register_import(DLL, "_wcreat", wcreat_impl);
    vm.register_import(DLL, "_close", close_impl);
    vm.register_import(DLL, "_read", read_impl);
    vm.register_import(DLL, "_write", write_impl);
    vm.register_import(DLL, "_lseek", lseek_impl);
    vm.register_import(DLL, "_lseeki64", lseeki64_impl);
    vm.register_import(DLL, "_tell", tell_impl);
    vm.register_import(DLL, "_telli64", telli64_impl);
    vm.register_import(DLL, "_dup", dup_impl);
    vm.register_import(DLL, "_dup2", dup2_impl);
    vm.register_import(DLL, "_eof", eof_impl);
    vm.register_import(DLL, "_commit", commit_impl);
    vm.register_import(DLL, "_chsize", chsize_impl);
    vm.register_import(DLL, "_chsize_s", chsize_s_impl);
    vm.register_import(DLL, "_filelength", filelength_impl);
    vm.register_import(DLL, "_filelengthi64", filelengthi64_impl);
    vm.register_import(DLL, "_locking", locking_impl);
    vm.register_import(DLL, "_isatty", isatty_impl);
    vm.register_import(DLL, "_setmode", setmode_impl);
    vm.register_import(DLL, "_umask", umask_impl);
    vm.register_import(DLL, "_umask_s", umask_s_impl);
    vm.register_import(DLL, "_unlink", unlink_impl);
    vm.register_import(DLL, "_wunlink", wunlink_impl);
    vm.register_import(DLL, "_access", access_impl);
    vm.register_import(DLL, "_access_s", access_s_impl);
    vm.register_import(DLL, "_waccess", waccess_impl);
    vm.register_import(DLL, "_waccess_s", waccess_s_impl);
    vm.register_import(DLL, "_chmod", chmod_impl);
    vm.register_import(DLL, "_wchmod", wchmod_impl);
    vm.register_import(DLL, "_mktemp", mktemp_impl);
    vm.register_import(DLL, "_mktemp_s", mktemp_s_impl);
    vm.register_import(DLL, "_wmktemp", wmktemp_impl);
    vm.register_import(DLL, "_wmktemp_s", wmktemp_s_impl);
    vm.register_import(DLL, "_pipe", pipe_impl);
    vm.register_import(DLL, "_getcwd", getcwd_impl);
    vm.register_import(DLL, "_wgetcwd", wgetcwd_impl);
    vm.register_import(DLL, "_getdcwd", getdcwd_impl);
    vm.register_import(DLL, "_wgetdcwd", wgetdcwd_impl);
    vm.register_import(DLL, "_getdcwd_nolock", getdcwd_nolock_impl);
    vm.register_import(DLL, "_wgetdcwd_nolock", wgetdcwd_nolock_impl);
    vm.register_import(DLL, "_chdir", chdir_impl);
    vm.register_import(DLL, "_wchdir", wchdir_impl);
    vm.register_import(DLL, "_mkdir", mkdir_impl);
    vm.register_import(DLL, "_wmkdir", wmkdir_impl);
    vm.register_import(DLL, "_rmdir", rmdir_impl);
    vm.register_import(DLL, "_wrmdir", wrmdir_impl);
    vm.register_import(DLL, "_getdrive", getdrive_impl);
    vm.register_import(DLL, "_chdrive", chdrive_impl);

    // File find
    vm.register_import(DLL, "_findfirst32", findfirst32_impl);
    vm.register_import(DLL, "_findfirst32i64", findfirst32i64_impl);
    vm.register_import(DLL, "_findfirst64", findfirst64_impl);
    vm.register_import(DLL, "_findfirst64i32", findfirst64i32_impl);
    vm.register_import(DLL, "_findnext32", findnext32_impl);
    vm.register_import(DLL, "_findnext32i64", findnext32i64_impl);
    vm.register_import(DLL, "_findnext64", findnext64_impl);
    vm.register_import(DLL, "_findnext64i32", findnext64i32_impl);
    vm.register_import(DLL, "_findclose", findclose_impl);
    vm.register_import(DLL, "_wfindfirst32", wfindfirst32_impl);
    vm.register_import(DLL, "_wfindfirst32i64", wfindfirst32i64_impl);
    vm.register_import(DLL, "_wfindfirst64", wfindfirst64_impl);
    vm.register_import(DLL, "_wfindfirst64i32", wfindfirst64i32_impl);
    vm.register_import(DLL, "_wfindnext32", wfindnext32_impl);
    vm.register_import(DLL, "_wfindnext32i64", wfindnext32i64_impl);
    vm.register_import(DLL, "_wfindnext64", wfindnext64_impl);
    vm.register_import(DLL, "_wfindnext64i32", wfindnext64i32_impl);

    // File stat
    vm.register_import(DLL, "_stat32", stat32_impl);
    vm.register_import(DLL, "_stat32i64", stat32i64_impl);
    vm.register_import(DLL, "_stat64", stat64_impl);
    vm.register_import(DLL, "_stat64i32", stat64i32_impl);
    vm.register_import(DLL, "_wstat32", wstat32_impl);
    vm.register_import(DLL, "_wstat32i64", wstat32i64_impl);
    vm.register_import(DLL, "_wstat64", wstat64_impl);
    vm.register_import(DLL, "_wstat64i32", wstat64i32_impl);
    vm.register_import(DLL, "_fstat32", fstat32_impl);
    vm.register_import(DLL, "_fstat32i64", fstat32i64_impl);
    vm.register_import(DLL, "_fstat64", fstat64_impl);
    vm.register_import(DLL, "_fstat64i32", fstat64i32_impl);

    // Internal pioinfo
    vm.register_import(DLL, "__pioinfo", pioinfo);
    vm.register_import(DLL, "__badioinfo", badioinfo);
}
