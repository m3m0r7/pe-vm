use crate::vm::Vm;

use super::helpers::{now_parts, read_system_time, write_utf16};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetDateFormatW", crate::vm::stdcall_args(6), get_date_format_w);
    vm.register_import_stdcall("KERNEL32.dll", "GetTimeFormatW", crate::vm::stdcall_args(6), get_time_format_w);
}

fn get_date_format_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0) as usize;
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:04}-{:02}-{:02}", parts.year, parts.month, parts.day);
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_time_format_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0) as usize;
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:02}:{:02}:{:02}", parts.hour, parts.minute, parts.second);
    write_utf16(vm, out_ptr, out_len, &text)
}
