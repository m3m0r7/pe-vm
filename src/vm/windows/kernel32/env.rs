//! Kernel32 environment stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetEnvironmentVariableA",
        crate::vm::stdcall_args(3),
        get_environment_variable_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SetEnvironmentVariableA",
        crate::vm::stdcall_args(2),
        set_environment_variable_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "ExpandEnvironmentStringsA",
        crate::vm::stdcall_args(3),
        expand_environment_strings_a,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetEnvironmentStringsW", crate::vm::stdcall_args(0), get_environment_strings_w);
    vm.register_import_stdcall("KERNEL32.dll", "FreeEnvironmentStringsW", crate::vm::stdcall_args(1), free_environment_strings_w);
}

fn get_environment_variable_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let buffer_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let buffer_size = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
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
    let name_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
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
    let src_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let dst_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
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
