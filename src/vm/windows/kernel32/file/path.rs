use crate::vm::Vm;
use crate::vm_args;

use super::constants::DRIVE_FIXED;
use std::sync::atomic::{AtomicU32, Ordering};

static TEMP_COUNTER: AtomicU32 = AtomicU32::new(1);

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetDriveTypeA",
        crate::vm::stdcall_args(1),
        get_drive_type_a,
    );
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
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SearchPathA",
        crate::vm::stdcall_args(6),
        search_path_a,
    );
}

fn get_drive_type_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DRIVE_FIXED
}

fn get_logical_drive_strings_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_buffer_len, buffer) = vm_args!(vm, stack_ptr; u32, u32);
    if buffer == 0 {
        return 0;
    }
    let bytes = [b'C', b':', b'\\', 0, 0];
    let _ = vm.write_bytes(buffer, &bytes);
    bytes.len() as u32 - 1
}

fn get_temp_file_name_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let (path_ptr, prefix_ptr, unique, out_ptr) = vm_args!(_vm, _stack_ptr; u32, u32, u32, u32);
    if out_ptr == 0 {
        return 0;
    }
    let mut path = if path_ptr == 0 {
        "C:\\Windows\\Temp".to_string()
    } else {
        read_wide_or_utf16le_str!(_vm, path_ptr)
    };
    if path.is_empty() {
        path = "C:\\Windows\\Temp".to_string();
    }
    let mut prefix = if prefix_ptr == 0 {
        "TMP".to_string()
    } else {
        read_wide_or_utf16le_str!(_vm, prefix_ptr)
    };
    if prefix.is_empty() {
        prefix = "TMP".to_string();
    }
    if prefix.len() > 3 {
        prefix.truncate(3);
    }
    let unique_val = if unique != 0 {
        unique
    } else {
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed).max(1)
    };
    let sep = if path.ends_with(['\\', '/']) { "" } else { "\\" };
    let filename = format!("{path}{sep}{prefix}{:04X}.tmp", unique_val & 0xFFFF);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetTempFileNameA: {filename}");
    }
    let mut bytes = filename.as_bytes().to_vec();
    bytes.push(0);
    let _ = _vm.write_bytes(out_ptr, &bytes);
    if unique == 0 {
        let host_path = _vm.map_path(&filename);
        if let Some(parent) = std::path::Path::new(&host_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&host_path, b"");
    }
    unique_val
}

fn get_windows_directory_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (buffer, size) = vm_args!(vm, stack_ptr; u32, u32);
    let size = size as usize;
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

fn search_path_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr, file_ptr, ext_ptr, buffer_len, buffer_ptr, file_part_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);
    let buffer_len = buffer_len as usize;

    if file_ptr == 0 {
        return 0;
    }
    let file = read_wide_or_utf16le_str!(vm, file_ptr);
    let path = if path_ptr == 0 {
        String::new()
    } else {
        read_wide_or_utf16le_str!(vm, path_ptr)
    };
    let ext = if ext_ptr == 0 {
        String::new()
    } else {
        read_wide_or_utf16le_str!(vm, ext_ptr)
    };

    let mut candidates = Vec::new();
    let file_has_sep = file.contains('\\') || file.contains('/') || file.contains(':');
    let file_has_ext = {
        let last_sep = file.rfind(['\\', '/']).unwrap_or(0);
        file[last_sep..].contains('.')
    };
    let suffix = if !ext.is_empty() && !file_has_ext {
        if ext.starts_with('.') {
            ext.clone()
        } else {
            format!(".{ext}")
        }
    } else {
        String::new()
    };

    if file_has_sep || path.is_empty() {
        candidates.push(format!("{file}{suffix}"));
    } else {
        for dir in path.split(';') {
            let dir = dir.trim();
            if dir.is_empty() {
                continue;
            }
            let sep = if dir.ends_with(['\\', '/']) { "" } else { "\\" };
            candidates.push(format!("{dir}{sep}{file}{suffix}"));
        }
    }

    let mut found = None;
    for candidate in candidates {
        if vm.file_exists(&candidate) {
            found = Some(candidate);
            break;
        }
    }

    let Some(result) = found else {
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] SearchPathA: {path} {file} {ext} -> <missing>");
        }
        return 0;
    };

    let required = result.len() + 1;
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] SearchPathA: {path} {file} {ext} -> {result}");
    }
    if buffer_ptr == 0 || buffer_len == 0 {
        return required as u32;
    }
    let mut bytes = result.as_bytes().to_vec();
    if bytes.len() >= buffer_len {
        bytes.truncate(buffer_len.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(buffer_ptr, &bytes);
    if file_part_ptr != 0 {
        let part_offset = result.rfind(['\\', '/']).map(|idx| idx + 1).unwrap_or(0);
        let _ = vm.write_u32(file_part_ptr, buffer_ptr + part_offset as u32);
    }
    required as u32
}
