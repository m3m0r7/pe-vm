use crate::vm::windows::macros::read_wide_or_utf16le_str;
use crate::vm::Vm;
use crate::vm_args;

use super::constants::{
    ERROR_FILE_NOT_FOUND, ERROR_INVALID_HANDLE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL,
    FILE_TYPE_DISK, INVALID_FILE_ATTRIBUTES,
};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FlushFileBuffers",
        crate::vm::stdcall_args(1),
        flush_file_buffers,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetFileAttributesA",
        crate::vm::stdcall_args(1),
        get_file_attributes_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetFileSize",
        crate::vm::stdcall_args(2),
        get_file_size,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetFileTime",
        crate::vm::stdcall_args(4),
        get_file_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetFileType",
        crate::vm::stdcall_args(1),
        get_file_type,
    );
}

fn flush_file_buffers(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_file_attributes_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr,) = vm_args!(vm, stack_ptr; u32);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_FILE_ATTRIBUTES;
    }
    let path = read_wide_or_utf16le_str(vm, path_ptr);
    let result = if vm.file_exists(&path) {
        let host_path = vm.map_path(&path);
        if std::path::Path::new(&host_path).is_dir() {
            FILE_ATTRIBUTE_DIRECTORY
        } else {
            FILE_ATTRIBUTE_NORMAL
        }
    } else {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        INVALID_FILE_ATTRIBUTES
    };
    if std::env::var("PE_VM_TRACE").is_ok() {
        let host_path = vm.map_path(&path);
        eprintln!(
            "[pe_vm] GetFileAttributesA: {path} -> 0x{result:08X} (host={host_path})"
        );
    }
    result
}

fn get_file_size(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle, high_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    match vm.file_size(handle) {
        Some(size) => {
            if high_ptr != 0 {
                let _ = vm.write_u32(high_ptr, 0);
            }
            size
        }
        None => {
            vm.set_last_error(ERROR_INVALID_HANDLE);
            INVALID_FILE_ATTRIBUTES
        }
    }
}

fn get_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_handle, creation, access, write) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    if creation != 0 {
        let _ = vm.write_u32(creation, 0);
        let _ = vm.write_u32(creation + 4, 0);
    }
    if access != 0 {
        let _ = vm.write_u32(access, 0);
        let _ = vm.write_u32(access + 4, 0);
    }
    if write != 0 {
        let _ = vm.write_u32(write, 0);
        let _ = vm.write_u32(write + 4, 0);
    }
    1
}

fn get_file_type(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    FILE_TYPE_DISK
}
