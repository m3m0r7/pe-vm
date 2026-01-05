//! SHELL32 stubs for shell execution helpers.

use crate::vm::Vm;

// Register shell entry points that may be imported by GUI DLLs.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "SHELL32.dll",
        "ShellExecuteA",
        crate::vm::stdcall_args(6),
        shell_execute_a,
    );
    vm.register_import_stdcall(
        "SHELL32.dll",
        "ShellExecuteExA",
        crate::vm::stdcall_args(1),
        shell_execute_ex_a,
    );
    vm.register_import_stdcall(
        "SHELL32.dll",
        "SHBrowseForFolderA",
        crate::vm::stdcall_args(1),
        sh_browse_for_folder_a,
    );
    vm.register_import_stdcall(
        "SHELL32.dll",
        "SHGetPathFromIDListA",
        crate::vm::stdcall_args(2),
        sh_get_path_from_id_list_a,
    );
    vm.register_import_stdcall(
        "SHELL32.dll",
        "SHGetFileInfoA",
        crate::vm::stdcall_args(5),
        sh_get_file_info_a,
    );
}

fn shell_execute_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Return a success code (>32) to indicate the request was handled.
    33
}

fn shell_execute_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn sh_browse_for_folder_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn sh_get_path_from_id_list_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if buffer != 0 {
        let _ = vm.write_bytes(buffer, b"C:\\\0");
        return 1;
    }
    0
}

fn sh_get_file_info_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
