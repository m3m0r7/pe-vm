//! WS2_32 Winsock stubs.

pub const DLL_NAME: &str = "WS2_32.dll";

mod constants;
mod events;
mod socket;
mod store;
mod trace;
mod util;

use crate::vm::Vm;

// Registers Winsock exports used by imported ordinals and common entry points.
pub fn register(vm: &mut Vm) {
    vm.register_import_ordinal_stdcall(DLL_NAME, 2, crate::vm::stdcall_args(3), socket::bind);
    vm.register_import_ordinal_stdcall(DLL_NAME, 3, crate::vm::stdcall_args(1), socket::closesocket);
    vm.register_import_ordinal_stdcall(DLL_NAME, 4, crate::vm::stdcall_args(3), socket::connect);
    vm.register_import_ordinal_stdcall(DLL_NAME, 9, crate::vm::stdcall_args(1), socket::htons);
    vm.register_import_ordinal_stdcall(DLL_NAME, 10, crate::vm::stdcall_args(3), socket::ioctlsocket);
    vm.register_import_ordinal_stdcall(DLL_NAME, 11, crate::vm::stdcall_args(1), socket::inet_addr);
    vm.register_import_ordinal_stdcall(DLL_NAME, 13, crate::vm::stdcall_args(2), socket::listen);
    vm.register_import_ordinal_stdcall(DLL_NAME, 16, crate::vm::stdcall_args(4), socket::recv);
    vm.register_import_ordinal_stdcall(DLL_NAME, 18, crate::vm::stdcall_args(5), socket::select);
    vm.register_import_ordinal_stdcall(DLL_NAME, 19, crate::vm::stdcall_args(4), socket::send);
    vm.register_import_ordinal_stdcall(DLL_NAME, 21, crate::vm::stdcall_args(5), socket::setsockopt);
    vm.register_import_ordinal_stdcall(DLL_NAME, 22, crate::vm::stdcall_args(2), socket::shutdown);
    vm.register_import_ordinal_stdcall(DLL_NAME, 23, crate::vm::stdcall_args(3), socket::socket);
    vm.register_import_ordinal_stdcall(DLL_NAME, 51, crate::vm::stdcall_args(3), socket::gethostbyaddr);
    vm.register_import_ordinal_stdcall(DLL_NAME, 52, crate::vm::stdcall_args(1), socket::gethostbyname);
    vm.register_import_ordinal_stdcall(DLL_NAME, 111, crate::vm::stdcall_args(0), store::wsa_get_last_error);
    vm.register_import_ordinal_stdcall(DLL_NAME, 112, crate::vm::stdcall_args(1), store::wsa_set_last_error);
    vm.register_import_ordinal_stdcall(DLL_NAME, 115, crate::vm::stdcall_args(2), socket::wsa_startup);
    vm.register_import_ordinal_stdcall(DLL_NAME, 116, crate::vm::stdcall_args(0), socket::wsa_cleanup);
    vm.register_import_ordinal_stdcall(DLL_NAME, 151, crate::vm::stdcall_args(2), events::wsafd_is_set);

    vm.register_import_stdcall(DLL_NAME, "WSAStartup", crate::vm::stdcall_args(2), socket::wsa_startup);
    vm.register_import_stdcall(DLL_NAME, "WSACleanup", crate::vm::stdcall_args(0), socket::wsa_cleanup);
    vm.register_import_stdcall(DLL_NAME, "WSAGetLastError", crate::vm::stdcall_args(0), store::wsa_get_last_error);
    vm.register_import_stdcall(DLL_NAME, "WSASetLastError", crate::vm::stdcall_args(1), store::wsa_set_last_error);
    vm.register_import_stdcall(DLL_NAME, "WSACreateEvent", crate::vm::stdcall_args(0), events::wsa_create_event);
    vm.register_import_stdcall(DLL_NAME, "WSACloseEvent", crate::vm::stdcall_args(1), events::wsa_close_event);
    vm.register_import_stdcall(DLL_NAME, "WSAEventSelect", crate::vm::stdcall_args(3), events::wsa_event_select);
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
    vm.register_import_stdcall(DLL_NAME, "ioctlsocket", crate::vm::stdcall_args(3), socket::ioctlsocket);
    vm.register_import_stdcall(DLL_NAME, "gethostbyaddr", crate::vm::stdcall_args(3), socket::gethostbyaddr);
    vm.register_import_stdcall(DLL_NAME, "gethostbyname", crate::vm::stdcall_args(1), socket::gethostbyname);
}
