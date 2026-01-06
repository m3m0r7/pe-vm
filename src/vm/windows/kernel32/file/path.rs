use crate::vm::Vm;

use super::constants::DRIVE_FIXED;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetDriveTypeA", crate::vm::stdcall_args(1), get_drive_type_a);
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
    vm.register_import_stdcall("KERNEL32.dll", "SearchPathA", crate::vm::stdcall_args(6), search_path_a);
}

fn get_drive_type_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DRIVE_FIXED
}

fn get_logical_drive_strings_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if buffer == 0 {
        return 0;
    }
    let bytes = [b'C', b':', b'\\', 0, 0];
    let _ = vm.write_bytes(buffer, &bytes);
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

fn search_path_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let file_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let ext_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let buffer_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    let buffer_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let file_part_ptr = vm.read_u32(stack_ptr + 24).unwrap_or(0);

    if file_ptr == 0 {
        return 0;
    }
    let file = vm.read_c_string(file_ptr).unwrap_or_default();
    let path = if path_ptr == 0 {
        String::new()
    } else {
        vm.read_c_string(path_ptr).unwrap_or_default()
    };
    let ext = if ext_ptr == 0 {
        String::new()
    } else {
        vm.read_c_string(ext_ptr).unwrap_or_default()
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

    if file_has_sep {
        candidates.push(format!("{file}{suffix}"));
    } else if path.is_empty() {
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
        let part_offset = result
            .rfind(['\\', '/'])
            .map(|idx| idx + 1)
            .unwrap_or(0);
        let _ = vm.write_u32(file_part_ptr, buffer_ptr + part_offset as u32);
    }
    required as u32
}
