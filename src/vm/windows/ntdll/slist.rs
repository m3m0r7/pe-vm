//! NTDLL SLIST stubs.

use crate::vm::windows::ntdll::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "RtlInitializeSListHead", crate::vm::stdcall_args(1), rtl_initialize_slist_head);
}

fn rtl_initialize_slist_head(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (header,) = vm_args!(vm, stack_ptr; u32);
    if header != 0 {
        let _ = vm.write_u32(header, 0);
        let _ = vm.write_u32(header.wrapping_add(4), 0);
    }
    0
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
    fn test_rtl_initialize_slist_head_null_header() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap(); // null header
        let result = rtl_initialize_slist_head(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_rtl_initialize_slist_head_clears_header() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let header = vm.heap_start as u32;
        // Initialize header with non-zero values
        vm.write_u32(header, 0xDEADBEEF).unwrap();
        vm.write_u32(header + 4, 0xCAFEBABE).unwrap();
        vm.write_u32(stack + 4, header).unwrap();
        let result = rtl_initialize_slist_head(&mut vm, stack);
        assert_eq!(result, 0);
        // Header should be zeroed
        assert_eq!(vm.read_u32(header).unwrap(), 0);
        assert_eq!(vm.read_u32(header + 4).unwrap(), 0);
    }
}
