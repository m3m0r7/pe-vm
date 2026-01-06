//! Winsock constants.

pub(super) const INVALID_SOCKET: u32 = 0xFFFF_FFFF;
pub(super) const SOCKET_ERROR: u32 = 0xFFFF_FFFF;

pub(super) const WSAEINVAL: u32 = 10022;
pub(super) const WSAENOTSOCK: u32 = 10038;

pub(super) const WSADATA_SIZE: usize = 400;
pub(super) const WSADATA_VERSION: u16 = 0x0202;

pub(super) const SOCKET_HANDLE_BASE: u32 = 0x4000_0000;
pub(super) const EVENT_HANDLE_BASE: u32 = 0x5000_0000;
pub(super) const WSANETWORKEVENTS_SIZE: usize = 44;
