//! ADVAPI32 registry stubs.

use crate::vm::windows::registry::RegistryValue;
use crate::vm::windows::{get_registry, get_registry_mut};
use crate::vm::{Vm, VmError};

const ERROR_SUCCESS: u32 = 0;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_MORE_DATA: u32 = 234;
const ERROR_NO_MORE_ITEMS: u32 = 259;

const REG_SZ: u32 = 1;
const REG_BINARY: u32 = 3;
const REG_DWORD: u32 = 4;
const REG_MULTI_SZ: u32 = 7;

const HKEY_CLASSES_ROOT: u32 = 0x8000_0000;
const HKEY_CURRENT_USER: u32 = 0x8000_0001;
const HKEY_LOCAL_MACHINE: u32 = 0x8000_0002;
const HKEY_USERS: u32 = 0x8000_0003;
const HKEY_CURRENT_CONFIG: u32 = 0x8000_0005;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("ADVAPI32.dll", "RegOpenKeyExA", crate::vm::stdcall_args(5), reg_open_key_ex_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegCreateKeyExA", crate::vm::stdcall_args(9), reg_create_key_ex_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegQueryValueExA", crate::vm::stdcall_args(6), reg_query_value_ex_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegSetValueExA", crate::vm::stdcall_args(6), reg_set_value_ex_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegCloseKey", crate::vm::stdcall_args(1), reg_close_key);
    vm.register_import_stdcall("ADVAPI32.dll", "RegDeleteValueA", crate::vm::stdcall_args(2), reg_delete_value_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegDeleteKeyA", crate::vm::stdcall_args(2), reg_delete_key_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegEnumKeyExA", crate::vm::stdcall_args(8), reg_enum_key_ex_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegQueryInfoKeyA", crate::vm::stdcall_args(12), reg_query_info_key_a);
    vm.register_import_stdcall("ADVAPI32.dll", "RegQueryInfoKeyW", crate::vm::stdcall_args(12), reg_query_info_key_w);
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
    let value = match registry.get(&key) {
        Ok(Some(value)) => value,
        _ => return ERROR_FILE_NOT_FOUND,
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
    if size_ptr != 0 {
        let _ = vm.write_u32(size_ptr, bytes.len() as u32);
    }
    if data_ptr == 0 {
        return ERROR_SUCCESS;
    }
    let buffer_size = if size_ptr == 0 {
        bytes.len()
    } else {
        vm.read_u32(size_ptr).unwrap_or(bytes.len() as u32) as usize
    };
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
    if std::env::var("PE_VM_TRACE").is_ok() {
        let name = value_name.as_deref().unwrap_or("(Default)");
        eprintln!("[pe_vm] RegSetValueExA: {key} ({name}) = {text}");
    }

    let registry = match get_registry_mut(vm) {
        Some(value) => value,
        None => return ERROR_FILE_NOT_FOUND,
    };
    if registry
        .set(&key, RegistryValue::String(text))
        .is_err()
    {
        return ERROR_FILE_NOT_FOUND;
    }
    ERROR_SUCCESS
}

fn reg_close_key(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hkey = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if !is_root_hive(hkey) {
        vm.registry_close_handle(hkey);
    }
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

fn write_zero_counts(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let subkeys_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let max_subkey_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let values_ptr = vm.read_u32(stack_ptr + 24).unwrap_or(0);
    let max_value_ptr = vm.read_u32(stack_ptr + 28).unwrap_or(0);
    let _ = write_optional_u32(vm, subkeys_ptr, 0);
    let _ = write_optional_u32(vm, max_subkey_ptr, 0);
    let _ = write_optional_u32(vm, values_ptr, 0);
    let _ = write_optional_u32(vm, max_value_ptr, 0);
    ERROR_SUCCESS
}

fn write_optional_u32(vm: &mut Vm, ptr: u32, value: u32) -> Result<(), VmError> {
    if ptr == 0 {
        return Ok(());
    }
    vm.write_u32(ptr, value)
}

fn registry_prefix(vm: &Vm, hkey: u32) -> Result<String, VmError> {
    if let Some(path) = vm.registry_handle_path(hkey) {
        return Ok(path.to_string());
    }
    let prefix = match hkey {
        HKEY_CLASSES_ROOT => "HKCR",
        HKEY_CURRENT_USER => "HKCU",
        HKEY_LOCAL_MACHINE => "HKLM",
        HKEY_USERS => "HKU",
        HKEY_CURRENT_CONFIG => "HKCC",
        _ => return Err(VmError::InvalidConfig("unknown registry hive")),
    };
    Ok(prefix.to_string())
}

fn join_key(prefix: &str, subkey: &str) -> String {
    if subkey.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}\\{subkey}")
    }
}

