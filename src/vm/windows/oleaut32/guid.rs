//! GUID helpers for COM interfaces.

use crate::vm::windows::guid::parse_guid;
use crate::vm::Vm;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_read_guid_bytes() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        let guid_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        vm.write_bytes(ptr, &guid_bytes).unwrap();
        let result = read_guid_bytes(&vm, ptr).unwrap();
        assert_eq!(result, guid_bytes);
    }

    #[test]
    fn test_guid_matches_valid() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // IUnknown GUID: {00000000-0000-0000-C000-000000000046}
        let guid_bytes: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x46,
        ];
        vm.write_bytes(ptr, &guid_bytes).unwrap();
        assert!(guid_matches(
            &vm,
            ptr,
            "{00000000-0000-0000-C000-000000000046}"
        ));
    }

    #[test]
    fn test_guid_matches_mismatch() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        let guid_bytes: [u8; 16] = [0; 16];
        vm.write_bytes(ptr, &guid_bytes).unwrap();
        assert!(!guid_matches(
            &vm,
            ptr,
            "{00000001-0000-0000-0000-000000000000}"
        ));
    }

    #[test]
    fn test_guid_matches_invalid_string() {
        let vm = create_test_vm();
        // Invalid GUID string should return false
        assert!(!guid_matches(&vm, 0, "not-a-guid"));
    }
}
