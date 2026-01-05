//! VCRUNTIME memory stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import("VCRUNTIME140.dll", "memset", memset);
}

fn memset(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let value = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0) as u8;
    let size = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0) as usize;
    if dest == 0 {
        return 0;
    }
    for offset in 0..size {
        if vm.write_u8(dest.wrapping_add(offset as u32), value).is_err() {
            break;
        }
    }
    dest
}
