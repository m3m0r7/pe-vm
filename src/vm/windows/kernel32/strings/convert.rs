use crate::vm::Vm;

use encoding_rs::{SHIFT_JIS, UTF_8, WINDOWS_1252};

use super::helpers::{read_bytes, read_utf16};
use super::codepage::resolve_code_page;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "MultiByteToWideChar",
        crate::vm::stdcall_args(6),
        multi_byte_to_wide_char,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "WideCharToMultiByte",
        crate::vm::stdcall_args(8),
        wide_char_to_multi_byte,
    );
}

fn multi_byte_to_wide_char(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let code_page = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let dst_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
    if src_ptr == 0 {
        return 0;
    }
    // Include a terminator when the source length is -1.
    let (bytes, needs_null) = if src_len < 0 {
        (read_bytes(vm, src_ptr, src_len), true)
    } else {
        (read_bytes(vm, src_ptr, src_len), false)
    };
    let resolved = resolve_code_page(vm, code_page);
    let text = decode_codepage(resolved, &bytes);
    let mut utf16: Vec<u16> = text.encode_utf16().collect();
    if needs_null {
        utf16.push(0);
    }
    let required = utf16.len();
    if dst_ptr == 0 {
        return required as u32;
    }
    let write_len = dst_len.min(required);
    for (i, value) in utf16.iter().enumerate().take(write_len) {
        let _ = vm.write_u16(dst_ptr + (i as u32) * 2, *value);
    }
    write_len as u32
}

fn wide_char_to_multi_byte(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let code_page = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let dst_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
    let _def_char = vm.read_u32(stack_ptr + 28).unwrap_or(0);
    let _used_default = vm.read_u32(stack_ptr + 32).unwrap_or(0);
    if src_ptr == 0 {
        return 0;
    }
    // Include a terminator when the source length is -1.
    let (utf16, needs_null) = if src_len < 0 {
        (read_utf16(vm, src_ptr, src_len), true)
    } else {
        (read_utf16(vm, src_ptr, src_len), false)
    };
    let text = String::from_utf16_lossy(&utf16);
    let resolved = resolve_code_page(vm, code_page);
    if dst_ptr == 0 {
        let required = encode_codepage(resolved, &text);
        return if needs_null {
            required.len().saturating_add(1) as u32
        } else {
            required.len() as u32
        };
    }
    let mut bytes = encode_codepage(resolved, &text);
    if needs_null {
        bytes.push(0);
    }
    let write_len = dst_len.min(bytes.len());
    let _ = vm.write_bytes(dst_ptr, &bytes[..write_len]);
    write_len as u32
}

fn decode_codepage(code_page: u32, bytes: &[u8]) -> String {
    match code_page {
        932 => {
            let (text, _, _) = SHIFT_JIS.decode(bytes);
            text.into_owned()
        }
        65001 => {
            let (text, _, _) = UTF_8.decode(bytes);
            text.into_owned()
        }
        1252 => {
            let (text, _, _) = WINDOWS_1252.decode(bytes);
            text.into_owned()
        }
        _ => String::from_utf8_lossy(bytes).to_string(),
    }
}

fn encode_codepage(code_page: u32, text: &str) -> Vec<u8> {
    match code_page {
        932 => SHIFT_JIS.encode(text).0.into_owned(),
        65001 => UTF_8.encode(text).0.into_owned(),
        1252 => WINDOWS_1252.encode(text).0.into_owned(),
        _ => text.as_bytes().to_vec(),
    }
}
