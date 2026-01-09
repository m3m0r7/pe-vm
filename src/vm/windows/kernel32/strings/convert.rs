use crate::vm::Vm;
use crate::vm_args;

use encoding_rs::{SHIFT_JIS, UTF_8, WINDOWS_1252};

use super::codepage::resolve_code_page;
use super::helpers::{read_bytes, read_utf16};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

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
    let (code_page, _flags, src_ptr, src_len, dst_ptr, dst_len) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize);
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
    if std::env::var("PE_VM_TRACE_CONVERT").is_ok() {
        let preview = preview_bytes(&bytes);
        eprintln!(
            "[pe_vm] MultiByteToWideChar cp={resolved} src_ptr=0x{src_ptr:08X} src_len={src_len} dst_ptr=0x{dst_ptr:08X} dst_len={dst_len} bytes={preview:?}"
        );
    }
    let text = decode_codepage(resolved, &bytes);
    let mut utf16: Vec<u16> = text.encode_utf16().collect();
    if needs_null {
        utf16.push(0);
    }
    let required = utf16.len();
    if std::env::var("PE_VM_TRACE_CONVERT").is_ok() {
        let preview = preview_utf16(&utf16);
        eprintln!(
            "[pe_vm] MultiByteToWideChar required={required} utf16={preview:?}"
        );
    }
    if dst_ptr == 0 || dst_len == 0 {
        return required as u32;
    }
    if dst_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    for (i, value) in utf16.iter().enumerate() {
        let _ = vm.write_u16(dst_ptr + (i as u32) * 2, *value);
    }
    required as u32
}

fn wide_char_to_multi_byte(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (code_page, _flags, src_ptr, src_len, dst_ptr, dst_len, _def_char, _used_default) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize, u32, u32);
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
    let mut bytes = encode_codepage(resolved, &text);
    if needs_null {
        bytes.push(0);
    }
    let required = bytes.len();
    if std::env::var("PE_VM_TRACE_CONVERT").is_ok() {
        let preview = preview_utf16(&utf16);
        eprintln!(
            "[pe_vm] WideCharToMultiByte cp={resolved} src_ptr=0x{src_ptr:08X} src_len={src_len} dst_ptr=0x{dst_ptr:08X} dst_len={dst_len} utf16={preview:?} required={required}"
        );
    }
    if dst_ptr == 0 || dst_len == 0 {
        return required as u32;
    }
    if dst_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    let _ = vm.write_bytes(dst_ptr, &bytes);
    required as u32
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

fn preview_bytes(bytes: &[u8]) -> String {
    let mut out = String::new();
    let limit = bytes.len().min(64);
    for &byte in &bytes[..limit] {
        if (0x20..=0x7E).contains(&byte) {
            out.push(byte as char);
        } else {
            out.push('.');
        }
    }
    if bytes.len() > limit {
        out.push_str("...");
    }
    out
}

fn preview_utf16(units: &[u16]) -> String {
    let limit = units.len().min(32);
    let slice = &units[..limit];
    let mut text = String::from_utf16_lossy(slice);
    if units.len() > limit {
        text.push_str("...");
    }
    text
}
