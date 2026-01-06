use crate::vm::Vm;

use super::helpers::{compare_strings, read_string_arg, write_c_string};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "lstrlenA", crate::vm::stdcall_args(1), lstrlen_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcpyA", crate::vm::stdcall_args(2), lstrcpy_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcatA", crate::vm::stdcall_args(2), lstrcat_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcmpA", crate::vm::stdcall_args(2), lstrcmp_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcmpiA", crate::vm::stdcall_args(2), lstrcmpi_a);
    vm.register_import_stdcall("KERNEL32.dll", "lstrcpynA", crate::vm::stdcall_args(3), lstrcpyn_a);
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
