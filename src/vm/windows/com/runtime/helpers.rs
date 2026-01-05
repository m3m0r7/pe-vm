//! Helper utilities for COM runtime tasks.

use crate::vm::{Vm, VmError};
use crate::vm::windows::guid::parse_guid;

pub(super) fn alloc_guid(vm: &mut Vm, guid: &str) -> Result<u32, VmError> {
    let bytes = parse_guid(guid).ok_or(VmError::InvalidConfig("invalid GUID"))?;
    vm.alloc_bytes(&bytes, 4)
}

// Normalize CLSID strings to a consistent form used by registry lookups.
pub(super) fn normalize_clsid(clsid: &str) -> String {
    let trimmed = clsid.trim();
    let trimmed = trimmed.trim_start_matches('{').trim_end_matches('}');
    format!("{{{}}}", trimmed.to_ascii_uppercase())
}

pub(super) fn read_hex_window(vm: &Vm, addr: u32, len: usize) -> String {
    let mut out = String::new();
    for idx in 0..len {
        let value = vm.read_u8(addr.wrapping_add(idx as u32)).unwrap_or(0);
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{value:02X}"));
    }
    out
}
