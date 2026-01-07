//! Socket-related Winsock stubs.

use crate::vm::Vm;
use crate::vm_args;

use super::constants::{INVALID_SOCKET, SOCKET_ERROR, WSAEINVAL, WSAENOTSOCK, WSADATA_SIZE, WSADATA_VERSION};
use super::store::{alloc_socket, close_socket, set_last_error};
use super::trace::{log_connect, log_send, trace_net};
use super::util::{parse_ipv4, read_fd_count, read_sockaddr_in};

pub(super) fn bind(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn closesocket(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [handle] = vm_args!(vm, stack_ptr; u32);
    if handle == 0 {
        set_last_error(WSAENOTSOCK);
        return SOCKET_ERROR;
    }
    if close_socket(handle) {
        set_last_error(0);
        0
    } else {
        set_last_error(WSAENOTSOCK);
        SOCKET_ERROR
    }
}

pub(super) fn connect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, addr_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if let Some((host, port)) = read_sockaddr_in(vm, addr_ptr) {
        log_connect(&host, port);
    }
    set_last_error(0);
    0
}

pub(super) fn htons(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [value] = vm_args!(vm, stack_ptr; u32);
    let swapped = (value as u16).to_be();
    swapped as u32
}

pub(super) fn inet_addr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [ptr] = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return INVALID_SOCKET;
    }
    let text = vm.read_c_string(ptr).unwrap_or_default();
    match parse_ipv4(&text) {
        Some(addr) => addr,
        None => INVALID_SOCKET,
    }
}

pub(super) fn listen(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn recv(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, buf, len) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if buf != 0 && len != 0 {
        let _ = vm.memset(buf, 0, len as usize);
        trace_net(&format!("WSA recv stubbed {len} bytes"));
    }
    set_last_error(0);
    len
}

pub(super) fn select(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, read_ptr, write_ptr, except_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);

    let read_count = read_fd_count(vm, read_ptr);
    let write_count = read_fd_count(vm, write_ptr);
    let except_count = read_fd_count(vm, except_ptr);

    set_last_error(0);
    read_count.saturating_add(write_count).saturating_add(except_count)
}

pub(super) fn send(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, buf, len) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if buf != 0 && len != 0 {
        let mut bytes = Vec::with_capacity(len as usize);
        for offset in 0..len {
            if let Ok(value) = vm.read_u8(buf.wrapping_add(offset)) {
                bytes.push(value);
            }
        }
        log_send(&bytes);
    }
    set_last_error(0);
    len
}

pub(super) fn setsockopt(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn shutdown(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn socket(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    alloc_socket()
}

pub(super) fn wsa_startup(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (version, data_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    let version = version as u16;
    if data_ptr != 0 {
        let _ = vm.memset(data_ptr, 0, WSADATA_SIZE);
        let _ = vm.write_u16(data_ptr, version);
        let _ = vm.write_u16(data_ptr + 2, WSADATA_VERSION);
    } else {
        set_last_error(WSAEINVAL);
        return WSAEINVAL;
    }
    set_last_error(0);
    0
}

pub(super) fn wsa_cleanup(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}
