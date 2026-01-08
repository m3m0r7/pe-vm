use crate::vm::Vm;

pub(super) fn read_string_arg(vm: &mut Vm, ptr_addr: u32) -> String {
    let ptr = vm.read_u32(ptr_addr).unwrap_or(0);
    read_wide_or_utf16le_str!(vm, ptr)
}

pub(super) fn compare_strings(left: &str, right: &str) -> i32 {
    match left.cmp(right) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[allow(dead_code)]
pub(super) fn write_c_string(vm: &mut Vm, dest: u32, text: &str) {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    let _ = vm.write_bytes(dest, &bytes);
}

pub(super) fn read_bytes(vm: &Vm, ptr: u32, len: i32) -> Vec<u8> {
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

pub(super) fn read_utf16(vm: &Vm, ptr: u32, len: i32) -> Vec<u16> {
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

pub(super) fn read_w_len(vm: &Vm, ptr: u32, len: i32) -> Vec<u16> {
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
