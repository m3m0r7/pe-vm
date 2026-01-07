//! NTDLL PEB-related stubs.

use crate::register_func_stub;
use crate::vm::windows::ntdll::DLL_NAME;
use crate::vm::Vm;

register_func_stub!(DLL_NAME, rtl_get_current_peb, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "RtlGetCurrentPeb", crate::vm::stdcall_args(0), rtl_get_current_peb);
}
