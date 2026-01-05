//! Kernel32 file-system related stubs.

use crate::vm::Vm;

const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const FILE_TYPE_DISK: u32 = 1;
const DRIVE_FIXED: u32 = 3;
const INVALID_HANDLE_VALUE: u32 = 0xFFFF_FFFF;
const INVALID_FILE_ATTRIBUTES: u32 = 0xFFFF_FFFF;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;

const CREATE_NEW: u32 = 1;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;
const OPEN_ALWAYS: u32 = 4;
const TRUNCATE_EXISTING: u32 = 5;

const FILE_BEGIN: u32 = 0;

const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_INVALID_HANDLE: u32 = 6;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "CloseHandle", crate::vm::stdcall_args(1), close_handle);
    vm.register_import_stdcall("KERNEL32.dll", "CopyFileA", crate::vm::stdcall_args(3), copy_file_a);
    vm.register_import_stdcall("KERNEL32.dll", "CreateDirectoryA", crate::vm::stdcall_args(2), create_directory_a);
    vm.register_import_stdcall("KERNEL32.dll", "CreateFileA", crate::vm::stdcall_args(7), create_file_a);
    vm.register_import_stdcall("KERNEL32.dll", "CreateFileW", crate::vm::stdcall_args(7), create_file_w);
    vm.register_import_stdcall("KERNEL32.dll", "DeleteFileA", crate::vm::stdcall_args(1), delete_file_a);
    vm.register_import_stdcall("KERNEL32.dll", "FindClose", crate::vm::stdcall_args(1), find_close);
    vm.register_import_stdcall("KERNEL32.dll", "FindFirstFileA", crate::vm::stdcall_args(2), find_first_file_a);
    vm.register_import_stdcall("KERNEL32.dll", "FindNextFileA", crate::vm::stdcall_args(2), find_next_file_a);
    vm.register_import_stdcall("KERNEL32.dll", "FlushFileBuffers", crate::vm::stdcall_args(1), flush_file_buffers);
    vm.register_import_stdcall("KERNEL32.dll", "GetDriveTypeA", crate::vm::stdcall_args(1), get_drive_type_a);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetFileAttributesA",
        crate::vm::stdcall_args(1),
        get_file_attributes_a,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetFileSize", crate::vm::stdcall_args(2), get_file_size);
    vm.register_import_stdcall("KERNEL32.dll", "GetFileTime", crate::vm::stdcall_args(4), get_file_time);
    vm.register_import_stdcall("KERNEL32.dll", "GetFileType", crate::vm::stdcall_args(1), get_file_type);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetLogicalDriveStringsA",
        crate::vm::stdcall_args(2),
        get_logical_drive_strings_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetTempFileNameA",
        crate::vm::stdcall_args(4),
        get_temp_file_name_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetWindowsDirectoryA",
        crate::vm::stdcall_args(2),
        get_windows_directory_a,
    );
    vm.register_import_stdcall("KERNEL32.dll", "RemoveDirectoryW", crate::vm::stdcall_args(1), remove_directory_w);
    vm.register_import_stdcall("KERNEL32.dll", "SearchPathA", crate::vm::stdcall_args(6), search_path_a);
    vm.register_import_stdcall("KERNEL32.dll", "SetEndOfFile", crate::vm::stdcall_args(1), set_end_of_file);
    vm.register_import_stdcall("KERNEL32.dll", "SetFileAttributesA", crate::vm::stdcall_args(2), set_file_attributes_a);
    vm.register_import_stdcall("KERNEL32.dll", "SetFilePointer", crate::vm::stdcall_args(4), set_file_pointer);
    vm.register_import_stdcall("KERNEL32.dll", "SetFilePointerEx", crate::vm::stdcall_args(5), set_file_pointer_ex);
    vm.register_import_stdcall("KERNEL32.dll", "SetFileTime", crate::vm::stdcall_args(4), set_file_time);
}

fn close_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if handle != 0 {
        vm.file_close(handle);
    }
    1
}

