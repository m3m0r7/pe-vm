//! Kernel32 console and stdio stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetConsoleCP", crate::vm::stdcall_args(0), get_console_cp);
    vm.register_import_stdcall("KERNEL32.dll", "GetConsoleMode", crate::vm::stdcall_args(2), get_console_mode);
    vm.register_import_stdcall("KERNEL32.dll", "GetStdHandle", crate::vm::stdcall_args(1), get_std_handle);
    vm.register_import_stdcall("KERNEL32.dll", "SetStdHandle", crate::vm::stdcall_args(2), set_std_handle);
    vm.register_import_stdcall("KERNEL32.dll", "ReadConsoleW", crate::vm::stdcall_args(5), read_console_w);
    vm.register_import_stdcall("KERNEL32.dll", "ReadFile", crate::vm::stdcall_args(5), read_file);
    vm.register_import_stdcall("KERNEL32.dll", "WriteConsoleW", crate::vm::stdcall_args(5), write_console_w);
    vm.register_import_stdcall("KERNEL32.dll", "WriteFile", crate::vm::stdcall_args(5), write_file);
}

fn get_console_cp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn get_console_mode(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    1
}

fn get_std_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 4).unwrap_or(0)
}

fn set_std_handle(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn read_console_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let chars_read = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if chars_read != 0 {
        let _ = vm.write_u32(chars_read, 0);
    }
    1
}

fn read_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let count = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    let bytes_read = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if let Some(bytes) = vm.file_read(handle, count) {
        if buffer != 0 {
            let _ = vm.write_bytes(buffer, &bytes);
        }
        if bytes_read != 0 {
            let _ = vm.write_u32(bytes_read, bytes.len() as u32);
        }
        return 1;
    }
    if bytes_read != 0 {
        let _ = vm.write_u32(bytes_read, 0);
    }
    1
}

fn write_console_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let count = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let written = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if buffer != 0 && count > 0 {
        let mut units = Vec::with_capacity(count as usize);
        for i in 0..count {
            if let Ok(unit) = vm.read_u16(buffer + i * 2) {
                units.push(unit);
            }
        }
        let text = String::from_utf16_lossy(&units);
        vm.write_stdout(&text);
        if written != 0 {
            let _ = vm.write_u32(written, count);
        }
    }
    1
}

fn write_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let count = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let written = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if buffer != 0 && count > 0 {
        let mut bytes = Vec::with_capacity(count as usize);
        for i in 0..count {
            if let Ok(byte) = vm.read_u8(buffer + i) {
                bytes.push(byte);
            }
        }
        if let Some(wrote) = vm.file_write(handle, &bytes) {
            if written != 0 {
                let _ = vm.write_u32(written, wrote as u32);
            }
            return 1;
        }
        let text = String::from_utf8_lossy(&bytes);
        vm.write_stdout(&text);
        if written != 0 {
            let _ = vm.write_u32(written, count);
        }
    }
    1
}
