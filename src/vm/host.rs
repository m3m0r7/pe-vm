//! Host helper functions for VM stubs.

use super::Vm;
use crate::vm::windows;
use crate::vm_args;

pub fn host_printf(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    let text = vm.read_c_string(ptr).unwrap_or_default();
    vm.write_stdout(&text);
    text.len() as u32
}

pub fn host_message_box_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    windows::user32::message_box_a(vm, stack_ptr)
}

pub fn host_create_thread(vm: &mut Vm, stack_ptr: u32) -> u32 {
    windows::kernel32::create_thread(vm, stack_ptr)
}
