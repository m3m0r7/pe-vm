//! Winsock event helpers.

use crate::vm::Vm;
use crate::vm_args;

use super::constants::{WSAEINVAL, WSANETWORKEVENTS_SIZE};
use super::store::{alloc_event, set_last_error};

pub(super) fn wsa_create_event(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    alloc_event()
}

pub(super) fn wsa_close_event(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle,) = vm_args!(vm, stack_ptr; u32);
    if handle == 0 {
        set_last_error(WSAEINVAL);
        return 0;
    }
    set_last_error(0);
    1
}

pub(super) fn wsa_event_select(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn wsa_enum_network_events(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, events_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if events_ptr != 0 {
        let _ = vm.memset(events_ptr, 0, WSANETWORKEVENTS_SIZE);
    }
    set_last_error(0);
    0
}

pub(super) fn wsa_wait_for_multiple_events(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn wsafd_is_set(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle, set_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if set_ptr == 0 {
        return 0;
    }
    let count = vm.read_u32(set_ptr).unwrap_or(0);
    let mut cursor = set_ptr + 4;
    for _ in 0..count {
        let value = vm.read_u32(cursor).unwrap_or(0);
        if value == handle {
            return 1;
        }
        cursor = cursor.wrapping_add(4);
    }
    0
}
