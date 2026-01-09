//! WS2_32 Winsock stubs.

pub const DLL_NAME: &str = "WS2_32.dll";
const WSOCK32_NAME: &str = "WSOCK32.dll";

#[repr(u16)]
#[derive(Clone, Copy)]
enum Ws2Ord {
    Accept = 1,
    Bind = 2,
    CloseSocket = 3,
    Connect = 4,
    Htonl = 8,
    Htons = 9,
    IoctlSocket = 10,
    InetAddr = 11,
    Listen = 13,
    Recv = 16,
    Select = 18,
    Send = 19,
    SetSockOpt = 21,
    Shutdown = 22,
    Socket = 23,
    GetHostByAddr = 51,
    GetHostByName = 52,
    WsaGetLastError = 111,
    WsaSetLastError = 112,
    WsaStartup = 115,
    WsaCleanup = 116,
    WsAFdIsSet = 151,
}

#[repr(u16)]
#[derive(Clone, Copy)]
enum Wsock32Ord {
    Accept = 1,
    Bind = 2,
    CloseSocket = 3,
    Connect = 4,
    Htonl = 8,
    Htons = 9,
    InetAddr = 10,
    InetNtoa = 11,
    IoctlSocket = 12,
    Listen = 13,
    Recv = 16,
    Select = 18,
    Send = 19,
    SetSockOpt = 21,
    Shutdown = 22,
    Socket = 23,
    GetHostByAddr = 51,
    GetHostByName = 52,
    WsaGetLastError = 111,
    WsaSetLastError = 112,
    WsaStartup = 115,
    WsaCleanup = 116,
    WsAFdIsSet = 151,
}

mod constants;
mod events;
mod socket;
mod store;
mod trace;
mod util;

use crate::vm::Vm;

// Registers Winsock exports used by imported ordinals and common entry points.
pub fn register(vm: &mut Vm) {
    register_ws2_32(vm);
    register_wsock32(vm);

    vm.register_import_stdcall(
        DLL_NAME,
        "WSAStartup",
        crate::vm::stdcall_args(2),
        socket::wsa_startup,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSACleanup",
        crate::vm::stdcall_args(0),
        socket::wsa_cleanup,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSAGetLastError",
        crate::vm::stdcall_args(0),
        store::wsa_get_last_error,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSASetLastError",
        crate::vm::stdcall_args(1),
        store::wsa_set_last_error,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSACreateEvent",
        crate::vm::stdcall_args(0),
        events::wsa_create_event,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSACloseEvent",
        crate::vm::stdcall_args(1),
        events::wsa_close_event,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSAEventSelect",
        crate::vm::stdcall_args(3),
        events::wsa_event_select,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSAEnumNetworkEvents",
        crate::vm::stdcall_args(3),
        events::wsa_enum_network_events,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WSAWaitForMultipleEvents",
        crate::vm::stdcall_args(5),
        events::wsa_wait_for_multiple_events,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ioctlsocket",
        crate::vm::stdcall_args(3),
        socket::ioctlsocket,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "gethostbyaddr",
        crate::vm::stdcall_args(3),
        socket::gethostbyaddr,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "gethostbyname",
        crate::vm::stdcall_args(1),
        socket::gethostbyname,
    );
}

