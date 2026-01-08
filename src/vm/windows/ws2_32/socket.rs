//! Socket-related Winsock stubs.

use crate::vm::Vm;
use crate::vm_args;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs, UdpSocket};
use std::time::Duration;

use super::constants::{
    AF_INET, INVALID_SOCKET, SOCKET_ERROR, WSADATA_SIZE, WSADATA_VERSION, WSAEINVAL, WSAENOTSOCK,
    WSAEWOULDBLOCK,
};
use super::store::{alloc_socket, close_socket, register_socket, set_last_error, socket_by_handle};
use super::trace::{log_connect, log_send, trace_net};
use super::util::{parse_ipv4, read_fd_count, read_sockaddr_in};

fn network_fallback_host(vm: &Vm) -> Option<&str> {
    vm.config()
        .sandbox_config()
        .and_then(|sandbox| sandbox.network_fallback_host())
}

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
    let (handle, addr_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    trace_net(&format!(
        "WSA connect handle=0x{handle:08X} addr=0x{addr_ptr:08X}"
    ));
    if let Some((host, port)) = read_sockaddr_in(vm, addr_ptr) {
        let port = u16::from_be(port);
        // Use sandbox fallback host if configured
        let target_host = network_fallback_host(vm)
            .map(|s| s.to_string())
            .unwrap_or(host);
        if let Ok(ip) = target_host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(ip, port);
            if let Some(socket) = socket_by_handle(handle) {
                log_connect(&target_host, port);
                if socket.connect(addr).is_ok() {
                    set_last_error(0);
                    return 0;
                }
            } else {
                set_last_error(WSAENOTSOCK);
                return SOCKET_ERROR;
            }
        }
    }
    set_last_error(WSAEINVAL);
    SOCKET_ERROR
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
    let (handle, buf_ptr, len, _flags) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    trace_net(&format!("WSA recv called buf=0x{buf_ptr:08X} len={len}"));
    if len == 0 {
        set_last_error(0);
        return 0;
    }
    let mut buffer = vec![0u8; len as usize];
    if let Some(socket) = socket_by_handle(handle) {
        match socket.recv(&mut buffer) {
            Ok(received) => {
                if buf_ptr != 0 {
                    for (offset, byte) in buffer[..received].iter().enumerate() {
                        let _ = vm.write_u8(buf_ptr.wrapping_add(offset as u32), *byte);
                    }
                }
                set_last_error(0);
                return received as u32;
            }
            Err(err) => {
                if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) {
                    set_last_error(WSAEWOULDBLOCK);
                    return 0;
                }
            }
        }
    } else {
        set_last_error(WSAENOTSOCK);
        return SOCKET_ERROR;
    }
    set_last_error(WSAEINVAL);
    SOCKET_ERROR
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
    let (handle, buf_ptr, len, _flags) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    if len == 0 {
        set_last_error(0);
        return 0;
    }
    if buf_ptr == 0 {
        set_last_error(WSAEINVAL);
        return SOCKET_ERROR;
    }
    let mut bytes = Vec::with_capacity(len as usize);
    for offset in 0..len {
        bytes.push(vm.read_u8(buf_ptr.wrapping_add(offset)).unwrap_or(0));
    }
    if let Some(socket) = socket_by_handle(handle) {
        if let Ok(count) = socket.send(&bytes) {
            log_send(&bytes);
            set_last_error(0);
            return count as u32;
        }
    } else {
        set_last_error(WSAENOTSOCK);
        return SOCKET_ERROR;
    }
    set_last_error(WSAEINVAL);
    SOCKET_ERROR
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
    let handle = alloc_socket();
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            let _ = socket.set_read_timeout(Some(Duration::from_secs(2)));
            register_socket(handle, socket);
            set_last_error(0);
            handle
        }
        Err(_) => {
            set_last_error(WSAEINVAL);
            SOCKET_ERROR
        }
    }
}

pub(super) fn ioctlsocket(vm: &mut Vm, stack_ptr: u32) -> u32 {
    const FIONBIO: u32 = 0x8004_667E;
    const FIONREAD: u32 = 0x4004_667F;

    let (handle, cmd, argp) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if argp == 0 {
        set_last_error(WSAEINVAL);
        return SOCKET_ERROR;
    }
    let Some(socket) = socket_by_handle(handle) else {
        set_last_error(WSAENOTSOCK);
        return SOCKET_ERROR;
    };

    match cmd {
        FIONBIO => {
            let nonblocking = vm.read_u32(argp).unwrap_or(0) != 0;
            match socket.set_nonblocking(nonblocking) {
                Ok(()) => {
                    set_last_error(0);
                    0
                }
                Err(_) => {
                    set_last_error(WSAEINVAL);
                    SOCKET_ERROR
                }
            }
        }
        FIONREAD => {
            let mut buffer = [0u8; 512];
            match socket.peek(&mut buffer) {
                Ok(count) => {
                    let _ = vm.write_u32(argp, count as u32);
                    set_last_error(0);
                    0
                }
                Err(err) if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                    let _ = vm.write_u32(argp, 0);
                    set_last_error(0);
                    0
                }
                Err(_) => {
                    set_last_error(WSAEINVAL);
                    SOCKET_ERROR
                }
            }
        }
        _ => {
            set_last_error(WSAEINVAL);
            SOCKET_ERROR
        }
    }
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
    // Use sandbox fallback host if configured
    let target_name = network_fallback_host(vm).unwrap_or(&name);
    let addr = parse_ipv4(target_name)
        .or_else(|| resolve_host(target_name))
        .unwrap_or(default_addr_for_host(target_name));
    let ptr = alloc_hostent(vm, name.as_str(), addr);
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
    let name = format_host_by_addr(addr);
    let ptr = alloc_hostent(vm, name.as_str(), addr);
    set_last_error(0);
    ptr
}

fn alloc_hostent(vm: &mut Vm, name: &str, addr: u32) -> u32 {
    let name_ptr = alloc_c_string(vm, name);
    let aliases_ptr = alloc_u32_list(vm, &[0]);
    let addr_bytes = addr.to_le_bytes();
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

fn default_addr_for_host(name: &str) -> u32 {
    if name.eq_ignore_ascii_case("localhost") {
        u32::from_le_bytes([127, 0, 0, 1])
    } else {
        u32::from_le_bytes([192, 0, 2, 1])
    }
}

fn resolve_host(name: &str) -> Option<u32> {
    let addrs = (name, 0).to_socket_addrs().ok()?;
    for addr in addrs {
        if let SocketAddr::V4(v4) = addr {
            return Some(u32::from_le_bytes(v4.ip().octets()));
        }
    }
    None
}

fn format_host_by_addr(addr: u32) -> String {
    let bytes = addr.to_le_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
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
