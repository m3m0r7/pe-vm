//! IMM32 stubs for input method APIs.

use crate::vm::Vm;
use crate::vm::windows::check_stub;

// Register minimal IMM32 entry points used by GUI DLLs.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "IMM32.dll",
        "ImmAssociateContext",
        crate::vm::stdcall_args(2),
        imm_associate_context,
    );
}

fn imm_associate_context(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    check_stub(vm, "IMM32.dll", "ImmAssociateContext");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BypassSettings;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut bypass = BypassSettings::new();
        bypass.not_implemented_module = true;
        let mut vm = Vm::new(
            VmConfig::new()
                .architecture(Architecture::X86)
                .bypass(bypass),
        )
        .expect("vm");
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
    fn test_imm_associate_context_returns_zero() {
        let mut vm = create_test_vm();
        let result = imm_associate_context(&mut vm, 0);
        // Returns NULL (previous IMC handle)
        assert_eq!(result, 0);
    }
}
