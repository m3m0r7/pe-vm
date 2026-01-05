//! User32 windowing and input stubs.

use crate::vm::Vm;

const DUMMY_HWND: u32 = 1;
const DUMMY_HDC: u32 = 1;
const DUMMY_HMONITOR: u32 = 1;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("USER32.dll", "CreateWindowExA", crate::vm::stdcall_args(12), create_window_ex_a);
    vm.register_import_stdcall("USER32.dll", "DestroyWindow", crate::vm::stdcall_args(1), destroy_window);
    vm.register_import_stdcall("USER32.dll", "ShowWindow", crate::vm::stdcall_args(2), show_window);
    vm.register_import_stdcall("USER32.dll", "MoveWindow", crate::vm::stdcall_args(6), move_window);
    vm.register_import_stdcall("USER32.dll", "SetWindowPos", crate::vm::stdcall_args(7), set_window_pos);
    vm.register_import_stdcall("USER32.dll", "GetWindowRect", crate::vm::stdcall_args(2), get_window_rect);
    vm.register_import_stdcall("USER32.dll", "GetClientRect", crate::vm::stdcall_args(2), get_client_rect);
    vm.register_import_stdcall("USER32.dll", "GetWindowTextLengthA", crate::vm::stdcall_args(1), get_window_text_length_a);
    vm.register_import_stdcall("USER32.dll", "GetWindowTextA", crate::vm::stdcall_args(3), get_window_text_a);
    vm.register_import_stdcall("USER32.dll", "SetWindowTextA", crate::vm::stdcall_args(2), set_window_text_a);
    vm.register_import_stdcall("USER32.dll", "GetWindow", crate::vm::stdcall_args(2), get_window);
    vm.register_import_stdcall("USER32.dll", "GetParent", crate::vm::stdcall_args(1), get_parent);
    vm.register_import_stdcall("USER32.dll", "GetDesktopWindow", crate::vm::stdcall_args(0), get_desktop_window);
    vm.register_import_stdcall("USER32.dll", "IsWindow", crate::vm::stdcall_args(1), is_window);
    vm.register_import_stdcall("USER32.dll", "IsChild", crate::vm::stdcall_args(2), is_child);
    vm.register_import_stdcall("USER32.dll", "GetClassNameA", crate::vm::stdcall_args(3), get_class_name_a);
    vm.register_import_stdcall("USER32.dll", "GetWindowLongA", crate::vm::stdcall_args(2), get_window_long_a);
    vm.register_import_stdcall("USER32.dll", "SetWindowLongA", crate::vm::stdcall_args(3), set_window_long_a);
    vm.register_import_stdcall("USER32.dll", "GetDlgItem", crate::vm::stdcall_args(2), get_dlg_item);
    vm.register_import_stdcall("USER32.dll", "GetDlgItemTextA", crate::vm::stdcall_args(4), get_dlg_item_text_a);
    vm.register_import_stdcall("USER32.dll", "SendDlgItemMessageA", crate::vm::stdcall_args(5), send_dlg_item_message_a);
    vm.register_import_stdcall("USER32.dll", "EndDialog", crate::vm::stdcall_args(2), end_dialog);
    vm.register_import_stdcall(
        "USER32.dll",
        "DialogBoxIndirectParamA",
        crate::vm::stdcall_args(5),
        dialog_box_indirect_param_a,
    );
    vm.register_import_stdcall("USER32.dll", "MapWindowPoints", crate::vm::stdcall_args(4), map_window_points);
    vm.register_import_stdcall("USER32.dll", "MapDialogRect", crate::vm::stdcall_args(2), map_dialog_rect);
    vm.register_import_stdcall("USER32.dll", "ScreenToClient", crate::vm::stdcall_args(2), screen_to_client);
    vm.register_import_stdcall("USER32.dll", "ClientToScreen", crate::vm::stdcall_args(2), client_to_screen);
    vm.register_import_stdcall("USER32.dll", "SetWindowRgn", crate::vm::stdcall_args(3), set_window_rgn);
    vm.register_import_stdcall("USER32.dll", "SetWindowContextHelpId", crate::vm::stdcall_args(2), set_window_context_help_id);
    vm.register_import_stdcall("USER32.dll", "SetForegroundWindow", crate::vm::stdcall_args(1), set_foreground_window);
    vm.register_import_stdcall("USER32.dll", "InvalidateRect", crate::vm::stdcall_args(3), invalidate_rect);
    vm.register_import_stdcall("USER32.dll", "InvalidateRgn", crate::vm::stdcall_args(3), invalidate_rgn);
    vm.register_import_stdcall("USER32.dll", "RedrawWindow", crate::vm::stdcall_args(4), redraw_window);
    vm.register_import_stdcall("USER32.dll", "PostMessageA", crate::vm::stdcall_args(4), post_message_a);
    vm.register_import_stdcall("USER32.dll", "SendMessageA", crate::vm::stdcall_args(4), send_message_a);
    vm.register_import_stdcall(
        "USER32.dll",
        "RegisterWindowMessageA",
        crate::vm::stdcall_args(1),
        register_window_message_a,
    );
    vm.register_import_stdcall("USER32.dll", "CallWindowProcA", crate::vm::stdcall_args(5), call_window_proc_a);
    vm.register_import_stdcall("USER32.dll", "DefWindowProcA", crate::vm::stdcall_args(4), def_window_proc_a);
    vm.register_import_stdcall("USER32.dll", "UnregisterClassA", crate::vm::stdcall_args(2), unregister_class_a);
    vm.register_import_stdcall("USER32.dll", "SetParent", crate::vm::stdcall_args(2), set_parent);
    vm.register_import_stdcall("USER32.dll", "GetActiveWindow", crate::vm::stdcall_args(0), get_active_window);
    vm.register_import_stdcall("USER32.dll", "GetCursorPos", crate::vm::stdcall_args(1), get_cursor_pos);
    vm.register_import_stdcall("USER32.dll", "SetCursor", crate::vm::stdcall_args(1), set_cursor);
    vm.register_import_stdcall("USER32.dll", "SetCapture", crate::vm::stdcall_args(1), set_capture);
    vm.register_import_stdcall("USER32.dll", "ReleaseCapture", crate::vm::stdcall_args(0), release_capture);
    vm.register_import_stdcall("USER32.dll", "GetKeyState", crate::vm::stdcall_args(1), get_key_state);
    vm.register_import_stdcall("USER32.dll", "GetFocus", crate::vm::stdcall_args(0), get_focus);
    vm.register_import_stdcall("USER32.dll", "SetFocus", crate::vm::stdcall_args(1), set_focus);
    vm.register_import_stdcall("USER32.dll", "MonitorFromWindow", crate::vm::stdcall_args(2), monitor_from_window);
    vm.register_import_stdcall("USER32.dll", "GetMonitorInfoA", crate::vm::stdcall_args(2), get_monitor_info_a);
    vm.register_import_stdcall("USER32.dll", "GetSystemMetrics", crate::vm::stdcall_args(1), get_system_metrics);
    vm.register_import_stdcall("USER32.dll", "GetSysColor", crate::vm::stdcall_args(1), get_sys_color);
    vm.register_import_stdcall("USER32.dll", "PtInRect", crate::vm::stdcall_args(3), pt_in_rect);
    vm.register_import_stdcall("USER32.dll", "EqualRect", crate::vm::stdcall_args(2), equal_rect);
    vm.register_import_stdcall("USER32.dll", "OffsetRect", crate::vm::stdcall_args(3), offset_rect);
    vm.register_import_stdcall("USER32.dll", "UnionRect", crate::vm::stdcall_args(3), union_rect);
    vm.register_import_stdcall("USER32.dll", "IntersectRect", crate::vm::stdcall_args(3), intersect_rect);
    vm.register_import_stdcall("USER32.dll", "FillRect", crate::vm::stdcall_args(3), fill_rect);
    vm.register_import_stdcall("USER32.dll", "GetDC", crate::vm::stdcall_args(1), get_dc);
    vm.register_import_stdcall("USER32.dll", "ReleaseDC", crate::vm::stdcall_args(2), release_dc);
    vm.register_import_stdcall("USER32.dll", "BeginPaint", crate::vm::stdcall_args(2), begin_paint);
    vm.register_import_stdcall("USER32.dll", "EndPaint", crate::vm::stdcall_args(2), end_paint);
    vm.register_import_stdcall(
        "USER32.dll",
        "CreateAcceleratorTableA",
        crate::vm::stdcall_args(2),
        create_accelerator_table_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "DestroyAcceleratorTable",
        crate::vm::stdcall_args(1),
        destroy_accelerator_table,
    );
    vm.register_import_stdcall("USER32.dll", "LoadStringA", crate::vm::stdcall_args(4), load_string_a);
    vm.register_import("USER32.dll", "wsprintfA", wsprintf_a);
}

