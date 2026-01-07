//! Low-level I/O function stubs for MSVCR100.dll.

use crate::vm::Vm;

const DLL: &str = "MSVCR100.dll";



// Low-level file I/O
define_stub_fn!(DLL, open_impl, 0);
define_stub_fn!(DLL, wopen_impl, 0);
define_stub_fn!(DLL, sopen_impl, 0);
define_stub_fn!(DLL, sopen_s_impl, 0);
define_stub_fn!(DLL, wsopen_impl, 0);
define_stub_fn!(DLL, wsopen_s_impl, 0);
define_stub_fn!(DLL, creat_impl, 0);
define_stub_fn!(DLL, wcreat_impl, 0);
define_stub_fn!(DLL, close_impl, 0);
define_stub_fn!(DLL, read_impl, 0);
define_stub_fn!(DLL, write_impl, 0);
define_stub_fn!(DLL, lseek_impl, 0);
define_stub_fn!(DLL, lseeki64_impl, 0);
define_stub_fn!(DLL, tell_impl, 0);
define_stub_fn!(DLL, telli64_impl, 0);
define_stub_fn!(DLL, dup_impl, 0);
define_stub_fn!(DLL, dup2_impl, 0);
define_stub_fn!(DLL, eof_impl, 0);
define_stub_fn!(DLL, commit_impl, 0);
define_stub_fn!(DLL, chsize_impl, 0);
define_stub_fn!(DLL, chsize_s_impl, 0);
define_stub_fn!(DLL, filelength_impl, 0);
define_stub_fn!(DLL, filelengthi64_impl, 0);
define_stub_fn!(DLL, locking_impl, 0);
define_stub_fn!(DLL, isatty_impl, 0);
define_stub_fn!(DLL, setmode_impl, 0);
define_stub_fn!(DLL, umask_impl, 0);
define_stub_fn!(DLL, umask_s_impl, 0);
define_stub_fn!(DLL, unlink_impl, 0);
define_stub_fn!(DLL, wunlink_impl, 0);
define_stub_fn!(DLL, access_impl, 0);
define_stub_fn!(DLL, access_s_impl, 0);
define_stub_fn!(DLL, waccess_impl, 0);
define_stub_fn!(DLL, waccess_s_impl, 0);
define_stub_fn!(DLL, chmod_impl, 0);
define_stub_fn!(DLL, wchmod_impl, 0);
define_stub_fn!(DLL, mktemp_impl, 0);
define_stub_fn!(DLL, mktemp_s_impl, 0);
define_stub_fn!(DLL, wmktemp_impl, 0);
define_stub_fn!(DLL, wmktemp_s_impl, 0);
define_stub_fn!(DLL, pipe_impl, 0);
define_stub_fn!(DLL, getcwd_impl, 0);
define_stub_fn!(DLL, wgetcwd_impl, 0);
define_stub_fn!(DLL, getdcwd_impl, 0);
define_stub_fn!(DLL, wgetdcwd_impl, 0);
define_stub_fn!(DLL, getdcwd_nolock_impl, 0);
define_stub_fn!(DLL, wgetdcwd_nolock_impl, 0);
define_stub_fn!(DLL, chdir_impl, 0);
define_stub_fn!(DLL, wchdir_impl, 0);
define_stub_fn!(DLL, mkdir_impl, 0);
define_stub_fn!(DLL, wmkdir_impl, 0);
define_stub_fn!(DLL, rmdir_impl, 0);
define_stub_fn!(DLL, wrmdir_impl, 0);
define_stub_fn!(DLL, getdrive_impl, 0);
define_stub_fn!(DLL, chdrive_impl, 0);

// File find
define_stub_fn!(DLL, findfirst32_impl, 0);
define_stub_fn!(DLL, findfirst32i64_impl, 0);
define_stub_fn!(DLL, findfirst64_impl, 0);
define_stub_fn!(DLL, findfirst64i32_impl, 0);
define_stub_fn!(DLL, findnext32_impl, 0);
define_stub_fn!(DLL, findnext32i64_impl, 0);
define_stub_fn!(DLL, findnext64_impl, 0);
define_stub_fn!(DLL, findnext64i32_impl, 0);
define_stub_fn!(DLL, findclose_impl, 0);
define_stub_fn!(DLL, wfindfirst32_impl, 0);
define_stub_fn!(DLL, wfindfirst32i64_impl, 0);
define_stub_fn!(DLL, wfindfirst64_impl, 0);
define_stub_fn!(DLL, wfindfirst64i32_impl, 0);
define_stub_fn!(DLL, wfindnext32_impl, 0);
define_stub_fn!(DLL, wfindnext32i64_impl, 0);
define_stub_fn!(DLL, wfindnext64_impl, 0);
define_stub_fn!(DLL, wfindnext64i32_impl, 0);

// File stat
define_stub_fn!(DLL, stat32_impl, 0);
define_stub_fn!(DLL, stat32i64_impl, 0);
define_stub_fn!(DLL, stat64_impl, 0);
define_stub_fn!(DLL, stat64i32_impl, 0);
define_stub_fn!(DLL, wstat32_impl, 0);
define_stub_fn!(DLL, wstat32i64_impl, 0);
define_stub_fn!(DLL, wstat64_impl, 0);
define_stub_fn!(DLL, wstat64i32_impl, 0);
define_stub_fn!(DLL, fstat32_impl, 0);
define_stub_fn!(DLL, fstat32i64_impl, 0);
define_stub_fn!(DLL, fstat64_impl, 0);
define_stub_fn!(DLL, fstat64i32_impl, 0);

// Pioinfo
define_stub_fn!(DLL, pioinfo, 0);
define_stub_fn!(DLL, badioinfo, 0);

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
