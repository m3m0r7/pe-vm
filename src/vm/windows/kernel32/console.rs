//! Kernel32 console and stdio stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::windows::macros::{read_str_len, read_wstr_len};
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetConsoleCP", crate::vm::stdcall_args(0), get_console_cp);
    vm.register_import_stdcall(DLL_NAME, "GetConsoleMode", crate::vm::stdcall_args(2), get_console_mode);
    vm.register_import_stdcall(DLL_NAME, "GetStdHandle", crate::vm::stdcall_args(1), get_std_handle);
    vm.register_import_stdcall(DLL_NAME, "SetStdHandle", crate::vm::stdcall_args(2), set_std_handle);
    vm.register_import_stdcall(DLL_NAME, "ReadConsoleW", crate::vm::stdcall_args(5), read_console_w);
    vm.register_import_stdcall(DLL_NAME, "ReadFile", crate::vm::stdcall_args(5), read_file);
    vm.register_import_stdcall(DLL_NAME, "WriteConsoleW", crate::vm::stdcall_args(5), write_console_w);
    vm.register_import_stdcall(DLL_NAME, "WriteFile", crate::vm::stdcall_args(5), write_file);
}

fn get_console_cp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn get_console_mode(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out) = vm_args!(vm, stack_ptr; u32, u32);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    1
}

fn get_std_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (n_std_handle,) = vm_args!(vm, stack_ptr; u32);
    n_std_handle
}

fn set_std_handle(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn read_console_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, _, chars_read, _) = vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    if chars_read != 0 {
        let _ = vm.write_u32(chars_read, 0);
    }
    1
}

fn read_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle, buffer, count, bytes_read, _) = vm_args!(vm, stack_ptr; u32, u32, usize, u32, u32);
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
    let (_, buffer, count, written, _) = vm_args!(vm, stack_ptr; u32, u32, usize, u32, u32);
    if buffer != 0 && count > 0 {
        let text = read_wstr_len(vm, buffer, count);
        vm.write_stdout(&text);
        if written != 0 {
            let _ = vm.write_u32(written, count as u32);
        }
    }
    1
}

fn write_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle, buffer, count, written, _) = vm_args!(vm, stack_ptr; u32, u32, usize, u32, u32);
    if buffer != 0 && count > 0 {
        let bytes = read_str_len(vm, buffer, count);
        if let Some(wrote) = vm.file_write(handle, bytes.as_bytes()) {
            if written != 0 {
                let _ = vm.write_u32(written, wrote as u32);
            }
            return 1;
        }
        vm.write_stdout(&bytes);
        if written != 0 {
            let _ = vm.write_u32(written, count as u32);
        }
    }
    1
}
