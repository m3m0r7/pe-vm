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
    read_string_arg_a, read_string_arg_w, redirect_wow6432_key, registry_prefix,
    resolve_registry_value,
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
        "RegOpenKeyExW",
        crate::vm::stdcall_args(5),
        reg_open_key_ex_w,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegCreateKeyExA",
        crate::vm::stdcall_args(9),
        reg_create_key_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegCreateKeyExW",
        crate::vm::stdcall_args(9),
        reg_create_key_ex_w,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegQueryValueExA",
        crate::vm::stdcall_args(6),
        reg_query_value_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegQueryValueExW",
        crate::vm::stdcall_args(6),
        reg_query_value_ex_w,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegSetValueExA",
        crate::vm::stdcall_args(6),
        reg_set_value_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegSetValueExW",
        crate::vm::stdcall_args(6),
        reg_set_value_ex_w,
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
        "RegDeleteValueW",
        crate::vm::stdcall_args(2),
        reg_delete_value_w,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegDeleteKeyA",
        crate::vm::stdcall_args(2),
        reg_delete_key_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegDeleteKeyW",
        crate::vm::stdcall_args(2),
        reg_delete_key_w,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegEnumKeyExA",
        crate::vm::stdcall_args(8),
        reg_enum_key_ex_a,
    );
    vm.register_import_stdcall(
        "ADVAPI32.dll",
        "RegEnumKeyExW",
        crate::vm::stdcall_args(8),
        reg_enum_key_ex_w,
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
    reg_open_key_ex(vm, stack_ptr, "RegOpenKeyExA", false)
}

fn reg_open_key_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_open_key_ex(vm, stack_ptr, "RegOpenKeyExW", true)
}

fn reg_open_key_ex(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
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
    let subkey = if wide {
        read_string_arg_w(vm, subkey_ptr)
    } else {
        read_string_arg_a(vm, subkey_ptr)
    };
    let mut path = join_key(&prefix, &subkey);
    if let Some(redirected) = redirect_wow6432_key(vm, &path) {
        path = redirected;
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] {api}: {path} (hkey=0x{hkey:08X} subkey_ptr=0x{subkey_ptr:08X})"
        );
        if subkey_ptr != 0 {
            let raw = read_raw_bytes(vm, subkey_ptr, 128);
            let ascii = read_raw_ascii(vm, subkey_ptr, 128);
            eprintln!("[pe_vm] {api} subkey raw: {raw}");
            eprintln!("[pe_vm] {api} subkey ascii: {ascii}");
            let window_start = subkey_ptr.wrapping_sub(64);
            let window_raw = read_raw_bytes(vm, window_start, 192);
            let window_ascii = read_raw_ascii(vm, window_start, 192);
            eprintln!(
                "[pe_vm] {api} subkey window @0x{window_start:08X} raw: {window_raw}"
            );
            eprintln!(
                "[pe_vm] {api} subkey window @0x{window_start:08X} ascii: {window_ascii}"
            );
        }
    }
    let handle = vm.registry_open_handle(path);
    let _ = vm.write_u32(out_ptr, handle);
    ERROR_SUCCESS
}

fn reg_create_key_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_create_key_ex(vm, stack_ptr, "RegCreateKeyExA", false)
}

fn reg_create_key_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_create_key_ex(vm, stack_ptr, "RegCreateKeyExW", true)
}

fn reg_create_key_ex(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
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
    let subkey = if wide {
        read_string_arg_w(vm, subkey_ptr)
    } else {
        read_string_arg_a(vm, subkey_ptr)
    };
    let mut path = join_key(&prefix, &subkey);
    if let Some(redirected) = redirect_wow6432_key(vm, &path) {
        path = redirected;
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] {api}: {path}");
    }
    let handle = vm.registry_open_handle(path);
    let _ = vm.write_u32(out_ptr, handle);
    ERROR_SUCCESS
}

fn reg_query_value_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_query_value_ex(vm, stack_ptr, "RegQueryValueExA", false)
}

fn reg_query_value_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_query_value_ex(vm, stack_ptr, "RegQueryValueExW", true)
}

