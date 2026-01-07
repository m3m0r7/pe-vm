//! Kernel32 process-related stubs.

use crate::define_stub_fn;
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

define_stub_fn!(DLL_NAME, is_debugger_present, 0);
define_stub_fn!(DLL_NAME, is_processor_feature_present, 0);
define_stub_fn!(DLL_NAME, exit_process, 0);
define_stub_fn!(DLL_NAME, create_process_a, 0);
define_stub_fn!(DLL_NAME, output_debug_string_w, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "IsDebuggerPresent",
        crate::vm::stdcall_args(0),
        is_debugger_present,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCurrentProcessId",
        crate::vm::stdcall_args(0),
        get_current_process_id,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetStartupInfoA",
        crate::vm::stdcall_args(1),
        get_startup_info_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetStartupInfoW",
        crate::vm::stdcall_args(1),
        get_startup_info_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "IsProcessorFeaturePresent",
        crate::vm::stdcall_args(1),
        is_processor_feature_present,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "TerminateProcess",
        crate::vm::stdcall_args(2),
        terminate_process,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCurrentProcess",
        crate::vm::stdcall_args(0),
        get_current_process,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetSystemInfo",
        crate::vm::stdcall_args(1),
        get_system_info,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ExitProcess",
        crate::vm::stdcall_args(1),
        exit_process,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateProcessA",
        crate::vm::stdcall_args(10),
        create_process_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "OutputDebugStringW",
        crate::vm::stdcall_args(1),
        output_debug_string_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetHandleCount",
        crate::vm::stdcall_args(1),
        set_handle_count,
    );
}

fn get_current_process_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_startup_info_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info,) = vm_args!(vm, stack_ptr; u32);
    if info != 0 {
        let _ = vm.write_bytes(info, &[0u8; 68]);
        let _ = vm.write_u32(info, 68);
    }
    0
}

fn get_startup_info_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info,) = vm_args!(vm, stack_ptr; u32);
    if info != 0 {
        let _ = vm.write_bytes(info, &[0u8; 68]);
        let _ = vm.write_u32(info, 68);
    }
    0
}

fn set_handle_count(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (count,) = vm_args!(vm, stack_ptr; u32);
    count
}

fn terminate_process(vm: &mut Vm, stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
        eprintln!("[pe_vm] TerminateProcess at eip=0x{:08X}", vm.eip());
    }
    let _ = vm.write_u32(stack_ptr, 0);
    1
}

fn get_current_process(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0xFFFF_FFFF
}

fn get_system_info(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_ptr,) = vm_args!(vm, stack_ptr; u32);
    if info_ptr == 0 {
        return 0;
    }
    let _ = vm.write_u16(info_ptr, 0);
    let _ = vm.write_u16(info_ptr + 2, 0);
    let _ = vm.write_u32(info_ptr + 4, 0x1000);
    let _ = vm.write_u32(info_ptr + 8, 0x0001_0000);
    let _ = vm.write_u32(info_ptr + 12, 0x7FFF_0000);
    let _ = vm.write_u32(info_ptr + 16, 1);
    let _ = vm.write_u32(info_ptr + 20, 1);
    let _ = vm.write_u32(info_ptr + 24, 0x0000_0586);
    let _ = vm.write_u32(info_ptr + 28, 0x0001_0000);
    let _ = vm.write_u16(info_ptr + 32, 6);
    let _ = vm.write_u16(info_ptr + 34, 0);
    0
}
