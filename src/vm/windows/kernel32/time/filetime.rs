use crate::vm::Vm;

use super::helpers::{
    filetime_from_parts, filetime_now, parts_from_filetime, read_filetime, read_system_time,
    write_filetime, write_system_time,
};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetSystemTimeAsFileTime",
        crate::vm::stdcall_args(1),
        get_system_time_as_filetime,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetLocalTime", crate::vm::stdcall_args(1), get_local_time);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SystemTimeToFileTime",
        crate::vm::stdcall_args(2),
        system_time_to_filetime,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FileTimeToSystemTime",
        crate::vm::stdcall_args(2),
        file_time_to_system_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "LocalFileTimeToFileTime",
        crate::vm::stdcall_args(2),
        local_file_time_to_file_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FileTimeToLocalFileTime",
        crate::vm::stdcall_args(2),
        file_time_to_local_file_time,
    );
}

fn get_system_time_as_filetime(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if filetime_ptr == 0 {
        return 0;
    }

    let ticks = filetime_now();
    write_filetime(vm, filetime_ptr, ticks);

    0
}

fn get_local_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if out_ptr == 0 {
        return 0;
    }
    let parts = super::helpers::now_parts();
    write_system_time(vm, out_ptr, &parts);
    0
}

fn system_time_to_filetime(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if time_ptr == 0 || filetime_ptr == 0 {
        return 0;
    }
    let parts = read_system_time(vm, time_ptr);
    let ticks = filetime_from_parts(&parts);
    write_filetime(vm, filetime_ptr, ticks);
    1
}

fn file_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if filetime_ptr == 0 || time_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, filetime_ptr);
    let parts = parts_from_filetime(ticks);
    write_system_time(vm, time_ptr, &parts);
    1
}

fn local_file_time_to_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let local_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if local_ptr == 0 || filetime_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, local_ptr);
    write_filetime(vm, filetime_ptr, ticks);
    1
}

fn file_time_to_local_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let local_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if filetime_ptr == 0 || local_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, filetime_ptr);
    write_filetime(vm, local_ptr, ticks);
    1
}