fn copy_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_directory_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_file_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }
    let path = vm.read_c_string(path_ptr).unwrap_or_default();
    let desired = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let disposition = vm.read_u32(stack_ptr + 20).unwrap_or(OPEN_EXISTING);
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
        Ok(handle) => handle,
        Err(_) => {
            vm.set_last_error(ERROR_FILE_NOT_FOUND);
            INVALID_HANDLE_VALUE
        }
    }
}

fn create_file_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }
    let path = read_w_string(vm, path_ptr);
    let desired = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let disposition = vm.read_u32(stack_ptr + 20).unwrap_or(OPEN_EXISTING);
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
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if path_ptr == 0 {
        return 0;
    }
    let path = vm.read_c_string(path_ptr).unwrap_or_default();
    let existed = vm.file_exists(&path);
    vm.file_delete(&path);
    if existed { 1 } else { 0 }
}

fn find_close(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn find_first_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    INVALID_HANDLE_VALUE
}

fn find_next_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn flush_file_buffers(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_drive_type_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DRIVE_FIXED
}

fn get_file_attributes_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if path_ptr == 0 {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        return INVALID_FILE_ATTRIBUTES;
    }
    let path = vm.read_c_string(path_ptr).unwrap_or_default();
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetFileAttributesA: {path}");
    }
    if vm.file_exists(&path) {
        FILE_ATTRIBUTE_NORMAL
    } else {
        vm.set_last_error(ERROR_FILE_NOT_FOUND);
        INVALID_FILE_ATTRIBUTES
    }
}

fn get_file_size(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let high_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
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
    let creation = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let access = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let write = vm.read_u32(stack_ptr + 16).unwrap_or(0);
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

fn get_logical_drive_strings_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if buffer == 0 {
        return 0;
    }
    let bytes = b"C:\\\0\0";
    let _ = vm.write_bytes(buffer, bytes);
    bytes.len() as u32 - 1
}

fn get_temp_file_name_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_windows_directory_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    if buffer == 0 || size == 0 {
        return 0;
    }
    let mut bytes = b"C:\\Windows".to_vec();
    if bytes.len() >= size {
        bytes.truncate(size.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(buffer, &bytes);
    (bytes.len().saturating_sub(1)) as u32
}

fn remove_directory_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn search_path_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn set_end_of_file(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_file_attributes_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_file_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let distance = vm.read_u32(stack_ptr + 8).unwrap_or(0) as i32 as i64;
    let high_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let method = vm.read_u32(stack_ptr + 16).unwrap_or(FILE_BEGIN);
    let mut offset = distance;
    if high_ptr != 0 {
        let high = vm.read_u32(high_ptr).unwrap_or(0) as i32 as i64;
        offset |= high << 32;
    }
    match vm.file_seek(handle, offset, method) {
        Some(pos) => {
            if high_ptr != 0 {
                let _ = vm.write_u32(high_ptr, (pos >> 32) as u32);
            }
            pos as u32
        }
        None => {
            vm.set_last_error(ERROR_INVALID_HANDLE);
            INVALID_HANDLE_VALUE
        }
    }
}

fn set_file_pointer_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let low = vm.read_u32(stack_ptr + 8).unwrap_or(0) as u64;
    let high = vm.read_u32(stack_ptr + 12).unwrap_or(0) as u64;
    let out = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let method = vm.read_u32(stack_ptr + 20).unwrap_or(FILE_BEGIN);
    let offset = ((high << 32) | low) as i64;
    match vm.file_seek(handle, offset, method) {
        Some(pos) => {
            if out != 0 {
                let _ = vm.write_u32(out, pos as u32);
                let _ = vm.write_u32(out + 4, (pos >> 32) as u32);
            }
            1
        }
        None => {
            vm.set_last_error(ERROR_INVALID_HANDLE);
            0
        }
    }
}

fn set_file_time(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn read_w_string(vm: &Vm, ptr: u32) -> String {
    let mut units = Vec::new();
    let mut cursor = ptr;
    loop {
        let unit = vm.read_u16(cursor).unwrap_or(0);
        if unit == 0 {
            break;
        }
        units.push(unit);
        cursor = cursor.wrapping_add(2);
    }
    String::from_utf16_lossy(&units)
}
