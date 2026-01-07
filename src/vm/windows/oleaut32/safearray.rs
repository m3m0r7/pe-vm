//! SafeArray stubs.

use crate::vm::Vm;

use super::constants::S_OK;

// SafeArrayCreate(...)
pub(super) fn safe_array_create(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// SafeArrayAccessData(...)
pub(super) fn safe_array_access_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// SafeArrayUnaccessData(...)
pub(super) fn safe_array_unaccess_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
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
    fn test_safe_array_create_returns_null() {
        let mut vm = create_test_vm();
        let result = safe_array_create(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_safe_array_access_data_returns_s_ok() {
        let mut vm = create_test_vm();
        let result = safe_array_access_data(&mut vm, 0);
        assert_eq!(result, S_OK);
    }

    #[test]
    fn test_safe_array_unaccess_data_returns_s_ok() {
        let mut vm = create_test_vm();
        let result = safe_array_unaccess_data(&mut vm, 0);
        assert_eq!(result, S_OK);
    }
}
