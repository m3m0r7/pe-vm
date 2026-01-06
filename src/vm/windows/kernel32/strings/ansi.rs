use crate::vm::Vm;

use super::helpers::{compare_strings, read_bytes, read_string_arg};

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
    read_bytes(vm, ptr, -1).len() as u32
}

fn lstrcpy_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if dest == 0 || src == 0 {
        return dest;
    }
    let text = read_bytes(vm, src, -1);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let raw = read_raw_bytes(vm, src, 32);
        eprintln!(
            "[pe_vm] lstrcpyA dest=0x{dest:08X} src=0x{src:08X} text={:?} raw={raw}",
            render_bytes(&text)
        );
    }
    write_c_bytes(vm, dest, &text);
    dest
}

fn lstrcat_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if dest == 0 || src == 0 {
        return dest;
    }
    let mut dest_text = read_bytes(vm, dest, -1);
    let src_text = read_bytes(vm, src, -1);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let dest_raw = read_raw_bytes(vm, dest, 32);
        let src_raw = read_raw_bytes(vm, src, 32);
        eprintln!(
            "[pe_vm] lstrcatA dest=0x{dest:08X} src=0x{src:08X} dest_text={:?} src_text={:?} dest_raw={dest_raw} src_raw={src_raw}",
            render_bytes(&dest_text),
            render_bytes(&src_text)
        );
    }
    dest_text.extend_from_slice(&src_text);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] lstrcatA dest=0x{dest:08X} result_text={:?}",
            render_bytes(&dest_text)
        );
    }
    write_c_bytes(vm, dest, &dest_text);
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
    let mut bytes = read_bytes(vm, src, -1);
    if bytes.len() >= count {
        bytes.truncate(count.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(dest, &bytes);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let src_raw = read_raw_bytes(vm, src, 32);
        let rendered = render_bytes(&bytes[..bytes.len().saturating_sub(1)]);
        eprintln!(
            "[pe_vm] lstrcpynA dest=0x{dest:08X} src=0x{src:08X} count={count} text={rendered:?} src_raw={src_raw}"
        );
    }
    dest
}

fn read_raw_bytes(vm: &Vm, ptr: u32, len: usize) -> String {
    let mut out = String::new();
    for idx in 0..len {
        let byte = vm.read_u8(ptr.wrapping_add(idx as u32)).unwrap_or(0);
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02X}"));
    }
    out
}

fn write_c_bytes(vm: &mut Vm, dest: u32, bytes: &[u8]) {
    let mut out = bytes.to_vec();
    out.push(0);
    let _ = vm.write_bytes(dest, &out);
}

fn render_bytes(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}
