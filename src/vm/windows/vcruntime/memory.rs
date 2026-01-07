//! VCRUNTIME memory stubs.

use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import("VCRUNTIME140.dll", "memset", memset);
}

fn memset(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dest, value, size) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let value = value as u8;
    let size = size as usize;
    if dest == 0 {
        return 0;
    }
    for offset in 0..size {
        if vm
            .write_u8(dest.wrapping_add(offset as u32), value)
            .is_err()
        {
            break;
        }
    }
    dest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};
    use crate::vm_set_args;

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
    fn test_memset_null_dest() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm_set_args!(vm, stack; 0u32, 0xFFu32, 10u32);
        let result = memset(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_memset_fills_memory() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let dest = vm.heap_start as u32;
        vm_set_args!(vm, stack; dest, 0xABu32, 5u32);
        let result = memset(&mut vm, stack);
        assert_eq!(result, dest);
        // Check memory was filled
        for i in 0..5 {
            assert_eq!(vm.read_u8(dest + i).unwrap(), 0xAB);
        }
    }

    #[test]
    fn test_memset_returns_dest() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let dest = vm.heap_start as u32;
        vm_set_args!(vm, stack; dest, 0u32, 1u32);
        let result = memset(&mut vm, stack);
        assert_eq!(result, dest);
    }
}
