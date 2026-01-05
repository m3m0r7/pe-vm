//! Kernel32 heap/global memory stubs.

use crate::vm::Vm;

const HEAP_HANDLE: u32 = 0x1000;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetProcessHeap", crate::vm::stdcall_args(0), get_process_heap);
    vm.register_import_stdcall("KERNEL32.dll", "HeapAlloc", crate::vm::stdcall_args(3), heap_alloc);
    vm.register_import_stdcall("KERNEL32.dll", "HeapReAlloc", crate::vm::stdcall_args(4), heap_realloc);
    vm.register_import_stdcall("KERNEL32.dll", "HeapFree", crate::vm::stdcall_args(3), heap_free);
    vm.register_import_stdcall("KERNEL32.dll", "HeapSize", crate::vm::stdcall_args(3), heap_size);
    vm.register_import_stdcall("KERNEL32.dll", "HeapDestroy", crate::vm::stdcall_args(1), heap_destroy);
    vm.register_import_stdcall("KERNEL32.dll", "GlobalAlloc", crate::vm::stdcall_args(2), global_alloc);
    vm.register_import_stdcall("KERNEL32.dll", "GlobalFree", crate::vm::stdcall_args(1), global_free);
    vm.register_import_stdcall("KERNEL32.dll", "GlobalLock", crate::vm::stdcall_args(1), global_lock);
    vm.register_import_stdcall("KERNEL32.dll", "GlobalUnlock", crate::vm::stdcall_args(1), global_unlock);
    vm.register_import_stdcall("KERNEL32.dll", "GlobalHandle", crate::vm::stdcall_args(1), global_handle);
    vm.register_import_stdcall("KERNEL32.dll", "LocalFree", crate::vm::stdcall_args(1), local_free);
    vm.register_import_stdcall("KERNEL32.dll", "VirtualAlloc", crate::vm::stdcall_args(4), virtual_alloc);
    vm.register_import_stdcall("KERNEL32.dll", "VirtualFree", crate::vm::stdcall_args(3), virtual_free);
    vm.register_import_stdcall("KERNEL32.dll", "VirtualProtect", crate::vm::stdcall_args(4), virtual_protect);
    vm.register_import_stdcall("KERNEL32.dll", "VirtualQuery", crate::vm::stdcall_args(3), virtual_query);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FlushInstructionCache",
        crate::vm::stdcall_args(3),
        flush_instruction_cache,
    );
}

fn get_process_heap(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    HEAP_HANDLE
}

fn heap_alloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _heap = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    vm.heap_alloc(size)
}

fn heap_realloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _heap = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let _flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let mem = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    vm.heap_realloc(mem, size)
}

fn heap_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let mem = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if mem == 0 {
        return 0;
    }
    if vm.heap_free(mem) {
        1
    } else {
        0
    }
}

fn heap_size(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let mem = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if let Some(size) = vm.heap_size(mem) {
        size as u32
    } else {
        0xFFFF_FFFF
    }
}

fn heap_destroy(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn global_alloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _flags = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    vm.heap_alloc(size)
}

fn global_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn global_lock(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 4).unwrap_or(0)
}

fn global_unlock(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn global_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 4).unwrap_or(0)
}

fn local_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn virtual_alloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let _addr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let _alloc_type = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let _protect = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    vm.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0)
}

fn virtual_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn virtual_protect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let old_protect_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if old_protect_ptr != 0 {
        let _ = vm.write_u32(old_protect_ptr, 0x04);
    }
    1
}

fn virtual_query(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if info_ptr == 0 || len < 28 {
        return 0;
    }
    let base = vm.base();
    let _ = vm.write_u32(info_ptr, base);
    let _ = vm.write_u32(info_ptr + 4, base);
    let _ = vm.write_u32(info_ptr + 8, 0x04);
    let _ = vm.write_u32(info_ptr + 12, 0x1000);
    let _ = vm.write_u32(info_ptr + 16, 0x1000);
    let _ = vm.write_u32(info_ptr + 20, 0x04);
    let _ = vm.write_u32(info_ptr + 24, 0x20000);
    28
}

fn flush_instruction_cache(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
