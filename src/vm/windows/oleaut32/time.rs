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

// VariantTimeToSystemTime(...)
pub(super) fn variant_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, out) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out == 0 {
        return 0;
    }
    let _ = vm.write_bytes(out, &[0u8; 16]);
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
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 12, 0).unwrap(); // null output
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_variant_time_to_system_time_success() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        // Fill with non-zero
        for i in 0..16 {
            vm.write_u8(out_ptr + i, 0xFF).unwrap();
        }
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 1);
        // Should write 16 zero bytes (SYSTEMTIME struct)
        for i in 0..16 {
            assert_eq!(vm.read_u8(out_ptr + i).unwrap(), 0);
        }
    }
}