fn register_ws2_32(vm: &mut Vm) {
    let dll = DLL_NAME;
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Accept as u16,
        crate::vm::stdcall_args(3),
        socket::accept,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Bind as u16,
        crate::vm::stdcall_args(3),
        socket::bind,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::CloseSocket as u16,
        crate::vm::stdcall_args(1),
        socket::closesocket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Connect as u16,
        crate::vm::stdcall_args(3),
        socket::connect,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Htonl as u16,
        crate::vm::stdcall_args(1),
        socket::htonl,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Htons as u16,
        crate::vm::stdcall_args(1),
        socket::htons,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::IoctlSocket as u16,
        crate::vm::stdcall_args(3),
        socket::ioctlsocket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::InetAddr as u16,
        crate::vm::stdcall_args(1),
        socket::inet_addr,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Listen as u16,
        crate::vm::stdcall_args(2),
        socket::listen,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Recv as u16,
        crate::vm::stdcall_args(4),
        socket::recv,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Select as u16,
        crate::vm::stdcall_args(5),
        socket::select,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Send as u16,
        crate::vm::stdcall_args(4),
        socket::send,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::SetSockOpt as u16,
        crate::vm::stdcall_args(5),
        socket::setsockopt,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Shutdown as u16,
        crate::vm::stdcall_args(2),
        socket::shutdown,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::Socket as u16,
        crate::vm::stdcall_args(3),
        socket::socket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::GetHostByAddr as u16,
        crate::vm::stdcall_args(3),
        socket::gethostbyaddr,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::GetHostByName as u16,
        crate::vm::stdcall_args(1),
        socket::gethostbyname,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::WsaGetLastError as u16,
        crate::vm::stdcall_args(0),
        store::wsa_get_last_error,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::WsaSetLastError as u16,
        crate::vm::stdcall_args(1),
        store::wsa_set_last_error,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::WsaStartup as u16,
        crate::vm::stdcall_args(2),
        socket::wsa_startup,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::WsaCleanup as u16,
        crate::vm::stdcall_args(0),
        socket::wsa_cleanup,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Ws2Ord::WsAFdIsSet as u16,
        crate::vm::stdcall_args(2),
        events::wsafd_is_set,
    );
}

fn register_wsock32(vm: &mut Vm) {
    let dll = WSOCK32_NAME;
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Accept as u16,
        crate::vm::stdcall_args(3),
        socket::accept,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Bind as u16,
        crate::vm::stdcall_args(3),
        socket::bind,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::CloseSocket as u16,
        crate::vm::stdcall_args(1),
        socket::closesocket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Connect as u16,
        crate::vm::stdcall_args(3),
        socket::connect,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Htonl as u16,
        crate::vm::stdcall_args(1),
        socket::htonl,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Htons as u16,
        crate::vm::stdcall_args(1),
        socket::htons,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::InetAddr as u16,
        crate::vm::stdcall_args(1),
        socket::inet_addr,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::InetNtoa as u16,
        crate::vm::stdcall_args(1),
        socket::inet_ntoa,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::IoctlSocket as u16,
        crate::vm::stdcall_args(3),
        socket::ioctlsocket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Listen as u16,
        crate::vm::stdcall_args(2),
        socket::listen,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Recv as u16,
        crate::vm::stdcall_args(4),
        socket::recv,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Select as u16,
        crate::vm::stdcall_args(5),
        socket::select,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Send as u16,
        crate::vm::stdcall_args(4),
        socket::send,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::SetSockOpt as u16,
        crate::vm::stdcall_args(5),
        socket::setsockopt,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Shutdown as u16,
        crate::vm::stdcall_args(2),
        socket::shutdown,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::Socket as u16,
        crate::vm::stdcall_args(3),
        socket::socket,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::GetHostByAddr as u16,
        crate::vm::stdcall_args(3),
        socket::gethostbyaddr,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::GetHostByName as u16,
        crate::vm::stdcall_args(1),
        socket::gethostbyname,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::WsaGetLastError as u16,
        crate::vm::stdcall_args(0),
        store::wsa_get_last_error,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::WsaSetLastError as u16,
        crate::vm::stdcall_args(1),
        store::wsa_set_last_error,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::WsaStartup as u16,
        crate::vm::stdcall_args(2),
        socket::wsa_startup,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::WsaCleanup as u16,
        crate::vm::stdcall_args(0),
        socket::wsa_cleanup,
    );
    vm.register_import_ordinal_stdcall(
        dll,
        Wsock32Ord::WsAFdIsSet as u16,
        crate::vm::stdcall_args(2),
        events::wsafd_is_set,
    );
}
