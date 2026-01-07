//! VCRUNTIME runtime stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "VCRUNTIME140.dll",
        "__std_type_info_destroy_list",
        std_type_info_destroy,
    );
    vm.register_import(
        "VCRUNTIME140.dll",
        "_except_handler4_common",
        except_handler4_common,
    );
}

fn std_type_info_destroy(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn except_handler4_common(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