fn format_registry_key(prefix: &str, value_name: Option<&str>) -> String {
    match value_name {
        Some(name) => format!("{prefix}@{name}"),
        None => prefix.to_string(),
    }
}

fn read_bytes(vm: &Vm, ptr: u32, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(vm.read_u8(ptr + i as u32).unwrap_or(0));
    }
    out
}

fn read_raw_bytes(vm: &Vm, ptr: u32, len: usize) -> String {
    let mut out = String::new();
    for i in 0..len {
        let byte = vm.read_u8(ptr.wrapping_add(i as u32)).unwrap_or(0);
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02X}"));
    }
    out
}

fn read_raw_ascii(vm: &Vm, ptr: u32, len: usize) -> String {
    let mut out = String::new();
    for i in 0..len {
        let byte = vm.read_u8(ptr.wrapping_add(i as u32)).unwrap_or(0);
        if (0x20..0x7F).contains(&byte) {
            out.push(byte as char);
        } else {
            out.push('.');
        }
    }
    out
}

fn read_string_arg(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    let b0 = vm.read_u8(ptr).unwrap_or(0);
    let b1 = vm.read_u8(ptr.wrapping_add(1)).unwrap_or(0);
    let b2 = vm.read_u8(ptr.wrapping_add(2)).unwrap_or(0);
    let b3 = vm.read_u8(ptr.wrapping_add(3)).unwrap_or(0);
    let looks_wide = b0 != 0 && b1 == 0 && (b2 != 0 || b3 == 0);
    if looks_wide {
        let wide = read_w_string(vm, ptr);
        if !wide.is_empty() {
            return wide;
        }
    }
    let ascii = vm.read_c_string(ptr).unwrap_or_default();
    if !ascii.is_empty() {
        return ascii;
    }
    let mut found = None;
    for offset in 0..64u32 {
        let byte = vm.read_u8(ptr.wrapping_add(offset)).unwrap_or(0);
        if byte == 0 {
            continue;
        }
        let mut bytes = Vec::new();
        for i in 0..64u32 {
            let next = vm.read_u8(ptr.wrapping_add(offset + i)).unwrap_or(0);
            if next == 0 {
                break;
            }
            bytes.push(next);
        }
        if !bytes.is_empty() {
            found = Some(String::from_utf8_lossy(&bytes).to_string());
            break;
        }
    }
    if let Some(value) = found {
        if let Some(prefix) = scan_prefix(vm, ptr) {
            return format!("{prefix}{value}");
        }
        return value;
    }
    String::new()
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

fn scan_prefix(vm: &Vm, ptr: u32) -> Option<String> {
    let window_start = ptr.wrapping_sub(128);
    let mut current = Vec::new();
    let mut last_prefix = None;
    for i in 0..256u32 {
        let byte = vm.read_u8(window_start.wrapping_add(i)).unwrap_or(0);
        if (0x20..0x7F).contains(&byte) {
            current.push(byte);
            continue;
        }
        if !current.is_empty() {
            let text = String::from_utf8_lossy(&current).to_string();
            if text.contains('\\') && text.ends_with('\\') {
                last_prefix = Some(text);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        let text = String::from_utf8_lossy(&current).to_string();
        if text.contains('\\') && text.ends_with('\\') {
            last_prefix = Some(text);
        }
    }
    last_prefix
}

fn is_root_hive(hkey: u32) -> bool {
    matches!(
        hkey,
        HKEY_CLASSES_ROOT | HKEY_CURRENT_USER | HKEY_LOCAL_MACHINE | HKEY_USERS | HKEY_CURRENT_CONFIG
    )
}
