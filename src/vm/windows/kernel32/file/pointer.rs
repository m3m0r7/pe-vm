use crate::vm::Vm;

use super::constants::{ERROR_INVALID_HANDLE, FILE_BEGIN, INVALID_HANDLE_VALUE};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "SetEndOfFile", crate::vm::stdcall_args(1), set_end_of_file);
    vm.register_import_stdcall("KERNEL32.dll", "SetFilePointer", crate::vm::stdcall_args(4), set_file_pointer);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SetFilePointerEx",
        crate::vm::stdcall_args(5),
        set_file_pointer_ex,
    );
    vm.register_import_stdcall("KERNEL32.dll", "SetFileTime", crate::vm::stdcall_args(4), set_file_time);
}

fn set_end_of_file(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_file_pointer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let distance = vm.read_u32(stack_ptr + 8).unwrap_or(0) as i32 as i64;
    let high_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let method = vm.read_u32(stack_ptr + 16).unwrap_or(FILE_BEGIN);
    let mut offset = distance;
    if high_ptr != 0 {
        let high = vm.read_u32(high_ptr).unwrap_or(0) as i32 as i64;
        offset |= high << 32;
    }
    match vm.file_seek(handle, offset, method) {
        Some(pos) => {
            if high_ptr != 0 {
                let _ = vm.write_u32(high_ptr, (pos >> 32) as u32);
            }
            pos as u32
        }
        None => {
            vm.set_last_error(ERROR_INVALID_HANDLE);
            INVALID_HANDLE_VALUE
        }
    }
}

fn set_file_pointer_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let low = vm.read_u32(stack_ptr + 8).unwrap_or(0) as u64;
    let high = vm.read_u32(stack_ptr + 12).unwrap_or(0) as u64;
    let out = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let method = vm.read_u32(stack_ptr + 20).unwrap_or(FILE_BEGIN);
    let offset = ((high << 32) | low) as i64;
    match vm.file_seek(handle, offset, method) {
        Some(pos) => {
            if out != 0 {
                let _ = vm.write_u32(out, pos as u32);
                let _ = vm.write_u32(out + 4, (pos >> 32) as u32);
            }
            1
        }
        None => {
            vm.set_last_error(ERROR_INVALID_HANDLE);
            0
        }
    }
}

fn set_file_time(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
