use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "GetFileInformationByHandleExW",
        crate::vm::stdcall_args(4),
        get_file_information_by_handle_ex_w,
    );
    vm.register_import_any_stdcall(
        "SetFileInformationByHandleW",
        crate::vm::stdcall_args(4),
        set_file_information_by_handle_w,
    );
}

fn get_file_information_by_handle_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_ptr, size) = vm_args!(vm, stack_ptr; u32, usize);
    if info_ptr == 0 || size == 0 {
        return 0;
    }
    let _ = vm.write_bytes(info_ptr, &vec![0u8; size]);
    1
}

fn set_file_information_by_handle_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
