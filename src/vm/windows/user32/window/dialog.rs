use crate::pe::{ResourceData, ResourceDirectory, ResourceId, ResourceNode};
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::helpers::{write_c_string, write_rect};

/// Dummy dialog item handle base
const DUMMY_DLG_ITEM_HANDLE: u32 = 0x20000;

/// GetDlgItem - Retrieves a handle to a control in the dialog box
/// Parameters:
///   - hDlg: Handle to the dialog box
///   - nIDDlgItem: Identifier of the control
/// Returns: Handle to the control, or NULL if failed
fn get_dlg_item(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hdlg, id) = vm_args!(vm, stack_ptr; u32, u32);
    if hdlg != 0 && id != 0 {
        // Return a dummy handle derived from the control ID
        DUMMY_DLG_ITEM_HANDLE | (id & 0xFFFF)
    } else {
        0 // NULL
    }
}

/// SendDlgItemMessageA - Sends a message to a control in a dialog box
/// Parameters:
///   - hDlg: Handle to the dialog box
///   - nIDDlgItem: Identifier of the control
///   - Msg: Message to send
///   - wParam: Additional message-specific information
///   - lParam: Additional message-specific information
/// Returns: Result of message processing (depends on the message)
fn send_dlg_item_message_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_hdlg, _id, _msg, _wparam, _lparam) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    // Without real controls, return 0 as default
    0
}

/// EndDialog - Destroys a modal dialog box and returns to caller
/// Parameters:
///   - hDlg: Handle to the dialog box
///   - nResult: Value to return to caller
/// Returns: TRUE if succeeded, FALSE if failed
fn end_dialog(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hdlg, _result) = vm_args!(vm, stack_ptr; u32, i32);
    if hdlg != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// DialogBoxIndirectParamA - Creates a modal dialog box from a template in memory
/// Parameters:
///   - hInstance: Handle to the module
///   - lpTemplate: Pointer to dialog box template
///   - hWndParent: Handle to owner window
///   - lpDialogFunc: Pointer to dialog box procedure
///   - dwInitParam: Value to pass to dialog box procedure
/// Returns: nResult from EndDialog, or -1 on failure
fn dialog_box_indirect_param_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_hinstance, template_ptr, _parent, _dlg_proc, _init_param) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    if template_ptr == 0 {
        return 0xFFFFFFFF; // -1 (failure)
    }
    // We can't actually display dialogs, so return 0 (IDOK equivalent)
    // or -1 for failure. Since template is valid, return success.
    0
}

/// MapWindowPoints - Maps a set of points from one window's coordinate space to another
/// Parameters:
///   - hWndFrom: Handle to source window
///   - hWndTo: Handle to destination window
///   - lpPoints: Pointer to array of POINT structures
///   - cPoints: Number of points in array
/// Returns: Number of pixels added to horizontal/vertical coordinates (MAKELONG format)
fn map_window_points(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_from, _to, _points_ptr, _count) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    // Without real window positions, return 0 (no offset)
    0
}

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "GetDlgItem",
        crate::vm::stdcall_args(2),
        get_dlg_item,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetDlgItemTextA",
        crate::vm::stdcall_args(4),
        get_dlg_item_text_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SendDlgItemMessageA",
        crate::vm::stdcall_args(5),
        send_dlg_item_message_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "EndDialog",
        crate::vm::stdcall_args(2),
        end_dialog,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "DialogBoxIndirectParamA",
        crate::vm::stdcall_args(5),
        dialog_box_indirect_param_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "MapWindowPoints",
        crate::vm::stdcall_args(4),
        map_window_points,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "MapDialogRect",
        crate::vm::stdcall_args(2),
        map_dialog_rect,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadStringA",
        crate::vm::stdcall_args(4),
        load_string_a,
    );
    vm.register_import(DLL_NAME, "wsprintfA", wsprintf_a);
}

pub(super) fn get_dlg_item_text_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, u32, usize);
    if buf_ptr != 0 && max_len > 0 {
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

pub(super) fn map_dialog_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, rect_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if rect_ptr != 0 {
        write_rect(vm, rect_ptr, 0, 0, 0, 0);
    }
    1
}

pub(super) fn load_string_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, string_id, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, u32, usize);
    if buf_ptr != 0 && max_len > 0 {
        if let Some(text) = load_string_resource(vm, string_id) {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] LoadStringA id={string_id} text={text:?}");
            }
            write_c_string(vm, buf_ptr, &text, max_len);
            return text.len().min(max_len.saturating_sub(1)) as u32;
        }
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] LoadStringA id={string_id} miss");
        }
        write_c_string(vm, buf_ptr, "", max_len);
    }
    0
}

fn load_string_resource(vm: &Vm, string_id: u32) -> Option<String> {
    let dir = vm.resource_dir()?;
    let block_id = (string_id / 16).wrapping_add(1);
    let entry = (string_id % 16) as usize;
    let data = find_string_block(dir, block_id)?;
    read_string_entry(data, entry)
}

