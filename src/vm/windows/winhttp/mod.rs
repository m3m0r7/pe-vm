//! WinHTTP stubs with a minimal HTTP client implementation.

pub const DLL_NAME: &str = "WINHTTP.dll";

mod http;
mod store;
mod types;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpOpen",
        crate::vm::stdcall_args(5),
        http::winhttp_open,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpConnect",
        crate::vm::stdcall_args(4),
        http::winhttp_connect,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpOpenRequest",
        crate::vm::stdcall_args(7),
        http::winhttp_open_request,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpAddRequestHeaders",
        crate::vm::stdcall_args(4),
        http::winhttp_add_request_headers,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpSendRequest",
        crate::vm::stdcall_args(7),
        http::winhttp_send_request,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpReceiveResponse",
        crate::vm::stdcall_args(2),
        http::winhttp_receive_response,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpQueryDataAvailable",
        crate::vm::stdcall_args(2),
        http::winhttp_query_data_available,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpReadData",
        crate::vm::stdcall_args(4),
        http::winhttp_read_data,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "WinHttpCloseHandle",
        crate::vm::stdcall_args(1),
        http::winhttp_close_handle,
    );
}
