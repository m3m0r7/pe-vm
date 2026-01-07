//! STKIT432.dll stub registration (Setup Toolkit).
//!
//! This module provides stub implementations for the Setup Toolkit DLL
//! commonly used in older Windows installers (REGSVR32.DLL wrapper).

pub const DLL_NAME: &str = "STKIT432.dll";
const DLL: &str = DLL_NAME;

use crate::vm::Vm;



// Setup Toolkit functions
define_stub_fn!(DLL, sync_shell, 0);
define_stub_fn!(DLL, dll_self_register, 0);
define_stub_fn!(DLL, alloc_unit, 0);
define_stub_fn!(DLL, set_time, 0);
define_stub_fn!(DLL, disk_space_free, 0);
define_stub_fn!(DLL, get_win_platform, 0);
define_stub_fn!(DLL, lmemcpy, 0);
define_stub_fn!(DLL, f_nt_with_shell, 0);

// Action logging functions
define_stub_fn!(DLL, abort_action, 0);
define_stub_fn!(DLL, add_action_note, 0);
define_stub_fn!(DLL, change_action_key, 0);
define_stub_fn!(DLL, commit_action, 0);
define_stub_fn!(DLL, f_within_action, 0);
define_stub_fn!(DLL, log_error, 0);
define_stub_fn!(DLL, log_warning, 0);
define_stub_fn!(DLL, new_action, 0);
define_stub_fn!(DLL, enable_logging, 0);
define_stub_fn!(DLL, disable_logging, 0);
define_stub_fn!(DLL, log_note, 0);
define_stub_fn!(DLL, log_config, 0);

// Shell folder functions
define_stub_fn!(DLL, f_create_shell_folder, 0);
define_stub_fn!(DLL, f_create_shell_link, 0);
define_stub_fn!(DLL, f_remove_shell_link, 0);
define_stub_fn!(DLL, get_long_path_name, 0);

pub fn register(vm: &mut Vm) {
    // Core setup functions
    vm.register_import(DLL_NAME, "SyncShell", sync_shell);
    vm.register_import(DLL_NAME, "DLLSelfRegister", dll_self_register);
    vm.register_import(DLL_NAME, "AllocUnit", alloc_unit);
    vm.register_import(DLL_NAME, "SetTime", set_time);
    vm.register_import(DLL_NAME, "DISKSPACEFREE", disk_space_free);
    vm.register_import(DLL_NAME, "GetWinPlatform", get_win_platform);
    vm.register_import(DLL_NAME, "lmemcpy", lmemcpy);
    vm.register_import(DLL_NAME, "fNTWithShell", f_nt_with_shell);

    // Action logging
    vm.register_import(DLL_NAME, "AbortAction", abort_action);
    vm.register_import(DLL_NAME, "AddActionNote", add_action_note);
    vm.register_import(DLL_NAME, "ChangeActionKey", change_action_key);
    vm.register_import(DLL_NAME, "CommitAction", commit_action);
    vm.register_import(DLL_NAME, "fWithinAction", f_within_action);
    vm.register_import(DLL_NAME, "LogError", log_error);
    vm.register_import(DLL_NAME, "LogWarning", log_warning);
    vm.register_import(DLL_NAME, "NewAction", new_action);
    vm.register_import(DLL_NAME, "EnableLogging", enable_logging);
    vm.register_import(DLL_NAME, "DisableLogging", disable_logging);
    vm.register_import(DLL_NAME, "LogNote", log_note);
    vm.register_import(DLL_NAME, "LogConfig", log_config);

    // Shell folder functions
    vm.register_import(DLL_NAME, "fCreateShellFolder", f_create_shell_folder);
    vm.register_import(DLL_NAME, "fCreateShellLink", f_create_shell_link);
    vm.register_import(DLL_NAME, "fRemoveShellLink", f_remove_shell_link);
    vm.register_import(DLL_NAME, "GetLongPathName", get_long_path_name);

    // Also register as REGSVR32.DLL for compatibility
    vm.register_import("REGSVR32.DLL", "SyncShell", sync_shell);
    vm.register_import("REGSVR32.DLL", "DLLSelfRegister", dll_self_register);
    vm.register_import("REGSVR32.DLL", "AllocUnit", alloc_unit);
    vm.register_import("REGSVR32.DLL", "SetTime", set_time);
    vm.register_import("REGSVR32.DLL", "DISKSPACEFREE", disk_space_free);
    vm.register_import("REGSVR32.DLL", "GetWinPlatform", get_win_platform);
    vm.register_import("REGSVR32.DLL", "lmemcpy", lmemcpy);
    vm.register_import("REGSVR32.DLL", "fNTWithShell", f_nt_with_shell);
    vm.register_import("REGSVR32.DLL", "AbortAction", abort_action);
    vm.register_import("REGSVR32.DLL", "AddActionNote", add_action_note);
    vm.register_import("REGSVR32.DLL", "ChangeActionKey", change_action_key);
    vm.register_import("REGSVR32.DLL", "CommitAction", commit_action);
    vm.register_import("REGSVR32.DLL", "fWithinAction", f_within_action);
    vm.register_import("REGSVR32.DLL", "LogError", log_error);
    vm.register_import("REGSVR32.DLL", "LogWarning", log_warning);
    vm.register_import("REGSVR32.DLL", "NewAction", new_action);
    vm.register_import("REGSVR32.DLL", "EnableLogging", enable_logging);
    vm.register_import("REGSVR32.DLL", "DisableLogging", disable_logging);
    vm.register_import("REGSVR32.DLL", "LogNote", log_note);
    vm.register_import("REGSVR32.DLL", "LogConfig", log_config);
    vm.register_import("REGSVR32.DLL", "fCreateShellFolder", f_create_shell_folder);
    vm.register_import("REGSVR32.DLL", "fCreateShellLink", f_create_shell_link);
    vm.register_import("REGSVR32.DLL", "fRemoveShellLink", f_remove_shell_link);
    vm.register_import("REGSVR32.DLL", "GetLongPathName", get_long_path_name);
}
