//! WS2_32 Winsock stubs.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::vm::Vm;

const INVALID_SOCKET: u32 = 0xFFFF_FFFF;
const SOCKET_ERROR: u32 = 0xFFFF_FFFF;

const WSAEINVAL: u32 = 10022;
const WSAENOTSOCK: u32 = 10038;

const WSADATA_SIZE: usize = 400;
const WSADATA_VERSION: u16 = 0x0202;

const SOCKET_HANDLE_BASE: u32 = 0x4000_0000;
const EVENT_HANDLE_BASE: u32 = 0x5000_0000;
const WSANETWORKEVENTS_SIZE: usize = 44;

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

// Registers Winsock exports used by imported ordinals and common entry points.
pub fn register(vm: &mut Vm) {
    vm.register_import_ordinal_stdcall("WS2_32.dll", 2, crate::vm::stdcall_args(3), bind);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 3, crate::vm::stdcall_args(1), closesocket);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 4, crate::vm::stdcall_args(3), connect);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 9, crate::vm::stdcall_args(1), htons);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 11, crate::vm::stdcall_args(1), inet_addr);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 13, crate::vm::stdcall_args(2), listen);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 16, crate::vm::stdcall_args(4), recv);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 18, crate::vm::stdcall_args(5), select);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 19, crate::vm::stdcall_args(4), send);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 21, crate::vm::stdcall_args(5), setsockopt);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 22, crate::vm::stdcall_args(2), shutdown);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 23, crate::vm::stdcall_args(3), socket);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 111, crate::vm::stdcall_args(0), wsa_get_last_error);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 112, crate::vm::stdcall_args(1), wsa_set_last_error);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 115, crate::vm::stdcall_args(2), wsa_startup);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 116, crate::vm::stdcall_args(0), wsa_cleanup);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 151, crate::vm::stdcall_args(2), wsafd_is_set);

    vm.register_import_stdcall("WS2_32.dll", "WSAStartup", crate::vm::stdcall_args(2), wsa_startup);
    vm.register_import_stdcall("WS2_32.dll", "WSACleanup", crate::vm::stdcall_args(0), wsa_cleanup);
    vm.register_import_stdcall("WS2_32.dll", "WSAGetLastError", crate::vm::stdcall_args(0), wsa_get_last_error);
    vm.register_import_stdcall("WS2_32.dll", "WSASetLastError", crate::vm::stdcall_args(1), wsa_set_last_error);
    vm.register_import_stdcall("WS2_32.dll", "WSACreateEvent", crate::vm::stdcall_args(0), wsa_create_event);
    vm.register_import_stdcall("WS2_32.dll", "WSACloseEvent", crate::vm::stdcall_args(1), wsa_close_event);
    vm.register_import_stdcall("WS2_32.dll", "WSAEventSelect", crate::vm::stdcall_args(3), wsa_event_select);
    vm.register_import_stdcall(
        "WS2_32.dll",
        "WSAEnumNetworkEvents",
        crate::vm::stdcall_args(3),
        wsa_enum_network_events,
    );
    vm.register_import_stdcall(
        "WS2_32.dll",
        "WSAWaitForMultipleEvents",
        crate::vm::stdcall_args(5),
        wsa_wait_for_multiple_events,
    );
}

// Socket handle helpers shared by stubbed Winsock calls.
fn alloc_socket() -> u32 {
    let mut guard = store().lock().expect("ws2_32 store");
    if guard.next_socket == 0 {
        guard.next_socket = SOCKET_HANDLE_BASE;
    }
    let handle = guard.next_socket;
    guard.next_socket = guard.next_socket.wrapping_add(1);
    guard.open_sockets.insert(handle);
    handle
}

fn close_socket(handle: u32) -> bool {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.open_sockets.remove(&handle)
}

fn alloc_event() -> u32 {
    let mut guard = store().lock().expect("ws2_32 store");
    if guard.next_event == 0 {
        guard.next_event = EVENT_HANDLE_BASE;
    }
    let handle = guard.next_event;
    guard.next_event = guard.next_event.wrapping_add(1);
    handle
}

fn set_last_error(code: u32) {
    let mut guard = store().lock().expect("ws2_32 store");
    guard.last_error = code;
}

fn last_error() -> u32 {
    let guard = store().lock().expect("ws2_32 store");
    guard.last_error
}

fn bind(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn closesocket(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
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

fn connect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn htons(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let value = vm.read_u32(stack_ptr + 4).unwrap_or(0) as u16;
    let swapped = value.to_be();
    swapped as u32
}

fn inet_addr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return INVALID_SOCKET;
    }
    let text = vm.read_c_string(ptr).unwrap_or_default();
    match parse_ipv4(&text) {
        Some(addr) => addr,
        None => INVALID_SOCKET,
    }
}

fn listen(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn recv(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buf = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let len = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if buf != 0 && len != 0 {
        let _ = vm.memset(buf, 0, len as usize);
    }
    set_last_error(0);
    len
}

fn select(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let read_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let write_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let except_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);

    let read_count = read_fd_count(vm, read_ptr);
    let write_count = read_fd_count(vm, write_ptr);
    let except_count = read_fd_count(vm, except_ptr);

    set_last_error(0);
    read_count.saturating_add(write_count).saturating_add(except_count)
}

fn send(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let len = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    set_last_error(0);
    len
}

fn setsockopt(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn shutdown(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn socket(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    alloc_socket()
}

fn wsa_get_last_error(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    last_error()
}

fn wsa_set_last_error(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let code = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    set_last_error(code);
    0
}

fn wsa_startup(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let version = vm.read_u32(stack_ptr + 4).unwrap_or(0) as u16;
    let data_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
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

fn wsa_cleanup(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn wsa_create_event(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    alloc_event()
}

fn wsa_close_event(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if handle == 0 {
        set_last_error(WSAEINVAL);
        return 0;
    }
    set_last_error(0);
    1
}

fn wsa_event_select(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn wsa_enum_network_events(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let events_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if events_ptr != 0 {
        let _ = vm.memset(events_ptr, 0, WSANETWORKEVENTS_SIZE);
    }
    set_last_error(0);
    0
}

fn wsa_wait_for_multiple_events(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    set_last_error(0);
    0
}

fn wsafd_is_set(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let set_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
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

fn read_fd_count(vm: &Vm, set_ptr: u32) -> u32 {
    if set_ptr == 0 {
        return 0;
    }
    vm.read_u32(set_ptr).unwrap_or(0)
}

fn parse_ipv4(text: &str) -> Option<u32> {
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
