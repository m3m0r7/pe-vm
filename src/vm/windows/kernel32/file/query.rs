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
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
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
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] GetFileSize: handle=0x{handle:08X} size={size}");
            }
            if high_ptr != 0 {
                let _ = vm.write_u32(high_ptr, 0);
            }
            size
        }
        None => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] GetFileSize: handle=0x{handle:08X} INVALID");
            }
            vm.set_last_error(ERROR_INVALID_HANDLE);
            INVALID_FILE_ATTRIBUTES
        }
    }
}

fn get_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_handle, creation, access, write) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    // Return a valid FILETIME corresponding to 2001/02/26 15:28:46 UTC.
    // FILETIME is 100-nanosecond intervals since January 1, 1601 UTC.
    // 2001-02-26 15:28:46 UTC = 126596465260000000 (100ns intervals)
    let filetime_low: u32 = 0x4D5D7400;
    let filetime_high: u32 = 0x01C0A7C0;
    if creation != 0 {
        let _ = vm.write_u32(creation, filetime_low);
        let _ = vm.write_u32(creation + 4, filetime_high);
    }
    if access != 0 {
        let _ = vm.write_u32(access, filetime_low);
        let _ = vm.write_u32(access + 4, filetime_high);
    }
    if write != 0 {
        let _ = vm.write_u32(write, filetime_low);
        let _ = vm.write_u32(write + 4, filetime_high);
    }
    1
}

fn get_file_type(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    FILE_TYPE_DISK
}
