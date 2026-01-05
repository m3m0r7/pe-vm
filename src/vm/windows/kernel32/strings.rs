//! Kernel32 ANSI/Unicode string helpers.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetACP", crate::vm::stdcall_args(0), get_acp);
    vm.register_import_stdcall("KERNEL32.dll", "GetOEMCP", crate::vm::stdcall_args(0), get_oemcp);
    vm.register_import_stdcall("KERNEL32.dll", "AreFileApisANSI", crate::vm::stdcall_args(0), are_file_apis_ansi);
    vm.register_import_stdcall("KERNEL32.dll", "IsValidCodePage", crate::vm::stdcall_args(1), is_valid_code_page);
    vm.register_import_stdcall("KERNEL32.dll", "GetCPInfo", crate::vm::stdcall_args(2), get_cp_info);
    vm.register_import_stdcall("KERNEL32.dll", "GetStringTypeW", crate::vm::stdcall_args(4), get_string_type_w);
    vm.register_import_stdcall("KERNEL32.dll", "CompareStringW", crate::vm::stdcall_args(6), compare_string_w);
    vm.register_import_stdcall("KERNEL32.dll", "LCMapStringW", crate::vm::stdcall_args(6), lc_map_string_w);
    vm.register_import_stdcall("KERNEL32.dll", "IsDBCSLeadByte", crate::vm::stdcall_args(1), is_dbcs_lead_byte);
    vm.register_import_stdcall("KERNEL32.dll", "MulDiv", crate::vm::stdcall_args(3), mul_div);
    vm.register_import_stdcall("KERNEL32.dll", "lstrlenA", crate::vm::stdcall_args(1), lstrlen_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcpyA", crate::vm::stdcall_args(2), lstrcpy_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcatA", crate::vm::stdcall_args(2), lstrcat_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcmpA", crate::vm::stdcall_args(2), lstrcmp_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcmpiA", crate::vm::stdcall_args(2), lstrcmpi_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcpynA", crate::vm::stdcall_args(3), lstrcpyn_a);
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

fn get_acp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn get_oemcp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn are_file_apis_ansi(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn is_valid_code_page(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_cp_info(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let _ = vm.write_u32(info_ptr, 1);
    let _ = vm.write_u8(info_ptr + 4, 0);
    for idx in 0..12 {
        let _ = vm.write_u8(info_ptr + 6 + idx, 0);
    }
    1
}

fn get_string_type_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let src_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as i32;
    let out_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if src_ptr == 0 || out_ptr == 0 {
        return 0;
    }
    let count = read_w_len(vm, src_ptr, src_len).len();
    for idx in 0..count {
        let _ = vm.write_u16(out_ptr + (idx as u32) * 2, 0);
    }
    1
}

fn compare_string_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let left_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let right_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let right_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as i32;
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
    let flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let dst_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
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

fn is_dbcs_lead_byte(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn mul_div(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let number = vm.read_u32(stack_ptr + 4).unwrap_or(0) as i64;
    let numerator = vm.read_u32(stack_ptr + 8).unwrap_or(0) as i64;
    let denominator = vm.read_u32(stack_ptr + 12).unwrap_or(1) as i64;
    if denominator == 0 {
        return 0;
    }
    let value = number.saturating_mul(numerator).saturating_add(denominator / 2) / denominator;
    value as u32
}

fn lstrlen_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return 0;
    }
    vm.read_c_string(ptr).map(|s| s.len() as u32).unwrap_or(0)
}

fn lstrcpy_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if dest == 0 || src == 0 {
        return dest;
    }
    let text = vm.read_c_string(src).unwrap_or_default();
    write_c_string(vm, dest, &text);
    dest
}

fn lstrcat_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if dest == 0 || src == 0 {
        return dest;
    }
    let mut dest_text = vm.read_c_string(dest).unwrap_or_default();
    let src_text = vm.read_c_string(src).unwrap_or_default();
    dest_text.push_str(&src_text);
    write_c_string(vm, dest, &dest_text);
    dest
}

fn lstrcmp_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left = read_string_arg(vm, stack_ptr + 4);
    let right = read_string_arg(vm, stack_ptr + 8);
    compare_strings(&left, &right) as u32
}

