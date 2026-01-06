//! Helper functions for registry stubs.

use crate::vm::windows::registry::RegistryValue;
use crate::vm::{Architecture, Vm, VmError};

use super::constants::{
    ERROR_SUCCESS, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    HKEY_USERS,
};

pub(super) fn write_zero_counts(vm: &mut Vm, stack_ptr: u32) -> u32 {
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

pub(super) fn registry_prefix(vm: &Vm, hkey: u32) -> Result<String, VmError> {
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

pub(super) fn join_key(prefix: &str, subkey: &str) -> String {
    if subkey.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}\\{subkey}")
    }
}

pub(super) fn format_registry_key(prefix: &str, value_name: Option<&str>) -> String {
    match value_name {
        Some(name) => format!("{prefix}@{name}"),
        None => prefix.to_string(),
    }
}

pub(super) fn resolve_registry_value<'a>(
    vm: &Vm,
    registry: &'a crate::vm::windows::registry::Registry,
    key: &str,
) -> Option<&'a RegistryValue> {
    if let Ok(Some(value)) = registry.get(key) {
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] RegQueryValueExA hit: {key}");
        }
        return Some(value);
    }
    let redirected = redirect_wow6432_key(vm, key)?;
    match registry.get(&redirected) {
        Ok(Some(value)) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] RegQueryValueExA redirect: {key} -> {redirected}");
            }
            Some(value)
        }
        _ => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] RegQueryValueExA miss: {key} (redirected {redirected})");
            }
            None
        }
    }
}

// Map 32-bit registry access to WOW6432Node when present.
fn redirect_wow6432_key(vm: &Vm, key: &str) -> Option<String> {
    if vm.config().architecture != Architecture::X86 {
        return None;
    }
    let (base, value_name) = match key.split_once('@') {
        Some((base, name)) => (base, Some(name)),
        None => (key, None),
    };
    let mut parts: Vec<&str> = base.split('\\').filter(|part| !part.is_empty()).collect();
    if parts.len() < 2 {
        return None;
    }
    let hive = parts[0];
    let is_hklm = hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE");
    let is_hkcu = hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER");
    if !(is_hklm || is_hkcu) {
        return None;
    }
    if !parts[1].eq_ignore_ascii_case("Software") {
        return None;
    }
    if parts.len() > 2 && parts[2].eq_ignore_ascii_case("WOW6432Node") {
        return None;
    }
    parts.insert(2, "WOW6432Node");
    let mut redirected = parts.join("\\");
    if let Some(name) = value_name {
        redirected.push('@');
        redirected.push_str(name);
    }
    Some(redirected)
}

pub(super) fn read_bytes(vm: &Vm, ptr: u32, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(vm.read_u8(ptr + i as u32).unwrap_or(0));
    }
    out
}

pub(super) fn read_raw_bytes(vm: &Vm, ptr: u32, len: usize) -> String {
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

pub(super) fn read_raw_ascii(vm: &Vm, ptr: u32, len: usize) -> String {
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

pub(super) fn read_string_arg(vm: &Vm, ptr: u32) -> String {
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

pub(super) fn is_root_hive(hkey: u32) -> bool {
    matches!(
        hkey,
        HKEY_CLASSES_ROOT | HKEY_CURRENT_USER | HKEY_LOCAL_MACHINE | HKEY_USERS | HKEY_CURRENT_CONFIG
    )
}
