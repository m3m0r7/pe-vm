use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::constants::DUMMY_HMONITOR;
use super::helpers::write_rect;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "MonitorFromWindow",
        crate::vm::stdcall_args(2),
        monitor_from_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetMonitorInfoA",
        crate::vm::stdcall_args(2),
        get_monitor_info_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetSystemMetrics",
        crate::vm::stdcall_args(1),
        get_system_metrics,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetSysColor",
        crate::vm::stdcall_args(1),
        get_sys_color,
    );
}

pub(super) fn monitor_from_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HMONITOR
}

pub(super) fn get_monitor_info_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, info_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if info_ptr != 0 {
        let _ = vm.write_u32(info_ptr, 40);
        write_rect(vm, info_ptr + 4, 0, 0, 640, 480);
        write_rect(vm, info_ptr + 20, 0, 0, 640, 480);
        let _ = vm.write_u32(info_ptr + 36, 0);
    }
    1
}

pub(super) fn get_system_metrics(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index,) = vm_args!(vm, stack_ptr; u32);
    match index {
        0 => 1024,
        1 => 768,
        _ => 0,
    }
}

pub(super) fn get_sys_color(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0x00FF_FFFF
}
