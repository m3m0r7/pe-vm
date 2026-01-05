//! WTSAPI32 stubs.

use crate::vm::Vm;

const WTS_SUCCESS: u32 = 1;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "WTSAPI32.dll",
        "WTSOpenServerA",
        crate::vm::stdcall_args(1),
        wts_open_server_a,
    );
    vm.register_import_stdcall(
        "WTSAPI32.dll",
        "WTSEnumerateSessionsA",
        crate::vm::stdcall_args(5),
        wts_enumerate_sessions_a,
    );
    vm.register_import_stdcall(
        "WTSAPI32.dll",
        "WTSCloseServer",
        crate::vm::stdcall_args(1),
        wts_close_server,
    );
    vm.register_import_stdcall(
        "WTSAPI32.dll",
        "WTSFreeMemory",
        crate::vm::stdcall_args(1),
        wts_free_memory,
    );
}

// HANDLE WTSOpenServerA(LPSTR pServerName)
fn wts_open_server_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

// BOOL WTSEnumerateSessionsA(HANDLE, DWORD, DWORD, PWTS_SESSION_INFO*, DWORD*)
fn wts_enumerate_sessions_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let sessions_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let count_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if sessions_ptr != 0 {
        let _ = vm.write_u32(sessions_ptr, 0);
    }
    if count_ptr != 0 {
        let _ = vm.write_u32(count_ptr, 0);
    }
    WTS_SUCCESS
}

// VOID WTSCloseServer(HANDLE)
fn wts_close_server(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// VOID WTSFreeMemory(PVOID)
fn wts_free_memory(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
