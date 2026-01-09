//! Kernel32 interlocked operation stubs.

use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedIncrement",
        crate::vm::stdcall_args(1),
        interlocked_increment,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedExchange",
        crate::vm::stdcall_args(2),
        interlocked_exchange,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedDecrement",
        crate::vm::stdcall_args(1),
        interlocked_decrement,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedPushEntrySList",
        crate::vm::stdcall_args(2),
        interlocked_push_entry_slist,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedPopEntrySList",
        crate::vm::stdcall_args(1),
        interlocked_pop_entry_slist,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InterlockedFlushSList",
        crate::vm::stdcall_args(1),
        interlocked_flush_slist,
    );
}

fn interlocked_increment(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (addr,) = vm_args!(vm, stack_ptr; u32);
    if addr == 0 {
        return 0;
    }
    let value = vm.read_u32(addr).unwrap_or(0).wrapping_add(1);
    let _ = vm.write_u32(addr, value);
    value
}

fn interlocked_exchange(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (addr, value) = vm_args!(vm, stack_ptr; u32, u32);
    if addr == 0 {
        return 0;
    }
    let prev = vm.read_u32(addr).unwrap_or(0);
    let _ = vm.write_u32(addr, value);
    prev
}

fn interlocked_decrement(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (addr,) = vm_args!(vm, stack_ptr; u32);
    if addr == 0 {
        return 0;
    }
    let value = vm.read_u32(addr).unwrap_or(0).wrapping_sub(1);
    let _ = vm.write_u32(addr, value);
    value
}

fn interlocked_push_entry_slist(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (header, entry) = vm_args!(vm, stack_ptr; u32, u32);
    if header == 0 || entry == 0 {
        return 0;
    }
    let head = vm.read_u32(header).unwrap_or(0);
    let _ = vm.write_u32(entry, head);
    let _ = vm.write_u32(header, entry);
    head
}

fn interlocked_pop_entry_slist(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (header,) = vm_args!(vm, stack_ptr; u32);
    if header == 0 {
        return 0;
    }
    let head = vm.read_u32(header).unwrap_or(0);
    if head == 0 {
        return 0;
    }
    let next = vm.read_u32(head).unwrap_or(0);
    let _ = vm.write_u32(header, next);
    head
}

fn interlocked_flush_slist(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (header,) = vm_args!(vm, stack_ptr; u32);
    if header == 0 {
        return 0;
    }
    let head = vm.read_u32(header).unwrap_or(0);
    let _ = vm.write_u32(header, 0);
    let _ = vm.write_u32(header.wrapping_add(4), 0);
    head
}
