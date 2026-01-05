//! NTDLL PEB-related stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("ntdll.dll", "RtlGetCurrentPeb", crate::vm::stdcall_args(0), rtl_get_current_peb);
}

fn rtl_get_current_peb(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