fn reg_query_value_ex(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
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
        let name = if wide {
            read_string_arg_w(vm, value_ptr)
        } else {
            read_string_arg_a(vm, value_ptr)
        };
        if name.is_empty() { None } else { Some(name) }
    };
    let key = format_registry_key(&prefix, value_name.as_deref());
    if std::env::var("PE_VM_TRACE").is_ok() {
        let name = value_name.as_deref().unwrap_or("(Default)");
        eprintln!(
            "[pe_vm] {api}: {key} ({name}) value_ptr=0x{value_ptr:08X}"
        );
        if size_ptr != 0 {
            let requested = vm.read_u32(size_ptr).unwrap_or(0);
            eprintln!("[pe_vm] {api} size request: {requested}");
        }
        if value_ptr != 0 {
            let raw = read_raw_bytes(vm, value_ptr, 96);
            let ascii = read_raw_ascii(vm, value_ptr, 96);
            eprintln!("[pe_vm] {api} name raw: {raw}");
            eprintln!("[pe_vm] {api} name ascii: {ascii}");
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
            let bytes = if wide {
                encode_wide_string(&text)
            } else {
                let mut bytes = text.as_bytes().to_vec();
                bytes.push(0);
                bytes
            };
            (REG_SZ, bytes)
        }
        RegistryValue::Dword(value) => (REG_DWORD, value.to_le_bytes().to_vec()),
        RegistryValue::MultiString(values) => {
            let bytes = if wide {
                encode_wide_multi_string(&values)
            } else {
                let mut bytes = Vec::new();
                for item in values {
                    bytes.extend_from_slice(item.as_bytes());
                    bytes.push(0);
                }
                bytes.push(0);
                bytes
            };
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
    reg_set_value_ex(vm, stack_ptr, "RegSetValueExA", false)
}

fn reg_set_value_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_set_value_ex(vm, stack_ptr, "RegSetValueExW", true)
}

fn reg_set_value_ex(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let _reserved = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let value_type = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let data_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let data_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;

    if data_ptr == 0 {
        return ERROR_SUCCESS;
    }

    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let value_name = if value_ptr == 0 {
        None
    } else {
        let name = if wide {
            read_string_arg_w(vm, value_ptr)
        } else {
            read_string_arg_a(vm, value_ptr)
        };
        if name.is_empty() { None } else { Some(name) }
    };
    let key = format_registry_key(&prefix, value_name.as_deref());

    let data = read_bytes(vm, data_ptr, data_len);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let name = value_name.as_deref().unwrap_or("(Default)");
        eprintln!(
            "[pe_vm] {api}: {key} ({name}) type=0x{value_type:08X} len={data_len}"
        );
    }
    let value = match value_type {
        REG_SZ => RegistryValue::String(if wide {
            decode_wide_string(&data)
        } else {
            String::from_utf8_lossy(&data).trim_end_matches('\0').to_string()
        }),
        REG_DWORD if data.len() >= 4 => {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&data[..4]);
            RegistryValue::Dword(u32::from_le_bytes(bytes))
        }
        REG_MULTI_SZ => RegistryValue::MultiString(if wide {
            decode_wide_multi_string(&data)
        } else {
            let text = String::from_utf8_lossy(&data);
            text.split('\0')
                .filter(|item| !item.is_empty())
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
        }),
        REG_BINARY => RegistryValue::Binary(data),
        _ => return ERROR_SUCCESS,
    };
    let redirected = redirect_wow6432_key(vm, &key);
    let registry = match get_registry_mut(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    let _ = registry.set(&key, value.clone());
    if let Some(redirected) = redirected {
        let _ = registry.set(&redirected, value);
    }
    ERROR_SUCCESS
}

fn encode_wide_string(value: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes
}

fn encode_wide_multi_string(values: &[String]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in values {
        for unit in value.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        bytes.extend_from_slice(&0u16.to_le_bytes());
    }
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes
}

fn decode_wide_string(bytes: &[u8]) -> String {
    let mut units = decode_wide_units(bytes);
    if let Some(end) = units.iter().position(|unit| *unit == 0) {
        units.truncate(end);
    }
    String::from_utf16_lossy(&units)
}

fn decode_wide_multi_string(bytes: &[u8]) -> Vec<String> {
    let units = decode_wide_units(bytes);
    let mut values = Vec::new();
    let mut current = Vec::new();
    for unit in units {
        if unit == 0 {
            if current.is_empty() {
                break;
            }
            values.push(String::from_utf16_lossy(&current));
            current.clear();
        } else {
            current.push(unit);
        }
    }
    if !current.is_empty() {
        values.push(String::from_utf16_lossy(&current));
    }
    values
}

fn decode_wide_units(bytes: &[u8]) -> Vec<u16> {
    let mut units = Vec::with_capacity(bytes.len() / 2);
    let mut chunks = bytes.chunks_exact(2);
    for chunk in &mut chunks {
        units.push(u16::from_le_bytes([chunk[0], chunk[1]]));
    }
    units
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

fn reg_delete_value_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    ERROR_SUCCESS
}

fn reg_delete_key_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    ERROR_SUCCESS
}

fn reg_delete_key_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    ERROR_SUCCESS
}

fn reg_enum_key_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    reg_enum_key_ex(_vm, _stack_ptr, "RegEnumKeyExA", false)
}

fn reg_enum_key_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_enum_key_ex(vm, stack_ptr, "RegEnumKeyExW", true)
}

fn reg_query_info_key_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_query_info_key(vm, stack_ptr, "RegQueryInfoKeyA", false)
}

fn reg_query_info_key_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    reg_query_info_key(vm, stack_ptr, "RegQueryInfoKeyW", true)
}

