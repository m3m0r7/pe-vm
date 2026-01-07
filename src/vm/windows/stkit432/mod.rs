//! STKIT432.dll stub registration (Setup Toolkit).
//!
//! This module provides stub implementations for the Setup Toolkit DLL
//! commonly used in older Windows installers (REGSVR32.DLL wrapper).

use crate::vm::windows::check_stub;
use crate::vm::Vm;

const DLL: &str = "STKIT432.dll";

macro_rules! stub {
    ($name:ident) => {
        fn $name(vm: &mut Vm, _sp: u32) -> u32 {
            check_stub(vm, DLL, stringify!($name));
            0
        }
    };
}

// Setup Toolkit functions
stub!(sync_shell);
stub!(dll_self_register);
stub!(alloc_unit);
stub!(set_time);
stub!(disk_space_free);
stub!(get_win_platform);
stub!(lmemcpy);
stub!(f_nt_with_shell);

// Action logging functions
stub!(abort_action);
stub!(add_action_note);
stub!(change_action_key);
stub!(commit_action);
stub!(f_within_action);
stub!(log_error);
stub!(log_warning);
stub!(new_action);
stub!(enable_logging);
stub!(disable_logging);
stub!(log_note);
stub!(log_config);

// Shell folder functions
stub!(f_create_shell_folder);
stub!(f_create_shell_link);
stub!(f_remove_shell_link);
stub!(get_long_path_name);

pub fn register(vm: &mut Vm) {
    // Core setup functions
    vm.register_import(DLL, "SyncShell", sync_shell);
    vm.register_import(DLL, "DLLSelfRegister", dll_self_register);
    vm.register_import(DLL, "AllocUnit", alloc_unit);
    vm.register_import(DLL, "SetTime", set_time);
    vm.register_import(DLL, "DISKSPACEFREE", disk_space_free);
    vm.register_import(DLL, "GetWinPlatform", get_win_platform);
    vm.register_import(DLL, "lmemcpy", lmemcpy);
    vm.register_import(DLL, "fNTWithShell", f_nt_with_shell);

    // Action logging
    vm.register_import(DLL, "AbortAction", abort_action);
    vm.register_import(DLL, "AddActionNote", add_action_note);
    vm.register_import(DLL, "ChangeActionKey", change_action_key);
    vm.register_import(DLL, "CommitAction", commit_action);
    vm.register_import(DLL, "fWithinAction", f_within_action);
    vm.register_import(DLL, "LogError", log_error);
    vm.register_import(DLL, "LogWarning", log_warning);
    vm.register_import(DLL, "NewAction", new_action);
    vm.register_import(DLL, "EnableLogging", enable_logging);
    vm.register_import(DLL, "DisableLogging", disable_logging);
    vm.register_import(DLL, "LogNote", log_note);
    vm.register_import(DLL, "LogConfig", log_config);

    // Shell folder functions
    vm.register_import(DLL, "fCreateShellFolder", f_create_shell_folder);
    vm.register_import(DLL, "fCreateShellLink", f_create_shell_link);
    vm.register_import(DLL, "fRemoveShellLink", f_remove_shell_link);
    vm.register_import(DLL, "GetLongPathName", get_long_path_name);

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
