//! Winsock handle store and error state.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::vm::Vm;

use super::constants::{EVENT_HANDLE_BASE, SOCKET_HANDLE_BASE};

#[derive(Default)]
struct SocketStore {
    next_socket: u32,
    open_sockets: HashSet<u32>,
    next_event: u32,
    last_error: u32,
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
    guard.open_sockets.insert(handle);
    handle
}

pub(super) fn close_socket(handle: u32) -> bool {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.open_sockets.remove(&handle)
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
    let code = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    set_last_error(code);
    0
}
