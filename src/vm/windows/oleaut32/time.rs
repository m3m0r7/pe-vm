//! Time conversion stubs.

use crate::vm::Vm;
use crate::vm_args;

// SystemTimeToVariantTime(...)
pub(super) fn system_time_to_variant_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out) = vm_args!(vm, stack_ptr; u32, u32);
    if out == 0 {
        return 0;
    }
    let bytes = 0f64.to_le_bytes();
    let _ = vm.write_bytes(out, &bytes);
    1
}

// VariantTimeToSystemTime(DOUBLE vtime, SYSTEMTIME *lpSystemTime)
// DOUBLE is 8 bytes, pointer is 4 bytes = 12 bytes total
pub(super) fn variant_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Skip 4 bytes for return address, then DOUBLE (vtime) is 8 bytes, then pointer
    if std::env::var("PE_VM_TRACE").is_ok() {
        // Dump more of the stack to understand the layout
        let mut stack_dump = String::new();
        for i in 0..8u32 {
            let val = vm.read_u32(stack_ptr + i * 4).unwrap_or(0xDEADBEEF);
            stack_dump.push_str(&format!(" +0x{:02X}=0x{val:08X}", i * 4));
        }
        let esp = vm.reg32(crate::vm::REG_ESP);
        eprintln!(
            "[pe_vm] VariantTimeToSystemTime: stack_ptr=0x{stack_ptr:08X} esp=0x{esp:08X}{stack_dump}"
        );
    }
    let out = vm.read_u32(stack_ptr + 4 + 8).unwrap_or(0);
    if out == 0 {
        // Return success even if output pointer is NULL.
        // Some callers may not need the result.
        return 1;
    }
    // Write a valid SYSTEMTIME structure (16 bytes)
    // Default to 2001/01/01 00:00:00
    let systemtime: [u8; 16] = [
        0xD1, 0x07, // wYear = 2001
        0x01, 0x00, // wMonth = 1
        0x01, 0x00, // wDayOfWeek = 1 (Monday)
        0x01, 0x00, // wDay = 1
        0x00, 0x00, // wHour = 0
        0x00, 0x00, // wMinute = 0
        0x00, 0x00, // wSecond = 0
        0x00, 0x00, // wMilliseconds = 0
    ];
    let _ = vm.write_bytes(out, &systemtime);
    1
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
    fn test_system_time_to_variant_time_null_out() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 8, 0).unwrap(); // null output
        let result = system_time_to_variant_time(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_system_time_to_variant_time_success() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let out_ptr = vm.heap_start as u32;
        vm.write_u32(stack + 8, out_ptr).unwrap();
        let result = system_time_to_variant_time(&mut vm, stack);
        assert_eq!(result, 1);
        // Should write 0.0 as f64 (8 bytes of zeros)
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = vm.read_u8(out_ptr + i as u32).unwrap();
        }
        assert_eq!(f64::from_le_bytes(bytes), 0.0);
    }

    #[test]
    fn test_variant_time_to_system_time_null_out() {
        let mut vm = create_test_vm();
        // Stack layout: [ret_addr(4)] [double vtime(8)] [ptr lpSystemTime(4)]
        let stack = vm.stack_top - 16;
        // Write null to lpSystemTime at offset 12 (4 + 8)
        vm.write_u32(stack + 12, 0).unwrap();
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_variant_time_to_system_time_success() {
        let mut vm = create_test_vm();
        // Stack layout: [ret_addr(4)] [double vtime(8)] [ptr lpSystemTime(4)]
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        // Fill with non-zero
        for i in 0..16 {
            vm.write_u8(out_ptr + i, 0xFF).unwrap();
        }
        // Write output pointer at offset 12 (4 + 8)
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 1);
        // Should write SYSTEMTIME struct (wYear=2001, wMonth=1, etc.)
        let year = vm.read_u16(out_ptr).unwrap();
        assert_eq!(year, 2001);
    }
}
