use crate::pe::{ResourceData, ResourceDirectory, ResourceId, ResourceNode};
use crate::vm::Vm;
use crate::vm_args;

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
    let (_, _, buf_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, u32, usize);
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
    let fmt = vm.read_c_string(fmt_ptr).unwrap_or_default();
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
                width = width.saturating_mul(10).saturating_add((next as u8 - b'0') as usize);
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
                let value = vm.read_c_string(ptr).unwrap_or_default();
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
        eprintln!("[pe_vm] wsprintfA fmt={fmt:?} args=[{}]", arg_log.join(", "));
    }
    let mut bytes = output.into_bytes();
    bytes.push(0);
    let _ = vm.write_bytes(buf_ptr, &bytes);
    if std::env::var("PE_VM_TRACE").is_ok() {
        if let Ok(text) = vm.read_c_string(buf_ptr) {
            eprintln!("[pe_vm] wsprintfA dest=0x{buf_ptr:08X} text={text:?}");
        }
    }
    (bytes.len().saturating_sub(1)) as u32
}
