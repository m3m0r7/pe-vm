//! Winsock handle store and error state.

use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, OnceLock};

use crate::vm::Vm;
use crate::vm_args;

use super::constants::{EVENT_HANDLE_BASE, SOCKET_HANDLE_BASE};

#[derive(Default)]
struct SocketStore {
    next_socket: u32,
    sockets: HashMap<u32, SocketInfo>,
    next_event: u32,
    last_error: u32,
}

#[derive(Clone)]
pub(super) enum SocketState {
    Udp(Arc<UdpSocket>),
    TcpStream(Arc<TcpStream>),
    TcpListener(Arc<TcpListener>),
    PendingTcp { bound: Option<SocketAddr> },
}

pub(super) struct SocketInfo {
    pub(super) state: SocketState,
}

fn store() -> &'static Mutex<SocketStore> {
    static STORE: OnceLock<Mutex<SocketStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(SocketStore {
            next_socket: SOCKET_HANDLE_BASE,
            next_event: EVENT_HANDLE_BASE,
            ..SocketStore::default()
        })
    })
}

pub(super) fn alloc_socket() -> u32 {
    let mut guard = store().lock().expect("ws2_32 store");
    if guard.next_socket == 0 {
        guard.next_socket = SOCKET_HANDLE_BASE;
    }
    let handle = guard.next_socket;
    guard.next_socket = guard.next_socket.wrapping_add(1);
    handle
}

pub(super) fn close_socket(handle: u32) -> bool {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.sockets.remove(&handle).is_some()
}

pub(super) fn register_socket(handle: u32, state: SocketState) {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.sockets.insert(handle, SocketInfo { state });
}

pub(super) fn socket_state(handle: u32) -> Option<SocketState> {
    let guard = store().lock().expect("ws2_32 store");
    guard.sockets.get(&handle).map(|info| info.state.clone())
}

pub(super) fn with_socket_mut<F, R>(handle: u32, f: F) -> Option<R>
where
    F: FnOnce(&mut SocketInfo) -> R,
{
    let mut guard = store().lock().expect("ws2_32 store");
    let info = guard.sockets.get_mut(&handle)?;
    Some(f(info))
}

pub(super) fn alloc_event() -> u32 {
    let mut guard = store().lock().expect("ws2_32 store");
    if guard.next_event == 0 {
        guard.next_event = EVENT_HANDLE_BASE;
    }
    let handle = guard.next_event;
    guard.next_event = guard.next_event.wrapping_add(1);
    handle
}

pub(super) fn set_last_error(code: u32) {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.last_error = code;
}

pub(super) fn last_error() -> u32 {
    let guard = store().lock().expect("ws2_32 store");
    guard.last_error
}

pub(super) fn wsa_get_last_error(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    last_error()
}

pub(super) fn wsa_set_last_error(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (code,) = vm_args!(vm, stack_ptr; u32);
    set_last_error(code);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};
    use std::net::UdpSocket;

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

    fn register_dummy_socket(handle: u32) {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind socket");
        register_socket(handle, SocketState::Udp(std::sync::Arc::new(socket)));
    }

    #[test]
    fn test_alloc_socket_returns_handle() {
        let handle = alloc_socket();
        assert!(handle >= SOCKET_HANDLE_BASE);
    }

    #[test]
    fn test_alloc_socket_increments() {
        let h1 = alloc_socket();
        let h2 = alloc_socket();
        assert_eq!(h2, h1.wrapping_add(1));
    }

    #[test]
    fn test_close_socket_success() {
        let handle = alloc_socket();
        register_dummy_socket(handle);
        let closed = close_socket(handle);
        assert!(closed);
    }

    #[test]
    fn test_close_socket_nonexistent() {
        let closed = close_socket(0xDEAD_BEEF);
        assert!(!closed);
    }

    #[test]
    fn test_alloc_event_returns_handle() {
        let handle = alloc_event();
        assert!(handle >= EVENT_HANDLE_BASE);
    }

    #[test]
    fn test_set_and_get_last_error() {
        set_last_error(12345);
        assert_eq!(last_error(), 12345);
    }

    #[test]
    fn test_wsa_get_last_error() {
        let mut vm = create_test_vm();
        set_last_error(99);
        let result = wsa_get_last_error(&mut vm, 0);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_wsa_set_last_error() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 42).unwrap();
        wsa_set_last_error(&mut vm, stack);
        assert_eq!(last_error(), 42);
    }
}
