//! Miscellaneous User32 helpers.

use crate::vm::Vm;

// Register smaller helpers that don't warrant their own module.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("USER32.dll", "EnableWindow", crate::vm::stdcall_args(2), enable_window);
    vm.register_import_stdcall("USER32.dll", "CharNextA", crate::vm::stdcall_args(1), char_next_a);
    vm.register_import_stdcall("USER32.dll", "CharNextW", crate::vm::stdcall_args(1), char_next_w);
}

fn enable_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn char_next_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return 0;
    }
    ptr.wrapping_add(1)
}

fn char_next_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return 0;
    }
    ptr.wrapping_add(2)
}
