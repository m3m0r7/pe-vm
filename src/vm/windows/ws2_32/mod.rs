//! WS2_32 Winsock stubs.

mod constants;
mod events;
mod socket;
mod store;
mod trace;
mod util;

use crate::vm::Vm;

// Registers Winsock exports used by imported ordinals and common entry points.
pub fn register(vm: &mut Vm) {
    vm.register_import_ordinal_stdcall("WS2_32.dll", 2, crate::vm::stdcall_args(3), socket::bind);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 3, crate::vm::stdcall_args(1), socket::closesocket);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 4, crate::vm::stdcall_args(3), socket::connect);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 9, crate::vm::stdcall_args(1), socket::htons);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 11, crate::vm::stdcall_args(1), socket::inet_addr);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 13, crate::vm::stdcall_args(2), socket::listen);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 16, crate::vm::stdcall_args(4), socket::recv);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 18, crate::vm::stdcall_args(5), socket::select);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 19, crate::vm::stdcall_args(4), socket::send);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 21, crate::vm::stdcall_args(5), socket::setsockopt);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 22, crate::vm::stdcall_args(2), socket::shutdown);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 23, crate::vm::stdcall_args(3), socket::socket);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 111, crate::vm::stdcall_args(0), store::wsa_get_last_error);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 112, crate::vm::stdcall_args(1), store::wsa_set_last_error);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 115, crate::vm::stdcall_args(2), socket::wsa_startup);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 116, crate::vm::stdcall_args(0), socket::wsa_cleanup);
    vm.register_import_ordinal_stdcall("WS2_32.dll", 151, crate::vm::stdcall_args(2), events::wsafd_is_set);

    vm.register_import_stdcall("WS2_32.dll", "WSAStartup", crate::vm::stdcall_args(2), socket::wsa_startup);
    vm.register_import_stdcall("WS2_32.dll", "WSACleanup", crate::vm::stdcall_args(0), socket::wsa_cleanup);
    vm.register_import_stdcall("WS2_32.dll", "WSAGetLastError", crate::vm::stdcall_args(0), store::wsa_get_last_error);
    vm.register_import_stdcall("WS2_32.dll", "WSASetLastError", crate::vm::stdcall_args(1), store::wsa_set_last_error);
    vm.register_import_stdcall("WS2_32.dll", "WSACreateEvent", crate::vm::stdcall_args(0), events::wsa_create_event);
    vm.register_import_stdcall("WS2_32.dll", "WSACloseEvent", crate::vm::stdcall_args(1), events::wsa_close_event);
    vm.register_import_stdcall("WS2_32.dll", "WSAEventSelect", crate::vm::stdcall_args(3), events::wsa_event_select);
    vm.register_import_stdcall(
        "WS2_32.dll",
        "WSAEnumNetworkEvents",
        crate::vm::stdcall_args(3),
        events::wsa_enum_network_events,
    );
    vm.register_import_stdcall(
        "WS2_32.dll",
        "WSAWaitForMultipleEvents",
        crate::vm::stdcall_args(5),
        events::wsa_wait_for_multiple_events,
    );
}
