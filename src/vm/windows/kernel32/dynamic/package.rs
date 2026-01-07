use crate::vm::Vm;
use crate::vm_args;

const APPMODEL_ERROR_NO_PACKAGE: u32 = 15_700;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "GetCurrentPackageId",
        crate::vm::stdcall_args(2),
        get_current_package_id,
    );
}

fn get_current_package_id(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [len_ptr] = vm_args!(vm, stack_ptr; u32);
    if len_ptr != 0 {
        let _ = vm.write_u32(len_ptr, 0);
    }
    APPMODEL_ERROR_NO_PACKAGE
}
