//! VCRUNTIME memory stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import("VCRUNTIME140.dll", "memset", memset);
}

fn memset(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let value = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0) as u8;
    let size = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0) as usize;
    if dest == 0 {
        return 0;
    }
    for offset in 0..size {
        if vm.write_u8(dest.wrapping_add(offset as u32), value).is_err() {
            break;
        }
    }
    dest
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
    fn test_memset_null_dest() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 4, 0).unwrap(); // null dest
        vm.write_u32(stack + 8, 0xFF).unwrap(); // value
        vm.write_u32(stack + 12, 10).unwrap(); // size
        let result = memset(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_memset_fills_memory() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let dest = vm.heap_start as u32;
        vm.write_u32(stack + 4, dest).unwrap();
        vm.write_u32(stack + 8, 0xAB).unwrap(); // value
        vm.write_u32(stack + 12, 5).unwrap(); // size
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
        vm.write_u32(stack + 4, dest).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, 1).unwrap();
        let result = memset(&mut vm, stack);
        assert_eq!(result, dest);
    }
}
