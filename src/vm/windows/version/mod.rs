//! VERSION.dll stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "VERSION.dll",
        "GetFileVersionInfoSizeA",
        crate::vm::stdcall_args(2),
        get_file_version_info_size_a,
    );
    vm.register_import_stdcall(
        "VERSION.dll",
        "GetFileVersionInfoA",
        crate::vm::stdcall_args(4),
        get_file_version_info_a,
    );
    vm.register_import_stdcall("VERSION.dll", "VerQueryValueA", crate::vm::stdcall_args(4), ver_query_value_a);
}

fn get_file_version_info_size_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if handle_ptr != 0 {
        let _ = vm.write_u32(handle_ptr, 0);
    }
    0
}

fn get_file_version_info_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn ver_query_value_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
