//! UCRT environment stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_initialize_narrow_environment",
        initialize_narrow_environment,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_seh_filter_dll",
        seh_filter_dll,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_configure_narrow_argv",
        configure_narrow_argv,
    );
}

fn configure_narrow_argv(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn initialize_narrow_environment(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn seh_filter_dll(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
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
    fn test_configure_narrow_argv_returns_zero() {
        let mut vm = create_test_vm();
        let result = configure_narrow_argv(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_initialize_narrow_environment_returns_zero() {
        let mut vm = create_test_vm();
        let result = initialize_narrow_environment(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_seh_filter_dll_returns_zero() {
        let mut vm = create_test_vm();
        let result = seh_filter_dll(&mut vm, 0);
        assert_eq!(result, 0);
    }
}
