use crate::vm::Vm;

use super::constants::INVALID_HANDLE_VALUE;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FindClose",
        crate::vm::stdcall_args(1),
        find_close,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FindFirstFileA",
        crate::vm::stdcall_args(2),
        find_first_file_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FindFirstFileExW",
        crate::vm::stdcall_args(6),
        find_first_file_ex_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FindNextFileA",
        crate::vm::stdcall_args(2),
        find_next_file_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FindNextFileW",
        crate::vm::stdcall_args(2),
        find_next_file_w,
    );
}

fn find_close(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn find_first_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    INVALID_HANDLE_VALUE
}

fn find_first_file_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    INVALID_HANDLE_VALUE
}

fn find_next_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn find_next_file_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
