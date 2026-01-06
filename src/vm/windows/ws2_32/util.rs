//! Winsock utilities.

use crate::vm::Vm;

pub(super) fn read_fd_count(vm: &Vm, set_ptr: u32) -> u32 {
    if set_ptr == 0 {
        return 0;
    }
    vm.read_u32(set_ptr).unwrap_or(0)
}

pub(super) fn parse_ipv4(text: &str) -> Option<u32> {
    let mut octets = [0u8; 4];
    let mut parts = text.split('.');
    for slot in &mut octets {
        let part = parts.next()?;
        let value: u8 = part.parse().ok()?;
        *slot = value;
    }
    if parts.next().is_some() {
        return None;
    }
    Some(u32::from_be_bytes(octets))
}

pub(super) fn read_sockaddr_in(vm: &Vm, ptr: u32) -> Option<(String, u16)> {
    if ptr == 0 {
        return None;
    }
    let family = vm.read_u16(ptr).ok()?;
    if family != 2 {
        return None;
    }
    let port = vm.read_u16(ptr + 2).ok()?;
    let addr = vm.read_u32(ptr + 4).ok()?;
    let octets = addr.to_be_bytes();
    let host = format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]);
    Some((host, port))
}
