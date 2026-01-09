//! Kernel32 process-related stubs.

use crate::define_stub_fn;
use crate::{read_str_arg, read_wstr_arg};
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

define_stub_fn!(DLL_NAME, is_debugger_present, 0);
define_stub_fn!(DLL_NAME, exit_process, 0);
define_stub_fn!(DLL_NAME, create_process_a, 0);

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
        "GetCurrentThreadId",
        crate::vm::stdcall_args(0),
        get_current_thread_id,
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
        "OutputDebugStringA",
        crate::vm::stdcall_args(1),
        output_debug_string_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetHandleCount",
        crate::vm::stdcall_args(1),
        set_handle_count,
    );
}

fn get_current_process_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Returns a fixed process ID for the emulated process.
    1
}

fn get_current_thread_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Returns a fixed thread ID for the main thread.
    1
}

fn output_debug_string_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr != 0 && std::env::var("PE_VM_TRACE_DEBUGSTR").is_ok() {
        let text = read_wstr_arg!(vm, ptr);
        if !text.is_empty() {
            eprintln!("[pe_vm] OutputDebugStringW: {text}");
        }
    }
    0
}

fn output_debug_string_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr != 0 && std::env::var("PE_VM_TRACE_DEBUGSTR").is_ok() {
        let text = read_str_arg!(vm, ptr);
        if !text.is_empty() {
            eprintln!("[pe_vm] OutputDebugStringA: {text}");
        }
    }
    0
}

// Processor feature constants from winnt.h
const PF_FLOATING_POINT_PRECISION_ERRATA: u32 = 0;
const PF_FLOATING_POINT_EMULATED: u32 = 1;
const PF_COMPARE_EXCHANGE_DOUBLE: u32 = 2;
const PF_MMX_INSTRUCTIONS_AVAILABLE: u32 = 3;
const PF_XMMI_INSTRUCTIONS_AVAILABLE: u32 = 6; // SSE
const PF_3DNOW_INSTRUCTIONS_AVAILABLE: u32 = 7;
const PF_RDTSC_INSTRUCTION_AVAILABLE: u32 = 8;
const PF_PAE_ENABLED: u32 = 9;
const PF_XMMI64_INSTRUCTIONS_AVAILABLE: u32 = 10; // SSE2
const PF_NX_ENABLED: u32 = 12;
const PF_SSE3_INSTRUCTIONS_AVAILABLE: u32 = 13;
const PF_COMPARE_EXCHANGE128: u32 = 14;
const PF_FASTFAIL_AVAILABLE: u32 = 23;

/// `IsProcessorFeaturePresent` checks if a processor feature is available.
///
/// Signature: BOOL WINAPI IsProcessorFeaturePresent(DWORD ProcessorFeature)
///
/// We emulate an x86 processor with common features enabled.
fn is_processor_feature_present(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (feature,) = vm_args!(vm, stack_ptr; u32);

    let result = match feature {
        PF_FLOATING_POINT_PRECISION_ERRATA => 0, // No Pentium FP bug
        PF_FLOATING_POINT_EMULATED => 0,         // Hardware FP available
        PF_COMPARE_EXCHANGE_DOUBLE => 0,         // CMPXCHG8B not emulated
        PF_MMX_INSTRUCTIONS_AVAILABLE => 0,      // MMX not emulated
        PF_XMMI_INSTRUCTIONS_AVAILABLE => 0,     // SSE not fully emulated
        PF_3DNOW_INSTRUCTIONS_AVAILABLE => 0,    // No 3DNow! (AMD only)
        PF_RDTSC_INSTRUCTION_AVAILABLE => 0,     // RDTSC not emulated
        PF_PAE_ENABLED => 0,                     // PAE not enabled in our VM
        PF_XMMI64_INSTRUCTIONS_AVAILABLE => 0,   // SSE2 not emulated
        PF_NX_ENABLED => 0,                      // NX/DEP not enforced in our VM
        PF_SSE3_INSTRUCTIONS_AVAILABLE => 0,     // SSE3 not emulated
        PF_COMPARE_EXCHANGE128 => 0,             // Not on 32-bit
        PF_FASTFAIL_AVAILABLE => 0,              // __fastfail not emulated
        _ => 0,                                  // Unknown feature
    };

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] IsProcessorFeaturePresent(feature={}) -> {}",
            feature, result
        );
    }

    result
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