fn lstrcmpi_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left = read_string_arg(vm, stack_ptr + 4).to_ascii_lowercase();
    let right = read_string_arg(vm, stack_ptr + 8).to_ascii_lowercase();
    compare_strings(&left, &right) as u32
}

fn lstrcpyn_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let count = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if dest == 0 || src == 0 || count == 0 {
        return dest;
    }
    let text = vm.read_c_string(src).unwrap_or_default();
    let mut trimmed = text.as_bytes().to_vec();
    if trimmed.len() >= count {
        trimmed.truncate(count - 1);
    }
    trimmed.push(0);
    let _ = vm.write_bytes(dest, &trimmed);
    dest
}

fn multi_byte_to_wide_char(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _code_page = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let dst_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
    if src_ptr == 0 {
        return 0;
    }
    let bytes = read_bytes(vm, src_ptr, src_len);
    let text = String::from_utf8_lossy(&bytes);
    let utf16: Vec<u16> = text.encode_utf16().collect();
    if dst_ptr == 0 {
        return utf16.len() as u32;
    }
    let write_len = dst_len.min(utf16.len());
    for (i, value) in utf16.iter().enumerate().take(write_len) {
        let _ = vm.write_u16(dst_ptr + (i as u32) * 2, *value);
    }
    write_len as u32
}

fn wide_char_to_multi_byte(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _code_page = vm.read_u32(stack_ptr + 4).unwrap_or(0);
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
    let utf16 = read_utf16(vm, src_ptr, src_len);
    let text = String::from_utf16_lossy(&utf16);
    if dst_ptr == 0 {
        return text.len() as u32;
    }
    let bytes = text.into_bytes();
    let write_len = dst_len.min(bytes.len());
    let _ = vm.write_bytes(dst_ptr, &bytes[..write_len]);
    write_len as u32
}

fn read_string_arg(vm: &mut Vm, ptr_addr: u32) -> String {
    let ptr = vm.read_u32(ptr_addr).unwrap_or(0);
    if ptr == 0 {
        return String::new();
    }
    vm.read_c_string(ptr).unwrap_or_default()
}

fn compare_strings(left: &str, right: &str) -> i32 {
    match left.cmp(right) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn write_c_string(vm: &mut Vm, dest: u32, text: &str) {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    let _ = vm.write_bytes(dest, &bytes);
}

fn read_bytes(vm: &Vm, ptr: u32, len: i32) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }
    if len < 0 {
        let mut out = Vec::new();
        let mut cursor = ptr;
        for _ in 0..0x10000 {
            let value = vm.read_u8(cursor).unwrap_or(0);
            if value == 0 {
                break;
            }
            out.push(value);
            cursor = cursor.wrapping_add(1);
        }
        return out;
    }
    let mut out = Vec::with_capacity(len as usize);
    for i in 0..len {
        out.push(vm.read_u8(ptr + i as u32).unwrap_or(0));
    }
    out
}

fn read_utf16(vm: &Vm, ptr: u32, len: i32) -> Vec<u16> {
    if len == 0 {
        return Vec::new();
    }
    if len < 0 {
        let mut out = Vec::new();
        let mut cursor = ptr;
        for _ in 0..0x10000 {
            let value = vm.read_u16(cursor).unwrap_or(0);
            if value == 0 {
                break;
            }
            out.push(value);
            cursor = cursor.wrapping_add(2);
        }
        return out;
    }
    let mut out = Vec::with_capacity(len as usize);
    for i in 0..len {
        out.push(vm.read_u16(ptr + (i as u32) * 2).unwrap_or(0));
    }
    out
}

fn read_w_len(vm: &Vm, ptr: u32, len: i32) -> Vec<u16> {
    if len == 0 {
        return Vec::new();
    }
    if len < 0 {
        let mut out = Vec::new();
        let mut cursor = ptr;
        for _ in 0..0x10000 {
            let value = vm.read_u16(cursor).unwrap_or(0);
            if value == 0 {
                break;
            }
            out.push(value);
            cursor = cursor.wrapping_add(2);
        }
        return out;
    }
    let mut out = Vec::with_capacity(len as usize);
    for i in 0..len {
        out.push(vm.read_u16(ptr + (i as u32) * 2).unwrap_or(0));
    }
    out
}
