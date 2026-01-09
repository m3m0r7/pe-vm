//! Utility helpers for WinINet stubs.

use crate::vm::Vm;

pub(super) fn read_optional_string(vm: &Vm, ptr: u32, len: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    if len == 0 || len == 0xFFFF_FFFF {
        return read_wide_or_utf16le_str!(vm, ptr);
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

pub(super) fn parse_host(host: &str) -> (String, bool) {
    let trimmed = host.trim();
    let mut secure = false;
    let trimmed = if let Some(rest) = trimmed.strip_prefix("https://") {
        secure = true;
        rest
    } else {
        trimmed
    };
    let trimmed = trimmed.strip_prefix("http://").unwrap_or(trimmed);
    let trimmed = trimmed.trim_end_matches('/');
    (trimmed.to_string(), secure)
}

pub(super) fn network_fallback_host(vm: &Vm) -> Option<&str> {
    vm.config()
        .sandbox_config()
        .and_then(|sandbox| sandbox.network_fallback_host())
}

pub(super) fn ensure_host_header(headers: &str, host: &str) -> String {
    if host.is_empty() {
        return headers.to_string();
    }
    let mut out_lines = Vec::new();
    let mut host_set = false;
    for line in headers.split('\n') {
        let trimmed = line.trim_end_matches('\r');
        if trimmed.to_ascii_lowercase().starts_with("host:") {
            let value = trimmed["host:".len()..].trim();
            if value.is_empty() && !host_set {
                out_lines.push(format!("Host: {host}"));
                host_set = true;
            } else {
                out_lines.push(trimmed.to_string());
                if !value.is_empty() {
                    host_set = true;
                }
            }
            continue;
        }
        if !trimmed.is_empty() {
            out_lines.push(trimmed.to_string());
        }
    }
    if !host_set {
        out_lines.push(format!("Host: {host}"));
    }
    let mut joined = out_lines.join("\r\n");
    if !joined.is_empty() {
        joined.push_str("\r\n");
    }
    joined
}
