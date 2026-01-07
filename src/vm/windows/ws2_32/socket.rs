//! Socket-related Winsock stubs.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::Vm;
use crate::vm_args;

use super::constants::{
    AF_INET, INVALID_SOCKET, SOCKET_ERROR, WSADATA_SIZE, WSADATA_VERSION, WSAEINVAL, WSAENOTSOCK,
};
use super::store::{alloc_socket, close_socket, set_last_error};
use super::trace::{log_connect, log_send, trace_net};
use super::util::{parse_ipv4, read_fd_count, read_sockaddr_in};

pub(super) fn bind(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

pub(super) fn closesocket(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle,) = vm_args!(vm, stack_ptr; u32);
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
    let (value,) = vm_args!(vm, stack_ptr; u32);
    let swapped = (value as u16).to_be();
    swapped as u32
}

pub(super) fn inet_addr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
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
    trace_net(&format!("WSA recv called buf=0x{buf:08X} len={len}"));
    if buf != 0 && len != 0 {
        let packet = build_ntp_response();
        let copy_len = (len as usize).min(packet.len());
        let _ = vm.write_bytes(buf, &packet[..copy_len]);
        trace_net(&format!(
            "WSA recv stubbed {copy_len} byte{}",
            if copy_len == 1 { "" } else { "s" }
        ));
        set_last_error(0);
        return copy_len as u32;
    }
    set_last_error(0);
    0
}

fn build_ntp_response() -> [u8; 48] {
    let mut packet = [0u8; 48];
    packet[0] = 0x1C; // LI=0, VN=3, Mode=4 (server)
    packet[1] = 1; // stratum
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let ntp_seconds = now.as_secs().wrapping_add(2_208_988_800);
    let ntp_fraction = ((now.subsec_nanos() as u64) << 32) / 1_000_000_000;
    packet[40..44].copy_from_slice(&(ntp_seconds as u32).to_be_bytes());
    packet[44..48].copy_from_slice(&(ntp_fraction as u32).to_be_bytes());
    packet
}

pub(super) fn select(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, read_ptr, write_ptr, except_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);

    let read_count = read_fd_count(vm, read_ptr);
    let write_count = read_fd_count(vm, write_ptr);
    let except_count = read_fd_count(vm, except_ptr);

    set_last_error(0);
    read_count
        .saturating_add(write_count)
        .saturating_add(except_count)
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

pub(super) fn ioctlsocket(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _cmd, argp) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if argp != 0 {
        let _ = vm.write_u32(argp, 0);
    }
    set_last_error(0);
    0
}

pub(super) fn gethostbyname(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (name_ptr,) = vm_args!(vm, stack_ptr; u32);
    if name_ptr == 0 {
        set_last_error(WSAEINVAL);
        return 0;
    }
    let name = vm.read_c_string(name_ptr).unwrap_or_default();
    if name.is_empty() {
        set_last_error(WSAEINVAL);
        return 0;
    }
    let addr = parse_ipv4(&name).unwrap_or(u32::from_be_bytes([127, 0, 0, 1]));
    let ptr = alloc_hostent(vm, &name, addr);
    set_last_error(0);
    ptr
}

pub(super) fn gethostbyaddr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (addr_ptr, len, addr_type) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if addr_ptr == 0 || len < 4 || addr_type as u16 != AF_INET {
        set_last_error(WSAEINVAL);
        return 0;
    }
    let addr = vm.read_u32(addr_ptr).unwrap_or(0);
    let name = "ntp.local";
    let ptr = alloc_hostent(vm, name, addr);
    set_last_error(0);
    ptr
}

fn alloc_hostent(vm: &mut Vm, name: &str, addr: u32) -> u32 {
    let name_ptr = alloc_c_string(vm, name);
    let aliases_ptr = alloc_u32_list(vm, &[0]);
    let addr_bytes = addr.to_be_bytes();
    let addr_ptr = vm.alloc_bytes(&addr_bytes, 4).unwrap_or(0);
    let addr_list_ptr = alloc_u32_list(vm, &[addr_ptr, 0]);

    if name_ptr == 0 || aliases_ptr == 0 || addr_ptr == 0 || addr_list_ptr == 0 {
        return 0;
    }

    let mut hostent = [0u8; 16];
    hostent[0..4].copy_from_slice(&name_ptr.to_le_bytes());
    hostent[4..8].copy_from_slice(&aliases_ptr.to_le_bytes());
    hostent[8..10].copy_from_slice(&AF_INET.to_le_bytes());
    hostent[10..12].copy_from_slice(&4u16.to_le_bytes());
    hostent[12..16].copy_from_slice(&addr_list_ptr.to_le_bytes());
    vm.alloc_bytes(&hostent, 4).unwrap_or(0)
}

fn alloc_c_string(vm: &mut Vm, value: &str) -> u32 {
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    vm.alloc_bytes(&bytes, 1).unwrap_or(0)
}

fn alloc_u32_list(vm: &mut Vm, values: &[u32]) -> u32 {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4).unwrap_or(0)
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
