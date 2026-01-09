//! Minimal MFCN40D.dll ordinal stubs.

use crate::vm::Vm;

const DLL_NAME: &str = "MFCN40D.DLL";

pub fn register(vm: &mut Vm) {
    vm.register_import_ordinal(DLL_NAME, 283, mfcn_stub);
}

fn mfcn_stub(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
