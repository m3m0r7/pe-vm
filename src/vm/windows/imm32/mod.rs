//! IMM32 stubs for input method APIs.

use crate::vm::Vm;

// Register minimal IMM32 entry points used by GUI DLLs.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "IMM32.dll",
        "ImmAssociateContext",
        crate::vm::stdcall_args(2),
        imm_associate_context,
    );
}

fn imm_associate_context(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