fn create_window_ex_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

fn destroy_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn show_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn move_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_window_pos(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_window_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 640, 480);
    }
    1
}

fn get_client_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 640, 480);
    }
    1
}

fn get_window_text_length_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_window_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let text_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if text_ptr != 0 && max_len > 0 {
        write_c_string(vm, text_ptr, "", max_len);
    }
    0
}

fn set_window_text_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_parent(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_desktop_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HWND
}

fn is_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let hwnd = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if hwnd == 0 { 0 } else { 1 }
}

fn is_child(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_class_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

fn get_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn set_window_long_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_dlg_item(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_dlg_item_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

fn send_dlg_item_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn end_dialog(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn dialog_box_indirect_param_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn map_window_points(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn map_dialog_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 0, 0);
    }
    1
}

fn screen_to_client(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn client_to_screen(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_window_rgn(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_window_context_help_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_foreground_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn invalidate_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn invalidate_rgn(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn redraw_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn post_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn send_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn register_window_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn call_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn def_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn unregister_class_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_parent(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_active_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_cursor_pos(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let point_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if point_ptr != 0 {
        write_point(vm, point_ptr, 0, 0);
    }
    1
}

fn set_cursor(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn set_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn release_capture(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_key_state(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn set_focus(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn monitor_from_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HMONITOR
}

fn get_monitor_info_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if info_ptr != 0 {
        let _ = vm.write_u32(info_ptr, 40);
        write_rect(vm, info_ptr + 4, 0, 0, 640, 480);
        write_rect(vm, info_ptr + 20, 0, 0, 640, 480);
        let _ = vm.write_u32(info_ptr + 36, 0);
    }
    1
}

fn get_system_metrics(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    match index {
        0 => 1024,
        1 => 768,
        _ => 0,
    }
}

fn get_sys_color(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0x00FF_FFFF
}

fn pt_in_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn equal_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn offset_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let dx = vm.read_u32(stack_ptr + 8).unwrap_or(0) as i32;
    let dy = vm.read_u32(stack_ptr + 12).unwrap_or(0) as i32;
    if rect_ptr != 0 {
        let left = vm.read_u32(rect_ptr).unwrap_or(0) as i32 + dx;
        let top = vm.read_u32(rect_ptr + 4).unwrap_or(0) as i32 + dy;
        let right = vm.read_u32(rect_ptr + 8).unwrap_or(0) as i32 + dx;
        let bottom = vm.read_u32(rect_ptr + 12).unwrap_or(0) as i32 + dy;
        write_rect(vm, rect_ptr, left, top, right, bottom);
    }
    1
}

fn union_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dst_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src1_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src2_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if dst_ptr != 0 && src1_ptr != 0 && src2_ptr != 0 {
        let l1 = vm.read_u32(src1_ptr).unwrap_or(0) as i32;
        let t1 = vm.read_u32(src1_ptr + 4).unwrap_or(0) as i32;
        let r1 = vm.read_u32(src1_ptr + 8).unwrap_or(0) as i32;
        let b1 = vm.read_u32(src1_ptr + 12).unwrap_or(0) as i32;
        let l2 = vm.read_u32(src2_ptr).unwrap_or(0) as i32;
        let t2 = vm.read_u32(src2_ptr + 4).unwrap_or(0) as i32;
        let r2 = vm.read_u32(src2_ptr + 8).unwrap_or(0) as i32;
        let b2 = vm.read_u32(src2_ptr + 12).unwrap_or(0) as i32;
        write_rect(vm, dst_ptr, l1.min(l2), t1.min(t2), r1.max(r2), b1.max(b2));
    }
    1
}

fn intersect_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dst_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src1_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src2_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if dst_ptr == 0 || src1_ptr == 0 || src2_ptr == 0 {
        return 0;
    }
    let l1 = vm.read_u32(src1_ptr).unwrap_or(0) as i32;
    let t1 = vm.read_u32(src1_ptr + 4).unwrap_or(0) as i32;
    let r1 = vm.read_u32(src1_ptr + 8).unwrap_or(0) as i32;
    let b1 = vm.read_u32(src1_ptr + 12).unwrap_or(0) as i32;
    let l2 = vm.read_u32(src2_ptr).unwrap_or(0) as i32;
    let t2 = vm.read_u32(src2_ptr + 4).unwrap_or(0) as i32;
    let r2 = vm.read_u32(src2_ptr + 8).unwrap_or(0) as i32;
    let b2 = vm.read_u32(src2_ptr + 12).unwrap_or(0) as i32;
    let left = l1.max(l2);
    let top = t1.max(t2);
    let right = r1.min(r2);
    let bottom = b1.min(b2);
    if right <= left || bottom <= top {
        write_rect(vm, dst_ptr, 0, 0, 0, 0);
        return 0;
    }
    write_rect(vm, dst_ptr, left, top, right, bottom);
    1
}

fn fill_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HDC
}

fn release_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn begin_paint(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ps_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if ps_ptr != 0 {
        let _ = vm.write_bytes(ps_ptr, &[0u8; 64]);
    }
    DUMMY_HDC
}

fn end_paint(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_accelerator_table_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn destroy_accelerator_table(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn load_string_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

fn wsprintf_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if buf_ptr != 0 {
        write_c_string(vm, buf_ptr, "", 1);
    }
    0
}

fn write_rect(vm: &mut Vm, rect_ptr: u32, left: i32, top: i32, right: i32, bottom: i32) {
    let _ = vm.write_u32(rect_ptr, left as u32);
    let _ = vm.write_u32(rect_ptr + 4, top as u32);
    let _ = vm.write_u32(rect_ptr + 8, right as u32);
    let _ = vm.write_u32(rect_ptr + 12, bottom as u32);
}

fn write_point(vm: &mut Vm, point_ptr: u32, x: i32, y: i32) {
    let _ = vm.write_u32(point_ptr, x as u32);
    let _ = vm.write_u32(point_ptr + 4, y as u32);
}

fn write_c_string(vm: &mut Vm, dest: u32, text: &str, max_len: usize) {
    let mut bytes = text.as_bytes().to_vec();
    if bytes.len() + 1 > max_len {
        bytes.truncate(max_len.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(dest, &bytes);
}
