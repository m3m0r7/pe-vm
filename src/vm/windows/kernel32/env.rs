//! Kernel32 environment stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetEnvironmentVariableA", crate::vm::stdcall_args(3), get_environment_variable_a);
    vm.register_import_stdcall(DLL_NAME, "SetEnvironmentVariableA", crate::vm::stdcall_args(2), set_environment_variable_a);
    vm.register_import_stdcall(DLL_NAME, "ExpandEnvironmentStringsA", crate::vm::stdcall_args(3), expand_environment_strings_a);
    vm.register_import_stdcall(DLL_NAME, "GetEnvironmentStringsW", crate::vm::stdcall_args(0), get_environment_strings_w);
    vm.register_import_stdcall(DLL_NAME, "FreeEnvironmentStringsW", crate::vm::stdcall_args(1), free_environment_strings_w);
}

fn get_environment_variable_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (name_ptr, buffer_ptr, buffer_size) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if name_ptr == 0 {
        return 0;
    }
    let name = vm.read_c_string(name_ptr).unwrap_or_default();
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetEnvironmentVariableA: {name}");
    }
    let Some(value) = vm.env_value(&name) else {
        return 0;
    };
    if buffer_ptr == 0 || buffer_size == 0 {
        return (value.len() + 1) as u32;
    }
    let mut bytes = value.as_bytes().to_vec();
    if bytes.len() >= buffer_size {
        bytes.truncate(buffer_size.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(buffer_ptr, &bytes);
    (bytes.len().saturating_sub(1)) as u32
}

fn set_environment_variable_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (name_ptr, value_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if name_ptr == 0 {
        return 0;
    }
    let name = vm.read_c_string(name_ptr).unwrap_or_default();
    if value_ptr == 0 {
        vm.set_env_entry(name, None);
        return 1;
    }
    let value = vm.read_c_string(value_ptr).unwrap_or_default();
    vm.set_env_entry(name, Some(value));
    1
}

fn expand_environment_strings_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (src_ptr, dst_ptr, dst_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if src_ptr == 0 {
        return 0;
    }
    let src = vm.read_c_string(src_ptr).unwrap_or_default();
    let expanded = expand_env(vm, &src);
    let required = expanded.len() + 1;
    if dst_ptr == 0 || dst_len == 0 {
        return required as u32;
    }
    let mut bytes = expanded.into_bytes();
    if bytes.len() >= dst_len {
        bytes.truncate(dst_len.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(dst_ptr, &bytes);
    required as u32
}

fn get_environment_strings_w(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let strings = [0u16, 0u16];
    let bytes: Vec<u8> = strings.iter().flat_map(|value| value.to_le_bytes()).collect();
    vm.alloc_bytes(&bytes, 2).unwrap_or(0)
}

fn free_environment_strings_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn expand_env(vm: &Vm, input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            out.push(ch);
            continue;
        }
        let mut name = String::new();
        let mut closed = false;
        for next in chars.by_ref() {
            if next == '%' {
                closed = true;
                break;
            }
            name.push(next);
        }
        if !closed {
            out.push('%');
            out.push_str(&name);
            break;
        }
        if name.is_empty() {
            out.push('%');
            continue;
        }
        if let Some(value) = vm.env_value(&name) {
            out.push_str(value);
        } else {
            out.push('%');
            out.push_str(&name);
            out.push('%');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};
    use std::collections::BTreeMap;

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

    fn create_test_vm_with_env(vars: &[(&str, &str)]) -> Vm {
        let mut vm = create_test_vm();
        let mut env = BTreeMap::new();
        for (k, v) in vars {
            env.insert(k.to_string(), v.to_string());
        }
        vm.set_env(env);
        vm
    }

    fn write_string(vm: &mut Vm, addr: u32, s: &str) {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);
        vm.write_bytes(addr, &bytes).unwrap();
    }

    #[test]
    fn test_get_environment_variable_not_found() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let name_ptr = vm.heap_start as u32;
        write_string(&mut vm, name_ptr, "NONEXISTENT");
        vm.write_u32(stack + 4, name_ptr).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, 0).unwrap();
        let result = get_environment_variable_a(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_environment_variable_found() {
        let mut vm = create_test_vm_with_env(&[("TEST_VAR", "test_value")]);
        let stack = vm.stack_top - 16;
        let name_ptr = vm.heap_start as u32;
        let buf_ptr = name_ptr + 64;
        write_string(&mut vm, name_ptr, "TEST_VAR");
        vm.write_u32(stack + 4, name_ptr).unwrap();
        vm.write_u32(stack + 8, buf_ptr).unwrap();
        vm.write_u32(stack + 12, 64).unwrap();
        let result = get_environment_variable_a(&mut vm, stack);
        assert_eq!(result, 10); // "test_value".len()
        assert_eq!(vm.read_c_string(buf_ptr).unwrap(), "test_value");
    }

    #[test]
    fn test_get_environment_variable_query_size() {
        let mut vm = create_test_vm_with_env(&[("MYVAR", "hello")]);
        let stack = vm.stack_top - 16;
        let name_ptr = vm.heap_start as u32;
        write_string(&mut vm, name_ptr, "MYVAR");
        vm.write_u32(stack + 4, name_ptr).unwrap();
        vm.write_u32(stack + 8, 0).unwrap(); // null buffer
        vm.write_u32(stack + 12, 0).unwrap();
        let result = get_environment_variable_a(&mut vm, stack);
        assert_eq!(result, 6); // "hello".len() + 1
    }

    #[test]
    fn test_set_environment_variable() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let name_ptr = vm.heap_start as u32;
        let value_ptr = name_ptr + 32;
        write_string(&mut vm, name_ptr, "NEW_VAR");
        write_string(&mut vm, value_ptr, "new_value");
        vm.write_u32(stack + 4, name_ptr).unwrap();
        vm.write_u32(stack + 8, value_ptr).unwrap();
        let result = set_environment_variable_a(&mut vm, stack);
        assert_eq!(result, 1);

        // Verify it was set
        assert_eq!(vm.env_value("NEW_VAR"), Some("new_value"));
    }

    #[test]
    fn test_set_environment_variable_delete() {
        let mut vm = create_test_vm_with_env(&[("TO_DELETE", "value")]);
        let stack = vm.stack_top - 12;
        let name_ptr = vm.heap_start as u32;
        write_string(&mut vm, name_ptr, "TO_DELETE");
        vm.write_u32(stack + 4, name_ptr).unwrap();
        vm.write_u32(stack + 8, 0).unwrap(); // null value = delete
        let result = set_environment_variable_a(&mut vm, stack);
        assert_eq!(result, 1);
        assert_eq!(vm.env_value("TO_DELETE"), None);
    }

    #[test]
    fn test_expand_environment_strings_no_vars() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let src_ptr = vm.heap_start as u32;
        let dst_ptr = src_ptr + 64;
        write_string(&mut vm, src_ptr, "plain text");
        vm.write_u32(stack + 4, src_ptr).unwrap();
        vm.write_u32(stack + 8, dst_ptr).unwrap();
        vm.write_u32(stack + 12, 64).unwrap();
        let result = expand_environment_strings_a(&mut vm, stack);
        assert_eq!(result, 11); // "plain text".len() + 1
        assert_eq!(vm.read_c_string(dst_ptr).unwrap(), "plain text");
    }

    #[test]
    fn test_expand_environment_strings_with_var() {
        let mut vm = create_test_vm_with_env(&[("HOME", "/home/user")]);
        let stack = vm.stack_top - 16;
        let src_ptr = vm.heap_start as u32;
        let dst_ptr = src_ptr + 64;
        write_string(&mut vm, src_ptr, "path=%HOME%");
        vm.write_u32(stack + 4, src_ptr).unwrap();
        vm.write_u32(stack + 8, dst_ptr).unwrap();
        vm.write_u32(stack + 12, 64).unwrap();
        let result = expand_environment_strings_a(&mut vm, stack);
        assert_eq!(vm.read_c_string(dst_ptr).unwrap(), "path=/home/user");
        assert_eq!(result, 16); // "path=/home/user".len() + 1
    }

    #[test]
    fn test_expand_env_missing_var() {
        let vm = create_test_vm();
        let result = expand_env(&vm, "hello %MISSING% world");
        assert_eq!(result, "hello %MISSING% world");
    }

    #[test]
    fn test_expand_env_double_percent() {
        let vm = create_test_vm();
        let result = expand_env(&vm, "100%% complete");
        assert_eq!(result, "100% complete");
    }

    #[test]
    fn test_expand_env_unclosed() {
        let vm = create_test_vm();
        let result = expand_env(&vm, "unclosed %VAR");
        assert_eq!(result, "unclosed %VAR");
    }

    #[test]
    fn test_get_environment_strings_w() {
        let mut vm = create_test_vm();
        let result = get_environment_strings_w(&mut vm, 0);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_free_environment_strings_w() {
        let mut vm = create_test_vm();
        let result = free_environment_strings_w(&mut vm, 0);
        assert_eq!(result, 1);
    }
}
