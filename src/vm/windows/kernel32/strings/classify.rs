use crate::vm::Vm;
use crate::vm_args;

use super::helpers::read_w_len;

pub(super) fn register(vm: &mut Vm) {
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
    vm.register_import_stdcall("KERNEL32.dll", "MulDiv", crate::vm::stdcall_args(3), mul_div);
}

fn get_string_type_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_type, src_ptr, src_len, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, i32, u32);
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
    let (_locale, _flags, left_ptr, left_len, right_ptr, right_len) = vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, i32);
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
    let (_locale, flags, src_ptr, src_len, dst_ptr, dst_len) = vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize);
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
    let (number, numerator, denominator) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let number = number as i64;
    let numerator = numerator as i64;
    let denominator = denominator as i64;
    if denominator == 0 {
        return 0;
    }
    let value = number.saturating_mul(numerator).saturating_add(denominator / 2) / denominator;
    value as u32
}
