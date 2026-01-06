use crate::vm::Vm;

use super::helpers::{write_c_string, write_rect};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "USER32.dll",
        "GetDlgItem",
        crate::vm::stdcall_args(2),
        get_dlg_item,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "GetDlgItemTextA",
        crate::vm::stdcall_args(4),
        get_dlg_item_text_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SendDlgItemMessageA",
        crate::vm::stdcall_args(5),
        send_dlg_item_message_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "EndDialog",
        crate::vm::stdcall_args(2),
        end_dialog,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "DialogBoxIndirectParamA",
        crate::vm::stdcall_args(5),
        dialog_box_indirect_param_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "MapWindowPoints",
        crate::vm::stdcall_args(4),
        map_window_points,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "MapDialogRect",
        crate::vm::stdcall_args(2),
        map_dialog_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "LoadStringA",
        crate::vm::stdcall_args(4),
        load_string_a,
    );
    vm.register_import("USER32.dll", "wsprintfA", wsprintf_a);
}

pub(super) fn get_dlg_item(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn get_dlg_item_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

pub(super) fn send_dlg_item_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn end_dialog(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn dialog_box_indirect_param_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn map_window_points(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn map_dialog_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 0, 0);
    }
    1
}

pub(super) fn load_string_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

pub(super) fn wsprintf_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if buf_ptr != 0 {
        write_c_string(vm, buf_ptr, "", 1);
    }
    0
}
