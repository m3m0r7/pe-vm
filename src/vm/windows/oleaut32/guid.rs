//! GUID helpers for COM interfaces.

use crate::vm::Vm;
use crate::vm::windows::guid::parse_guid;

pub(super) fn read_guid_bytes(vm: &Vm, ptr: u32) -> Option<[u8; 16]> {
    let mut bytes = [0u8; 16];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr.wrapping_add(idx as u32)).ok()?;
    }
    Some(bytes)
}

pub(super) fn guid_matches(vm: &Vm, ptr: u32, guid: &str) -> bool {
    let Some(expected) = parse_guid(guid) else {
        return false;
    };
    let mut actual = [0u8; 16];
    for (idx, slot) in actual.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr.wrapping_add(idx as u32)).unwrap_or(0);
    }
    actual == expected
}
