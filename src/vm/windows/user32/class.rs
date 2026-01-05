//! User32 class registration stubs.

use crate::vm::Vm;

const WNDCLASSEX_SIZE: u32 = 48;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("USER32.dll", "GetClassInfoExA", crate::vm::stdcall_args(3), get_class_info_ex_a);
    vm.register_import_stdcall("USER32.dll", "GetClassInfoExW", crate::vm::stdcall_args(3), get_class_info_ex_w);
    vm.register_import_stdcall("USER32.dll", "RegisterClassExA", crate::vm::stdcall_args(1), register_class_ex_a);
    vm.register_import_stdcall("USER32.dll", "RegisterClassExW", crate::vm::stdcall_args(1), register_class_ex_w);
}

fn get_class_info_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, WNDCLASSEX_SIZE);
        let _ = vm.write_bytes(out_ptr + 4, &[0u8; 44]);
    }
    1
}

fn get_class_info_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, WNDCLASSEX_SIZE);
        let _ = vm.write_bytes(out_ptr + 4, &[0u8; 44]);
    }
    1
}

fn register_class_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn register_class_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