fn find_string_block(dir: &ResourceDirectory, block_id: u32) -> Option<&ResourceData> {
    let type_node = dir
        .roots
        .iter()
        .find(|node| matches!(node.id, ResourceId::Id(id) if id == 6))?;
    let name_node = type_node
        .children
        .iter()
        .find(|node| matches!(node.id, ResourceId::Id(id) if id == block_id))?;
    find_resource_data(name_node)
}

fn find_resource_data(node: &ResourceNode) -> Option<&ResourceData> {
    if let Some(data) = node.data.as_ref() {
        return Some(data);
    }
    for child in &node.children {
        if let Some(data) = find_resource_data(child) {
            return Some(data);
        }
    }
    None
}

fn read_string_entry(data: &ResourceData, entry: usize) -> Option<String> {
    let bytes = &data.data;
    let mut cursor = 0usize;
    for idx in 0..16usize {
        if cursor + 2 > bytes.len() {
            return None;
        }
        let len = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]) as usize;
        cursor += 2;
        let byte_len = len.saturating_mul(2);
        if cursor + byte_len > bytes.len() {
            return None;
        }
        if idx == entry {
            let mut units = Vec::with_capacity(len);
            for chunk in bytes[cursor..cursor + byte_len].chunks(2) {
                let lo = *chunk.first().unwrap_or(&0);
                let hi = *chunk.get(1).unwrap_or(&0);
                units.push(u16::from_le_bytes([lo, hi]));
            }
            return Some(String::from_utf16_lossy(&units));
        }
        cursor += byte_len;
    }
    None
}

pub(super) fn wsprintf_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (buf_ptr, fmt_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if buf_ptr == 0 || fmt_ptr == 0 {
        return 0;
    }
    let fmt = read_wide_or_utf16le_str!(vm, fmt_ptr);
    let mut output = String::new();
    let mut arg_offset = 12u32;
    let mut arg_log = Vec::new();
    let mut chars = fmt.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            output.push(ch);
            continue;
        }
        if matches!(chars.peek(), Some('%')) {
            chars.next();
            output.push('%');
            continue;
        }
        let mut pad = ' ';
        let mut width = 0usize;
        if matches!(chars.peek(), Some('0')) {
            pad = '0';
            chars.next();
        }
        while let Some(next) = chars.peek().copied() {
            if next.is_ascii_digit() {
                width = width
                    .saturating_mul(10)
                    .saturating_add((next as u8 - b'0') as usize);
                chars.next();
            } else {
                break;
            }
        }
        let long = matches!(chars.peek(), Some('l'));
        if long {
            chars.next();
        }
        let spec = chars.next().unwrap_or('%');
        let arg_ptr = stack_ptr + arg_offset;
        let formatted = match spec {
            's' => {
                let ptr = vm.read_u32(arg_ptr).unwrap_or(0);
                arg_offset = arg_offset.wrapping_add(4);
                let value = read_wide_or_utf16le_str!(vm, ptr);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("s=0x{ptr:08X} {value:?}"));
                }
                value
            }
            'd' | 'i' => {
                let value = vm.read_u32(arg_ptr).unwrap_or(0) as i32;
                arg_offset = arg_offset.wrapping_add(4);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("d={value}"));
                }
                format!("{value}")
            }
            'u' => {
                let value = vm.read_u32(arg_ptr).unwrap_or(0);
                arg_offset = arg_offset.wrapping_add(4);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("u={value}"));
                }
                format!("{value}")
            }
            'x' => {
                let value = vm.read_u32(arg_ptr).unwrap_or(0);
                arg_offset = arg_offset.wrapping_add(4);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("x=0x{value:X}"));
                }
                format!("{value:x}")
            }
            'X' => {
                let value = vm.read_u32(arg_ptr).unwrap_or(0);
                arg_offset = arg_offset.wrapping_add(4);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("X=0x{value:X}"));
                }
                format!("{value:X}")
            }
            'c' => {
                let value = vm.read_u32(arg_ptr).unwrap_or(0) as u8;
                arg_offset = arg_offset.wrapping_add(4);
                if std::env::var("PE_VM_TRACE").is_ok() {
                    arg_log.push(format!("c={value}"));
                }
                (value as char).to_string()
            }
            _ => {
                if long {
                    arg_offset = arg_offset.wrapping_add(4);
                }
                format!("%{spec}")
            }
        };
        if width > formatted.len() {
            for _ in 0..(width - formatted.len()) {
                output.push(pad);
            }
        }
        output.push_str(&formatted);
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] wsprintfA fmt={fmt:?} args=[{}]",
            arg_log.join(", ")
        );
    }
    let mut bytes = output.into_bytes();
    bytes.push(0);
    let _ = vm.write_bytes(buf_ptr, &bytes);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let text = read_wide_or_utf16le_str!(vm, buf_ptr);
        eprintln!("[pe_vm] wsprintfA dest=0x{buf_ptr:08X} text={text:?}");
    }
    (bytes.len().saturating_sub(1)) as u32
}
