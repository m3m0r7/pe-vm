//! Kernel32 SLIST stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "InitializeSListHead",
        crate::vm::stdcall_args(1),
        initialize_slist_head,
    );
}

fn initialize_slist_head(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let header = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if header != 0 {
        let _ = vm.write_u32(header, 0);
        let _ = vm.write_u32(header.wrapping_add(4), 0);
    }
    0
}
