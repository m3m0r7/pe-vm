//! Winsock constants.

pub(super) const INVALID_SOCKET: u32 = 0xFFFF_FFFF;
pub(super) const SOCKET_ERROR: u32 = 0xFFFF_FFFF;
pub(super) const AF_INET: u16 = 2;

pub(super) const WSAEINVAL: u32 = 10022;
pub(super) const WSAEWOULDBLOCK: u32 = 10035;
pub(super) const WSAENOTSOCK: u32 = 10038;
#[cfg(test)]
pub(super) const WSAETIMEDOUT: u32 = 10060;

pub(super) const WSADATA_SIZE: usize = 400;
pub(super) const WSADATA_VERSION: u16 = 0x0202;

pub(super) const SOCKET_HANDLE_BASE: u32 = 0x4000_0000;
pub(super) const EVENT_HANDLE_BASE: u32 = 0x5000_0000;
pub(super) const WSANETWORKEVENTS_SIZE: usize = 44;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_socket_value() {
        assert_eq!(INVALID_SOCKET, 0xFFFF_FFFF);
    }

    #[test]
    fn test_socket_error_value() {
        assert_eq!(SOCKET_ERROR, 0xFFFF_FFFF);
    }

    #[test]
    fn test_wsa_error_codes() {
        assert_eq!(WSAEINVAL, 10022);
        assert_eq!(WSAEWOULDBLOCK, 10035);
        assert_eq!(WSAENOTSOCK, 10038);
        assert_eq!(WSAETIMEDOUT, 10060);
    }

    #[test]
    fn test_wsadata_constants() {
        assert_eq!(WSADATA_SIZE, 400);
        assert_eq!(WSADATA_VERSION, 0x0202);
    }

    #[test]
    fn test_handle_bases() {
        assert_eq!(SOCKET_HANDLE_BASE, 0x4000_0000);
        assert_eq!(EVENT_HANDLE_BASE, 0x5000_0000);
        // Bases should be different
        assert_ne!(SOCKET_HANDLE_BASE, EVENT_HANDLE_BASE);
    }

    #[test]
    fn test_network_events_size() {
        assert_eq!(WSANETWORKEVENTS_SIZE, 44);
    }
}
