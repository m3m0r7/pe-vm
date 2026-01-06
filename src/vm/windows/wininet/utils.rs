//! Utility helpers for WinINet stubs.

use crate::vm::Vm;

pub(super) fn read_c_string(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    vm.read_c_string(ptr).unwrap_or_default()
}

pub(super) fn read_optional_string(vm: &Vm, ptr: u32, len: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    if len == 0 || len == 0xFFFF_FFFF {
        return read_c_string(vm, ptr);
    }
    let mut bytes = Vec::with_capacity(len as usize);
    for offset in 0..len {
        if let Ok(value) = vm.read_u8(ptr.wrapping_add(offset)) {
            if value == 0 {
                break;
            }
            bytes.push(value);
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}

pub(super) fn read_optional_bytes(vm: &Vm, ptr: u32, len: usize) -> Vec<u8> {
    if ptr == 0 || len == 0 {
        return Vec::new();
    }
    let mut bytes = Vec::with_capacity(len);
    for offset in 0..len {
        if let Ok(value) = vm.read_u8(ptr.wrapping_add(offset as u32)) {
            bytes.push(value);
        }
    }
    bytes
}

pub(super) fn normalize_host(host: &str) -> String {
    let trimmed = host.trim();
    let trimmed = trimmed.strip_prefix("http://").unwrap_or(trimmed);
    let trimmed = trimmed.strip_prefix("https://").unwrap_or(trimmed);
    trimmed.trim_end_matches('/').to_string()
}

pub(super) fn network_fallback_host(vm: &Vm) -> Option<&str> {
    vm.config()
        .sandbox_config()
        .and_then(|sandbox| sandbox.network_fallback_host())
}
