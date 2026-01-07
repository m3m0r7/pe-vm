//! Kernel32 heap/global memory stubs.

use crate::vm::Vm;
use crate::vm_args;

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
    let (_heap, _flags, size) = vm_args!(vm, stack_ptr; u32, u32, usize);
    vm.heap_alloc(size)
}

fn heap_realloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_heap, _flags, mem, size) = vm_args!(vm, stack_ptr; u32, u32, u32, usize);
    vm.heap_realloc(mem, size)
}

fn heap_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, mem) = vm_args!(vm, stack_ptr; u32, u32, u32);
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
    let (_, _, mem) = vm_args!(vm, stack_ptr; u32, u32, u32);
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
    let (_flags, size) = vm_args!(vm, stack_ptr; u32, usize);
    vm.heap_alloc(size)
}

fn global_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn global_lock(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [h_mem] = vm_args!(vm, stack_ptr; u32);
    h_mem
}

fn global_unlock(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn global_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [p_mem] = vm_args!(vm, stack_ptr; u32);
    p_mem
}

fn local_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn virtual_alloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_addr, size, _alloc_type, _protect) = vm_args!(vm, stack_ptr; u32, usize, u32, u32);
    vm.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0)
}

fn virtual_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn virtual_protect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, _, old_protect_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    if old_protect_ptr != 0 {
        let _ = vm.write_u32(old_protect_ptr, 0x04);
    }
    1
}

fn virtual_query(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, info_ptr, len) = vm_args!(vm, stack_ptr; u32, u32, usize);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

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

    #[test]
    fn test_get_process_heap_returns_handle() {
        let mut vm = create_test_vm();
        let result = get_process_heap(&mut vm, 0);
        assert_eq!(result, HEAP_HANDLE);
    }

    #[test]
    fn test_heap_alloc_returns_nonzero() {
        let mut vm = create_test_vm();
        // Setup stack: heap handle, flags, size
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 4, HEAP_HANDLE).unwrap();
        vm.write_u32(stack + 8, 0).unwrap(); // flags
        vm.write_u32(stack + 12, 64).unwrap(); // size
        let ptr = heap_alloc(&mut vm, stack);
        assert_ne!(ptr, 0);
    }

    #[test]
    fn test_heap_alloc_and_free() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 4, HEAP_HANDLE).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, 64).unwrap();
        let ptr = heap_alloc(&mut vm, stack);
        assert_ne!(ptr, 0);

        // Free it
        vm.write_u32(stack + 12, ptr).unwrap();
        let result = heap_free(&mut vm, stack);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_heap_free_null_returns_zero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 12, 0).unwrap(); // null pointer
        let result = heap_free(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_heap_size_returns_size() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 4, HEAP_HANDLE).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, 128).unwrap();
        let ptr = heap_alloc(&mut vm, stack);

        vm.write_u32(stack + 12, ptr).unwrap();
        let size = heap_size(&mut vm, stack);
        assert!(size >= 128);
    }

    #[test]
    fn test_heap_size_invalid_returns_error() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 12, 0xDEAD_BEEF).unwrap(); // invalid ptr
        let size = heap_size(&mut vm, stack);
        assert_eq!(size, 0xFFFF_FFFF);
    }

    #[test]
    fn test_global_alloc_returns_nonzero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 4, 0).unwrap(); // flags
        vm.write_u32(stack + 8, 32).unwrap(); // size
        let ptr = global_alloc(&mut vm, stack);
        assert_ne!(ptr, 0);
    }

    #[test]
    fn test_global_lock_returns_same_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let handle = 0x1234;
        vm.write_u32(stack + 4, handle).unwrap();
        let result = global_lock(&mut vm, stack);
        assert_eq!(result, handle);
    }

    #[test]
    fn test_global_unlock_returns_one() {
        let mut vm = create_test_vm();
        let result = global_unlock(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_virtual_alloc_returns_nonzero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        vm.write_u32(stack + 4, 0).unwrap(); // addr (NULL = let system choose)
        vm.write_u32(stack + 8, 0x1000).unwrap(); // size
        vm.write_u32(stack + 12, 0x1000).unwrap(); // MEM_COMMIT
        vm.write_u32(stack + 16, 0x04).unwrap(); // PAGE_READWRITE
        let ptr = virtual_alloc(&mut vm, stack);
        assert_ne!(ptr, 0);
    }

    #[test]
    fn test_virtual_free_returns_one() {
        let mut vm = create_test_vm();
        let result = virtual_free(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_virtual_protect_writes_old_protect() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let old_protect_ptr = vm.heap_start as u32;
        vm.write_u32(stack + 16, old_protect_ptr).unwrap();
        let result = virtual_protect(&mut vm, stack);
        assert_eq!(result, 1);
        assert_eq!(vm.read_u32(old_protect_ptr).unwrap(), 0x04);
    }

    #[test]
    fn test_heap_destroy_returns_one() {
        let mut vm = create_test_vm();
        let result = heap_destroy(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_flush_instruction_cache_returns_one() {
        let mut vm = create_test_vm();
        let result = flush_instruction_cache(&mut vm, 0);
        assert_eq!(result, 1);
    }
}
