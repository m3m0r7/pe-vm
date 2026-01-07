//! WTSAPI32 stubs.

use crate::vm::Vm;
use crate::vm::windows::check_stub;

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
fn wts_open_server_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    check_stub(vm, "WTSAPI32.dll", "WTSOpenServerA");
    1
}

// BOOL WTSEnumerateSessionsA(HANDLE, DWORD, DWORD, PWTS_SESSION_INFO*, DWORD*)
fn wts_enumerate_sessions_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    check_stub(vm, "WTSAPI32.dll", "WTSEnumerateSessionsA");
    let sessions_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let count_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if sessions_ptr != 0 {
        let _ = vm.write_u32(sessions_ptr, 0);
    }
    if count_ptr != 0 {
        let _ = vm.write_u32(count_ptr, 0);
    }
    1 // WTS_SUCCESS
}

// VOID WTSCloseServer(HANDLE)
fn wts_close_server(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    check_stub(vm, "WTSAPI32.dll", "WTSCloseServer");
    0
}

// VOID WTSFreeMemory(PVOID)
fn wts_free_memory(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    check_stub(vm, "WTSAPI32.dll", "WTSFreeMemory");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BypassSettings;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut bypass = BypassSettings::new();
        bypass.not_implemented_module = true;
        let mut vm = Vm::new(
            VmConfig::new()
                .architecture(Architecture::X86)
                .bypass(bypass),
        )
        .expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_wts_open_server_a_returns_handle() {
        let mut vm = create_test_vm();
        let result = wts_open_server_a(&mut vm, 0);
        // Returns a non-null handle
        assert_eq!(result, 1);
    }

    #[test]
    fn test_wts_enumerate_sessions_a_success() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 24;
        let sessions_ptr = vm.heap_start as u32;
        let count_ptr = sessions_ptr + 4;
        vm.write_u32(sessions_ptr, 0xDEADBEEF).unwrap();
        vm.write_u32(count_ptr, 0xDEADBEEF).unwrap();
        vm.write_u32(stack + 16, sessions_ptr).unwrap();
        vm.write_u32(stack + 20, count_ptr).unwrap();
        let result = wts_enumerate_sessions_a(&mut vm, stack);
        assert_eq!(result, 1); // WTS_SUCCESS
        assert_eq!(vm.read_u32(sessions_ptr).unwrap(), 0);
        assert_eq!(vm.read_u32(count_ptr).unwrap(), 0);
    }

    #[test]
    fn test_wts_close_server_returns_zero() {
        let mut vm = create_test_vm();
        let result = wts_close_server(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_wts_free_memory_returns_zero() {
        let mut vm = create_test_vm();
        let result = wts_free_memory(&mut vm, 0);
        assert_eq!(result, 0);
    }
}
