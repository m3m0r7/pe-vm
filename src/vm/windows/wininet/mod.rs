//! WinINet stubs with a minimal HTTP client implementation.

mod client;
mod http;
mod store;
mod trace;
mod types;
mod utils;

use crate::vm::Vm;

// Register the minimal WinINet entry points used by JVLink.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("WININET.dll", "InternetOpenA", crate::vm::stdcall_args(5), http::internet_open_a);
    vm.register_import_stdcall("WININET.dll", "InternetConnectA", crate::vm::stdcall_args(8), http::internet_connect_a);
    vm.register_import_stdcall("WININET.dll", "HttpOpenRequestA", crate::vm::stdcall_args(8), http::http_open_request_a);
    vm.register_import_stdcall("WININET.dll", "HttpSendRequestA", crate::vm::stdcall_args(5), http::http_send_request_a);
    vm.register_import_stdcall("WININET.dll", "HttpQueryInfoA", crate::vm::stdcall_args(5), http::http_query_info_a);
    vm.register_import_stdcall("WININET.dll", "InternetReadFile", crate::vm::stdcall_args(4), http::internet_read_file);
    vm.register_import_stdcall("WININET.dll", "InternetSetOptionA", crate::vm::stdcall_args(4), http::internet_set_option_a);
    vm.register_import_stdcall("WININET.dll", "InternetCloseHandle", crate::vm::stdcall_args(1), http::internet_close_handle);
}
