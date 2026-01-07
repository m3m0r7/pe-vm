use crate::vm::Vm;
use crate::vm_args;

use super::helpers::{compare_strings, read_bytes, read_string_arg};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrlenA",
        crate::vm::stdcall_args(1),
        lstrlen_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrcpyA",
        crate::vm::stdcall_args(2),
        lstrcpy_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrcatA",
        crate::vm::stdcall_args(2),
        lstrcat_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrcmpA",
        crate::vm::stdcall_args(2),
        lstrcmp_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrcmpiA",
        crate::vm::stdcall_args(2),
        lstrcmpi_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "lstrcpynA",
        crate::vm::stdcall_args(3),
        lstrcpyn_a,
    );
}

fn lstrlen_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    read_bytes(vm, ptr, -1).len() as u32
}

fn lstrcpy_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dest, src) = vm_args!(vm, stack_ptr; u32, u32);
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
    let (dest, src) = vm_args!(vm, stack_ptr; u32, u32);
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
    let (dest, src, count) = vm_args!(vm, stack_ptr; u32, u32, usize);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};
    use crate::vm_set_args;

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    fn write_string(vm: &mut Vm, addr: u32, s: &str) {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);
        vm.write_bytes(addr, &bytes).unwrap();
    }

    #[test]
    fn test_lstrlen_a_empty() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let str_ptr = vm.heap_start as u32;
        vm.write_u8(str_ptr, 0).unwrap(); // empty string
        vm.write_u32(stack + 4, str_ptr).unwrap();
        let len = lstrlen_a(&mut vm, stack);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_lstrlen_a_hello() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let str_ptr = vm.heap_start as u32;
        write_string(&mut vm, str_ptr, "Hello");
        vm.write_u32(stack + 4, str_ptr).unwrap();
        let len = lstrlen_a(&mut vm, stack);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_lstrlen_a_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let len = lstrlen_a(&mut vm, stack);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_lstrcpy_a() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let dest = vm.heap_start as u32;
        let src = dest + 64;
        write_string(&mut vm, src, "Hello");
        vm_set_args!(vm, stack; dest, src);
        let result = lstrcpy_a(&mut vm, stack);
        assert_eq!(result, dest);
        assert_eq!(vm.read_c_string(dest).unwrap(), "Hello");
    }

    #[test]
    fn test_lstrcpy_a_null_returns_dest() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let dest = vm.heap_start as u32;
        vm_set_args!(vm, stack; dest, 0u32);
        let result = lstrcpy_a(&mut vm, stack);
        assert_eq!(result, dest);
    }

    #[test]
    fn test_lstrcat_a() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let dest = vm.heap_start as u32;
        let src = dest + 64;
        write_string(&mut vm, dest, "Hello");
        write_string(&mut vm, src, "World");
        vm_set_args!(vm, stack; dest, src);
        let result = lstrcat_a(&mut vm, stack);
        assert_eq!(result, dest);
        assert_eq!(vm.read_c_string(dest).unwrap(), "HelloWorld");
    }

    #[test]
    fn test_lstrcmp_a_equal() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let s1 = vm.heap_start as u32;
        let s2 = s1 + 32;
        write_string(&mut vm, s1, "test");
        write_string(&mut vm, s2, "test");
        vm_set_args!(vm, stack; s1, s2);
        let result = lstrcmp_a(&mut vm, stack) as i32;
        assert_eq!(result, 0);
    }

    #[test]
    fn test_lstrcmp_a_less() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let s1 = vm.heap_start as u32;
        let s2 = s1 + 32;
        write_string(&mut vm, s1, "aaa");
        write_string(&mut vm, s2, "bbb");
        vm_set_args!(vm, stack; s1, s2);
        let result = lstrcmp_a(&mut vm, stack) as i32;
        assert!(result < 0);
    }

    #[test]
    fn test_lstrcmp_a_greater() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let s1 = vm.heap_start as u32;
        let s2 = s1 + 32;
        write_string(&mut vm, s1, "zzz");
        write_string(&mut vm, s2, "aaa");
        vm_set_args!(vm, stack; s1, s2);
        let result = lstrcmp_a(&mut vm, stack) as i32;
        assert!(result > 0);
    }

    #[test]
    fn test_lstrcmpi_a_case_insensitive() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let s1 = vm.heap_start as u32;
        let s2 = s1 + 32;
        write_string(&mut vm, s1, "TEST");
        write_string(&mut vm, s2, "test");
        vm_set_args!(vm, stack; s1, s2);
        let result = lstrcmpi_a(&mut vm, stack) as i32;
        assert_eq!(result, 0);
    }

    #[test]
    fn test_lstrcpyn_a() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let dest = vm.heap_start as u32;
        let src = dest + 64;
        write_string(&mut vm, src, "HelloWorld");
        vm_set_args!(vm, stack; dest, src, 6u32);
        let result = lstrcpyn_a(&mut vm, stack);
        assert_eq!(result, dest);
        assert_eq!(vm.read_c_string(dest).unwrap(), "Hello");
    }

    #[test]
    fn test_lstrcpyn_a_full_copy() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let dest = vm.heap_start as u32;
        let src = dest + 64;
        write_string(&mut vm, src, "Hi");
        vm_set_args!(vm, stack; dest, src, 10u32);
        let result = lstrcpyn_a(&mut vm, stack);
        assert_eq!(result, dest);
        assert_eq!(vm.read_c_string(dest).unwrap(), "Hi");
    }
}
