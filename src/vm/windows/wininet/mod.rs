//! WinINet stubs with a minimal HTTP client implementation.

pub const DLL_NAME: &str = "WININET.dll";

mod client;
mod http;
mod store;
mod trace;
mod types;
mod utils;

use crate::vm::Vm;

// Register minimal WinINet entry points used by common DLLs.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "InternetOpenA",
        crate::vm::stdcall_args(5),
        http::internet_open_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InternetConnectA",
        crate::vm::stdcall_args(8),
        http::internet_connect_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "HttpOpenRequestA",
        crate::vm::stdcall_args(8),
        http::http_open_request_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "HttpSendRequestA",
        crate::vm::stdcall_args(5),
        http::http_send_request_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "HttpQueryInfoA",
        crate::vm::stdcall_args(5),
        http::http_query_info_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InternetReadFile",
        crate::vm::stdcall_args(4),
        http::internet_read_file,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InternetSetOptionA",
        crate::vm::stdcall_args(4),
        http::internet_set_option_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InternetCloseHandle",
        crate::vm::stdcall_args(1),
        http::internet_close_handle,
    );
}
