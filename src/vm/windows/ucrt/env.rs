//! UCRT environment stubs.

use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_initialize_narrow_environment",
        initialize_narrow_environment,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_seh_filter_dll",
        seh_filter_dll,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_seh_filter_exe",
        seh_filter_exe,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_configure_narrow_argv",
        configure_narrow_argv,
    );
}

/// `_configure_narrow_argv` configures how command-line arguments are parsed.
///
/// Signature: int __cdecl _configure_narrow_argv(int mode)
///
/// mode values:
/// - 0: _crt_argv_no_arguments - No arguments
/// - 1: _crt_argv_unexpanded_arguments - Don't expand wildcards
/// - 2: _crt_argv_expanded_arguments - Expand wildcards
///
/// Returns 0 on success, -1 on failure.
fn configure_narrow_argv(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (mode,) = vm_args!(vm, stack_ptr; u32);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] _configure_narrow_argv(mode={})", mode);
    }

    // Mode values 0, 1, 2 are valid
    if mode > 2 {
        return 0xFFFF_FFFF; // -1 as u32
    }

    // In our VM, we don't actually configure argv parsing differently.
    // Just acknowledge the call succeeded.
    0
}

/// `_initialize_narrow_environment` initializes the narrow character environment.
///
/// Signature: int __cdecl _initialize_narrow_environment(void)
///
/// This function initializes internal CRT state for environment variable access
/// via getenv() and related functions. Returns 0 on success.
fn initialize_narrow_environment(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] _initialize_narrow_environment()");
    }

    // Initialize environment if needed.
    // The VM's environment is already set up via VmConfig, so this is a no-op.
    // The actual environment access happens through getenv() stub.
    let _ = vm;
    0
}

// SEH filter return values
const EXCEPTION_EXECUTE_HANDLER: u32 = 1;
const EXCEPTION_CONTINUE_SEARCH: u32 = 0;
#[allow(dead_code)]
const EXCEPTION_CONTINUE_EXECUTION: u32 = 0xFFFF_FFFF; // -1

/// `_seh_filter_dll` is the default SEH filter for DLL exceptions.
///
/// Signature: int __cdecl _seh_filter_dll(
///     unsigned long exception_code,
///     struct _EXCEPTION_POINTERS* exception_pointers
/// )
///
/// Returns EXCEPTION_EXECUTE_HANDLER (1) for C++ exceptions,
/// EXCEPTION_CONTINUE_SEARCH (0) for other exceptions.
fn seh_filter_dll(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (exception_code, _exception_pointers) = vm_args!(vm, stack_ptr; u32, u32);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] _seh_filter_dll(exception_code=0x{:08X})",
            exception_code
        );
    }

    // C++ exception code (thrown by throw statements)
    const CPP_EXCEPTION_CODE: u32 = 0xE06D7363; // 'msc' | 0xE0000000

    if exception_code == CPP_EXCEPTION_CODE {
        // Let the handler deal with C++ exceptions
        EXCEPTION_EXECUTE_HANDLER
    } else {
        // Continue searching for other exceptions
        EXCEPTION_CONTINUE_SEARCH
    }
}

/// `_seh_filter_exe` is the default SEH filter for EXE exceptions.
///
/// Same signature and behavior as _seh_filter_dll.
fn seh_filter_exe(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (exception_code, _exception_pointers) = vm_args!(vm, stack_ptr; u32, u32);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] _seh_filter_exe(exception_code=0x{:08X})",
            exception_code
        );
    }

    const CPP_EXCEPTION_CODE: u32 = 0xE06D7363;

    if exception_code == CPP_EXCEPTION_CODE {
        EXCEPTION_EXECUTE_HANDLER
    } else {
        EXCEPTION_CONTINUE_SEARCH
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    fn setup_stack_args(vm: &mut Vm, args: &[u32]) -> u32 {
        // vm_args! reads from stack_ptr + 4 for first arg
        // Layout: [ret_addr][arg0][arg1]...
        // Make sure we're writing within memory bounds
        let stack_ptr = vm.base + 0x1000; // Use an address well within memory range
        let _ = vm.write_u32(stack_ptr, 0x12345678); // return address
        for (i, &arg) in args.iter().enumerate() {
            let _ = vm.write_u32(stack_ptr + 4 + (i as u32) * 4, arg);
        }
        stack_ptr
    }

    #[test]
    fn test_configure_narrow_argv_valid_modes() {
        let mut vm = create_test_vm();

        // mode=0
        let stack_ptr = setup_stack_args(&mut vm, &[0]);
        let result = configure_narrow_argv(&mut vm, stack_ptr);
        assert_eq!(result, 0);

        // mode=1
        let stack_ptr = setup_stack_args(&mut vm, &[1]);
        let result = configure_narrow_argv(&mut vm, stack_ptr);
        assert_eq!(result, 0);

        // mode=2
        let stack_ptr = setup_stack_args(&mut vm, &[2]);
        let result = configure_narrow_argv(&mut vm, stack_ptr);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_configure_narrow_argv_invalid_mode() {
        let mut vm = create_test_vm();
        let stack_ptr = setup_stack_args(&mut vm, &[99]); // invalid mode
        let result = configure_narrow_argv(&mut vm, stack_ptr);
        assert_eq!(result, 0xFFFF_FFFF); // -1
    }

    #[test]
    fn test_initialize_narrow_environment_returns_zero() {
        let mut vm = create_test_vm();
        let result = initialize_narrow_environment(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_seh_filter_dll_cpp_exception() {
        let mut vm = create_test_vm();
        // _seh_filter_dll(exception_code, exception_pointers)
        let stack_ptr = setup_stack_args(&mut vm, &[0xE06D7363, 0]); // C++ exception
        let result = seh_filter_dll(&mut vm, stack_ptr);
        assert_eq!(result, EXCEPTION_EXECUTE_HANDLER);
    }

    #[test]
    fn test_seh_filter_dll_other_exception() {
        let mut vm = create_test_vm();
        let stack_ptr = setup_stack_args(&mut vm, &[0xC0000005, 0]); // Access violation
        let result = seh_filter_dll(&mut vm, stack_ptr);
        assert_eq!(result, EXCEPTION_CONTINUE_SEARCH);
    }
}