fn reg_enum_key_ex(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let index = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let name_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let name_len_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let class_ptr = vm.read_u32(stack_ptr + 24).unwrap_or(0);
    let class_len_ptr = vm.read_u32(stack_ptr + 28).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr + 32).unwrap_or(0);
    if name_len_ptr == 0 {
        return ERROR_FILE_NOT_FOUND;
    }
    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let query_prefix = redirect_wow6432_key(vm, &prefix).unwrap_or(prefix);
    let registry = match get_registry(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    let subkeys = match registry.subkeys(&query_prefix) {
        Ok(values) => values,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    if index >= subkeys.len() {
        return ERROR_NO_MORE_ITEMS;
    }
    let name = &subkeys[index];
    let required_len = if wide {
        name.encode_utf16().count()
    } else {
        name.len()
    };
    let buffer_len = vm.read_u32(name_len_ptr).unwrap_or(0) as usize;
    if buffer_len <= required_len {
        let _ = vm.write_u32(name_len_ptr, required_len as u32);
        return ERROR_MORE_DATA;
    }
    if name_ptr == 0 {
        return ERROR_FILE_NOT_FOUND;
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] {api}: {query_prefix} index={index} name={name}");
    }
    if wide {
        let mut cursor = name_ptr;
        for unit in name.encode_utf16() {
            let _ = vm.write_u16(cursor, unit);
            cursor = cursor.wrapping_add(2);
        }
        let _ = vm.write_u16(cursor, 0);
    } else {
        let mut bytes = name.as_bytes().to_vec();
        bytes.push(0);
        let _ = vm.write_bytes(name_ptr, &bytes);
    }
    let _ = vm.write_u32(name_len_ptr, required_len as u32);
    if class_len_ptr != 0 {
        let _ = vm.write_u32(class_len_ptr, 0);
    }
    if class_ptr != 0 {
        if wide {
            let _ = vm.write_u16(class_ptr, 0);
        } else {
            let _ = vm.write_u8(class_ptr, 0);
        }
    }
    if filetime_ptr != 0 {
        let _ = vm.write_u32(filetime_ptr, 0);
        let _ = vm.write_u32(filetime_ptr + 4, 0);
    }
    ERROR_SUCCESS
}

fn reg_query_info_key(vm: &mut Vm, stack_ptr: u32, api: &str, wide: bool) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let class_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let class_len_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let subkeys_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let max_subkey_ptr = vm.read_u32(stack_ptr + 24).unwrap_or(0);
    let max_class_ptr = vm.read_u32(stack_ptr + 28).unwrap_or(0);
    let values_ptr = vm.read_u32(stack_ptr + 32).unwrap_or(0);
    let max_value_ptr = vm.read_u32(stack_ptr + 36).unwrap_or(0);
    let max_value_len_ptr = vm.read_u32(stack_ptr + 40).unwrap_or(0);
    let security_ptr = vm.read_u32(stack_ptr + 44).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr + 48).unwrap_or(0);

    let prefix = match registry_prefix(vm, hkey) {
        Ok(value) => value,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };
    let query_prefix = redirect_wow6432_key(vm, &prefix).unwrap_or(prefix);
    let registry = match get_registry(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    let stats = match registry.stats(&query_prefix, wide) {
        Ok(stats) => stats,
        Err(_) => return ERROR_FILE_NOT_FOUND,
    };

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] {api}: {query_prefix} subkeys={} values={}",
            stats.subkey_count, stats.value_count
        );
    }
    if subkeys_ptr != 0 {
        let _ = vm.write_u32(subkeys_ptr, stats.subkey_count);
    }
    if max_subkey_ptr != 0 {
        let _ = vm.write_u32(max_subkey_ptr, stats.max_subkey_len);
    }
    if values_ptr != 0 {
        let _ = vm.write_u32(values_ptr, stats.value_count);
    }
    if max_value_ptr != 0 {
        let _ = vm.write_u32(max_value_ptr, stats.max_value_name_len);
    }
    if max_value_len_ptr != 0 {
        let _ = vm.write_u32(max_value_len_ptr, stats.max_value_len);
    }
    if max_class_ptr != 0 {
        let _ = vm.write_u32(max_class_ptr, 0);
    }
    if class_len_ptr != 0 {
        let _ = vm.write_u32(class_len_ptr, 0);
    }
    if class_ptr != 0 {
        if wide {
            let _ = vm.write_u16(class_ptr, 0);
        } else {
            let _ = vm.write_u8(class_ptr, 0);
        }
    }
    if security_ptr != 0 {
        let _ = vm.write_u32(security_ptr, 0);
    }
    if filetime_ptr != 0 {
        let _ = vm.write_u32(filetime_ptr, 0);
        let _ = vm.write_u32(filetime_ptr + 4, 0);
    }
    ERROR_SUCCESS
}
