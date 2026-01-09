use crate::vm::Vm;
use crate::vm_args;

use super::helpers::{read_bytes, read_w_len};

const CT_CTYPE1: u32 = 0x0001;

const C1_UPPER: u16 = 0x0001;
const C1_LOWER: u16 = 0x0002;
const C1_DIGIT: u16 = 0x0004;
const C1_SPACE: u16 = 0x0008;
const C1_PUNCT: u16 = 0x0010;
const C1_CNTRL: u16 = 0x0020;
const C1_BLANK: u16 = 0x0040;
const C1_XDIGIT: u16 = 0x0080;
const C1_ALPHA: u16 = 0x0100;
const C1_DEFINED: u16 = 0x0200;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetStringTypeA",
        crate::vm::stdcall_args(4),
        get_string_type_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetStringTypeW",
        crate::vm::stdcall_args(4),
        get_string_type_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CompareStringW",
        crate::vm::stdcall_args(6),
        compare_string_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "LCMapStringA",
        crate::vm::stdcall_args(6),
        lc_map_string_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "LCMapStringW",
        crate::vm::stdcall_args(6),
        lc_map_string_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "IsDBCSLeadByte",
        crate::vm::stdcall_args(1),
        is_dbcs_lead_byte,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "MulDiv",
        crate::vm::stdcall_args(3),
        mul_div,
    );
}

fn get_string_type_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_type, src_ptr, src_len, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, i32, u32);
    if src_ptr == 0 || out_ptr == 0 {
        return 0;
    }
    let units = read_w_len(vm, src_ptr, src_len);
    for (idx, unit) in units.iter().enumerate() {
        let flags = if info_type & CT_CTYPE1 != 0 {
            ctype1_from_u16(*unit)
        } else {
            0
        };
        let _ = vm.write_u16(out_ptr + (idx as u32) * 2, flags);
    }
    1
}

fn get_string_type_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_type, src_ptr, src_len, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, i32, u32);
    if src_ptr == 0 || out_ptr == 0 {
        return 0;
    }
    let bytes = read_bytes(vm, src_ptr, src_len);
    for (idx, byte) in bytes.iter().enumerate() {
        let flags = if info_type & CT_CTYPE1 != 0 {
            ctype1_from_u8(*byte)
        } else {
            0
        };
        let _ = vm.write_u16(out_ptr + (idx as u32) * 2, flags);
    }
    1
}

fn compare_string_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale, _flags, left_ptr, left_len, right_ptr, right_len) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, i32);
    if left_ptr == 0 || right_ptr == 0 {
        return 0;
    }
    let left = String::from_utf16_lossy(&read_w_len(vm, left_ptr, left_len));
    let right = String::from_utf16_lossy(&read_w_len(vm, right_ptr, right_len));
    match left.cmp(&right) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    }
}

fn lc_map_string_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale, flags, src_ptr, src_len, dst_ptr, dst_len) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize);
    if src_ptr == 0 {
        return 0;
    }
    let mut text = String::from_utf16_lossy(&read_w_len(vm, src_ptr, src_len));
    if flags & 0x0000_0100 != 0 {
        text = text.to_ascii_lowercase();
    } else if flags & 0x0000_0200 != 0 {
        text = text.to_ascii_uppercase();
    }
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let needed = utf16.len() + 1;
    if dst_ptr == 0 || dst_len == 0 {
        return needed as u32;
    }
    let write_len = dst_len.saturating_sub(1).min(utf16.len());
    for (idx, unit) in utf16.iter().take(write_len).enumerate() {
        let _ = vm.write_u16(dst_ptr + (idx as u32) * 2, *unit);
    }
    let _ = vm.write_u16(dst_ptr + (write_len as u32) * 2, 0);
    needed as u32
}

fn lc_map_string_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale, flags, src_ptr, src_len, dst_ptr, dst_len) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize);
    if src_ptr == 0 {
        return 0;
    }
    let mut text = String::from_utf8_lossy(&read_bytes(vm, src_ptr, src_len)).into_owned();
    if flags & 0x0000_0100 != 0 {
        text = text.to_ascii_lowercase();
    } else if flags & 0x0000_0200 != 0 {
        text = text.to_ascii_uppercase();
    }
    let needed = text.len() + 1;
    if dst_ptr == 0 || dst_len == 0 {
        return needed as u32;
    }
    let mut bytes = text.into_bytes();
    if bytes.len() >= dst_len {
        bytes.truncate(dst_len.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(dst_ptr, &bytes);
    needed as u32
}

fn is_dbcs_lead_byte(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn ctype1_from_u16(unit: u16) -> u16 {
    if unit <= 0x7F {
        return ctype1_from_u8(unit as u8);
    }
    let Some(ch) = char::from_u32(unit as u32) else {
        return 0;
    };
    let mut flags = C1_DEFINED;
    if ch.is_alphabetic() {
        flags |= C1_ALPHA;
    }
    if ch.is_uppercase() {
        flags |= C1_UPPER;
    }
    if ch.is_lowercase() {
        flags |= C1_LOWER;
    }
    if ch.is_numeric() {
        flags |= C1_DIGIT;
    }
    if ch.is_whitespace() {
        flags |= C1_SPACE;
    }
    if ch.is_control() {
        flags |= C1_CNTRL;
    }
    flags
}

fn ctype1_from_u8(byte: u8) -> u16 {
    match byte {
        b'0'..=b'9' => C1_DIGIT | C1_XDIGIT | C1_DEFINED,
        b'A'..=b'Z' => {
            let mut flags = C1_UPPER | C1_ALPHA | C1_DEFINED;
            if (b'A'..=b'F').contains(&byte) {
                flags |= C1_XDIGIT;
            }
            flags
        }
        b'a'..=b'z' => {
            let mut flags = C1_LOWER | C1_ALPHA | C1_DEFINED;
            if (b'a'..=b'f').contains(&byte) {
                flags |= C1_XDIGIT;
            }
            flags
        }
        b' ' => C1_SPACE | C1_BLANK | C1_DEFINED,
        b'\t' => C1_SPACE | C1_BLANK | C1_CNTRL | C1_DEFINED,
        b'\n' | b'\r' | 0x0B | 0x0C => C1_SPACE | C1_CNTRL | C1_DEFINED,
        0x00..=0x1F | 0x7F => C1_CNTRL | C1_DEFINED,
        0x21..=0x7E => C1_PUNCT | C1_DEFINED,
        _ => C1_DEFINED,
    }
}

fn mul_div(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (number, numerator, denominator) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let number = number as i64;
    let numerator = numerator as i64;
    let denominator = denominator as i64;
    if denominator == 0 {
        return 0;
    }
    let value = number
        .saturating_mul(numerator)
        .saturating_add(denominator / 2)
        / denominator;
    value as u32
}
