//! Kernel32 SLIST stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "InitializeSListHead",
        crate::vm::stdcall_args(1),
        initialize_slist_head,
    );
}

fn initialize_slist_head(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (header,) = vm_args!(vm, stack_ptr; u32);
    if header != 0 {
        let _ = vm.write_u32(header, 0);
        let _ = vm.write_u32(header.wrapping_add(4), 0);
    }
    0
}
