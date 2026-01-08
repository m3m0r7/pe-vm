use crate::vm::Vm;
use crate::vm_args;

use super::constants::{
    CREATE_ALWAYS, CREATE_NEW, ERROR_FILE_NOT_FOUND, GENERIC_READ, GENERIC_WRITE,
    INVALID_HANDLE_VALUE, OPEN_ALWAYS, TRUNCATE_EXISTING,
};
use super::helpers::read_w_string;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CopyFileA",
        crate::vm::stdcall_args(3),
        copy_file_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CreateDirectoryA",
        crate::vm::stdcall_args(2),
        create_directory_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CreateFileA",
        crate::vm::stdcall_args(7),
        create_file_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CreateFileW",
        crate::vm::stdcall_args(7),
        create_file_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "DeleteFileA",
        crate::vm::stdcall_args(1),
        delete_file_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "RemoveDirectoryW",
        crate::vm::stdcall_args(1),
        remove_directory_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SetFileAttributesA",
        crate::vm::stdcall_args(2),
        set_file_attributes_a,
    );
}

fn copy_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_directory_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr,) = vm_args!(vm, stack_ptr; u32);
    if path_ptr == 0 {
        return 0;
    }
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    let host_path = vm.map_path(&path);
    std::fs::create_dir_all(host_path).map(|_| 1).unwrap_or(0)
}

fn create_file_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr, desired, _share_mode, _security_attrs, disposition) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    let readable = desired & GENERIC_READ != 0;
    let writable = desired & GENERIC_WRITE != 0;
    let create = matches!(disposition, CREATE_NEW | CREATE_ALWAYS | OPEN_ALWAYS);
    let truncate = matches!(disposition, CREATE_ALWAYS | TRUNCATE_EXISTING);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] CreateFileA: {path} read={readable} write={writable} create={create} truncate={truncate}"
        );
    }
    match vm.file_open(&path, readable, writable, create, truncate) {
        Ok(handle) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] CreateFileA: {path} -> handle=0x{handle:08X}");
            }
            handle
        }
        Err(_) => {
            vm.set_last_error(ERROR_FILE_NOT_FOUND);
            INVALID_HANDLE_VALUE
        }
    }
}

fn create_file_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr, desired, _share_mode, _security_attrs, disposition) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }
    let path = read_w_string(vm, path_ptr);
    let readable = desired & GENERIC_READ != 0;
    let writable = desired & GENERIC_WRITE != 0;
    let create = matches!(disposition, CREATE_NEW | CREATE_ALWAYS | OPEN_ALWAYS);
    let truncate = matches!(disposition, CREATE_ALWAYS | TRUNCATE_EXISTING);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] CreateFileW: {path} read={readable} write={writable} create={create} truncate={truncate}"
        );
    }
    match vm.file_open(&path, readable, writable, create, truncate) {
        Ok(handle) => handle,
        Err(_) => {
            vm.set_last_error(ERROR_FILE_NOT_FOUND);
            INVALID_HANDLE_VALUE
        }
    }
}

fn delete_file_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr,) = vm_args!(vm, stack_ptr; u32);
    if path_ptr == 0 {
        return 0;
    }
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    let existed = vm.file_exists(&path);
    vm.file_delete(&path);
    if existed {
        1
    } else {
        0
    }
}

fn remove_directory_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_file_attributes_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
