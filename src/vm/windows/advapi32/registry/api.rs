//! Registry API entry points.

use crate::vm::windows::registry::RegistryValue;
use crate::vm::windows::{get_registry, get_registry_mut};
use crate::vm::Vm;

use super::constants::{
    ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, REG_BINARY,
    REG_DWORD, REG_MULTI_SZ, REG_SZ,
};
use super::helpers::{
    format_registry_key, is_root_hive, join_key, read_bytes, read_raw_ascii, read_raw_bytes,
    read_string_arg, registry_prefix, resolve_registry_value, write_zero_counts,
};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegOpenKeyExA",
        crate::vm::stdcall_args(5),
        reg_open_key_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegCreateKeyExA",
        crate::vm::stdcall_args(9),
        reg_create_key_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegQueryValueExA",
        crate::vm::stdcall_args(6),
        reg_query_value_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegSetValueExA",
        crate::vm::stdcall_args(6),
        reg_set_value_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegCloseKey",
        crate::vm::stdcall_args(1),
        reg_close_key,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegDeleteValueA",
        crate::vm::stdcall_args(2),
        reg_delete_value_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegDeleteKeyA",
        crate::vm::stdcall_args(2),
        reg_delete_key_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegEnumKeyExA",
        crate::vm::stdcall_args(8),
        reg_enum_key_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegQueryInfoKeyA",
        crate::vm::stdcall_args(12),
        reg_query_info_key_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegQueryInfoKeyW",
        crate::vm::stdcall_args(12),
        reg_query_info_key_w,
    );
}

fn reg_open_key_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let subkey_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if out_ptr == 0 {
        return ERROR_FILE_NOT_FOUND;
    }
    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let subkey = read_string_arg(vm, subkey_ptr);
    let path = join_key(&prefix, &subkey);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] RegOpenKeyExA: {path} (hkey=0x{hkey:08X} subkey_ptr=0x{subkey_ptr:08X})"
        );
        if subkey_ptr != 0 {
            let raw = read_raw_bytes(vm, subkey_ptr, 128);
            let ascii = read_raw_ascii(vm, subkey_ptr, 128);
            eprintln!("[pe_vm] RegOpenKeyExA subkey raw: {raw}");
            eprintln!("[pe_vm] RegOpenKeyExA subkey ascii: {ascii}");
            let window_start = subkey_ptr.wrapping_sub(64);
            let window_raw = read_raw_bytes(vm, window_start, 192);
            let window_ascii = read_raw_ascii(vm, window_start, 192);
            eprintln!(
                "[pe_vm] RegOpenKeyExA subkey window @0x{window_start:08X} raw: {window_raw}"
            );
            eprintln!(
                "[pe_vm] RegOpenKeyExA subkey window @0x{window_start:08X} ascii: {window_ascii}"
            );
        }
    }
    let handle = vm.registry_open_handle(path);
    let _ = vm.write_u32(out_ptr, handle);
    ERROR_SUCCESS
}

fn reg_create_key_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let subkey_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 28).unwrap_or(0);
    if out_ptr == 0 {
        return ERROR_FILE_NOT_FOUND;
    }
    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let subkey = read_string_arg(vm, subkey_ptr);
    let path = join_key(&prefix, &subkey);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] RegCreateKeyExA: {path}");
    }
    let handle = vm.registry_open_handle(path);
    let _ = vm.write_u32(out_ptr, handle);
    ERROR_SUCCESS
}

fn reg_query_value_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let type_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let data_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let size_ptr = vm.read_u32(stack_ptr + 24).unwrap_or(0);

    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let value_name = if value_ptr == 0 {
        None
    } else {
        let name = read_string_arg(vm, value_ptr);
        if name.is_empty() { None } else { Some(name) }
    };
    let key = format_registry_key(&prefix, value_name.as_deref());
    if std::env::var("PE_VM_TRACE").is_ok() {
        let name = value_name.as_deref().unwrap_or("(Default)");
        eprintln!(
            "[pe_vm] RegQueryValueExA: {key} ({name}) value_ptr=0x{value_ptr:08X}"
        );
        if size_ptr != 0 {
            let requested = vm.read_u32(size_ptr).unwrap_or(0);
            eprintln!("[pe_vm] RegQueryValueExA size request: {requested}");
        }
        if value_ptr != 0 {
            let raw = read_raw_bytes(vm, value_ptr, 96);
            let ascii = read_raw_ascii(vm, value_ptr, 96);
            eprintln!("[pe_vm] RegQueryValueExA name raw: {raw}");
            eprintln!("[pe_vm] RegQueryValueExA name ascii: {ascii}");
        }
    }

    let registry = match get_registry(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    let value = match resolve_registry_value(vm, registry, &key) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };

    let (value_type, bytes) = match value {
        RegistryValue::String(text) => {
            let mut bytes = text.as_bytes().to_vec();
            bytes.push(0);
            (REG_SZ, bytes)
        }
        RegistryValue::Dword(value) => (REG_DWORD, value.to_le_bytes().to_vec()),
        RegistryValue::MultiString(values) => {
            let mut bytes = Vec::new();
            for item in values {
                bytes.extend_from_slice(item.as_bytes());
                bytes.push(0);
            }
            bytes.push(0);
            (REG_MULTI_SZ, bytes)
        }
        RegistryValue::Binary(bytes) => (REG_BINARY, bytes.clone()),
    };

    if type_ptr != 0 {
        let _ = vm.write_u32(type_ptr, value_type);
    }
    let requested_size = if size_ptr == 0 {
        None
    } else {
        Some(vm.read_u32(size_ptr).unwrap_or(0) as usize)
    };
    if size_ptr != 0 {
        let _ = vm.write_u32(size_ptr, bytes.len() as u32);
    }
    if data_ptr == 0 {
        return ERROR_SUCCESS;
    }
    let buffer_size = requested_size.unwrap_or(bytes.len());
    if buffer_size < bytes.len() {
        let _ = vm.write_bytes(data_ptr, &bytes[..buffer_size]);
        return ERROR_MORE_DATA;
    }
    let _ = vm.write_bytes(data_ptr, &bytes);
    ERROR_SUCCESS
}

fn reg_set_value_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let _reserved = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let value_type = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let data_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let data_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;

    if value_type != REG_SZ || data_ptr == 0 {
        return ERROR_SUCCESS;
    }

    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let value_name = if value_ptr == 0 {
        None
    } else {
        let name = read_string_arg(vm, value_ptr);
        if name.is_empty() { None } else { Some(name) }
    };
    let key = format_registry_key(&prefix, value_name.as_deref());

    let data = read_bytes(vm, data_ptr, data_len);
    let text = String::from_utf8_lossy(&data).trim_end_matches('\0').to_string();
    let registry = match get_registry_mut(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    let _ = registry.set(&key, RegistryValue::String(text));
    ERROR_SUCCESS
}

fn reg_close_key(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if is_root_hive(hkey) {
        return ERROR_SUCCESS;
    }
    vm.registry_close_handle(hkey);
    ERROR_SUCCESS
}

fn reg_delete_value_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    ERROR_SUCCESS
}

fn reg_delete_key_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    ERROR_SUCCESS
}

fn reg_enum_key_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] RegEnumKeyExA");
    }
    ERROR_NO_MORE_ITEMS
}

fn reg_query_info_key_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] RegQueryInfoKeyA");
    }
    write_zero_counts(vm, stack_ptr)
}

fn reg_query_info_key_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] RegQueryInfoKeyW");
    }
    write_zero_counts(vm, stack_ptr)
}
