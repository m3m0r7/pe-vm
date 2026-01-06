//! Helper functions for registry stubs.

use crate::vm::windows::registry::RegistryValue;
use crate::vm::{Architecture, Vm, VmError};

use super::constants::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};

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
            eprintln!("[pe_vm] RegQueryValueEx hit: {key}");
        }
        return Some(value);
    }
    let redirected = redirect_wow6432_key(vm, key)?;
    match registry.get(&redirected) {
        Ok(Some(value)) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] RegQueryValueEx redirect: {key} -> {redirected}");
            }
            Some(value)
        }
        _ => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] RegQueryValueEx miss: {key} (redirected {redirected})");
            }
            None
        }
    }
}

// Map 32-bit registry access to WOW6432Node when present.
pub(super) fn redirect_wow6432_key(vm: &Vm, key: &str) -> Option<String> {
    if vm.config().architecture_value() != Architecture::X86 {
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

pub(super) fn read_string_arg_a(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    vm.read_c_string(ptr).unwrap_or_default()
}

pub(super) fn read_string_arg_w(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    read_w_string(vm, ptr)
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

pub(super) fn is_root_hive(hkey: u32) -> bool {
    matches!(
        hkey,
        HKEY_CLASSES_ROOT | HKEY_CURRENT_USER | HKEY_LOCAL_MACHINE | HKEY_USERS | HKEY_CURRENT_CONFIG
    )
}
