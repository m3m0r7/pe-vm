//! VCRUNTIME runtime stubs.

use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "VCRUNTIME140.dll",
        "__std_type_info_destroy_list",
        std_type_info_destroy_list,
    );
    vm.register_import(
        "VCRUNTIME140.dll",
        "_except_handler4_common",
        except_handler4_common,
    );
}

/// `__std_type_info_destroy_list` destroys a linked list of type_info nodes.
///
/// Signature: void __cdecl __std_type_info_destroy_list(__type_info_node* root)
///
/// The `__type_info_node` structure:
/// ```c
/// struct __type_info_node {
///     void* _MemPtr;           // +0: Pointer to allocated memory (type name string)
///     __type_info_node* _Next; // +4: Next node in list
/// };
/// ```
///
/// This function walks the linked list and frees each node's memory.
/// In our VM, we don't track these allocations separately, so we just
/// walk the list to properly consume the argument.
fn std_type_info_destroy_list(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (mut node_ptr,) = vm_args!(vm, stack_ptr; u32);

    // Walk the linked list and "free" each node
    // In a real implementation, this would call free() on _MemPtr
    while node_ptr != 0 {
        // Read _MemPtr at offset 0 (we don't need to free it in our VM)
        let _mem_ptr = vm.read_u32(node_ptr).unwrap_or(0);

        // Read _Next at offset 4
        let next_ptr = vm.read_u32(node_ptr.wrapping_add(4)).unwrap_or(0);

        // Move to next node
        node_ptr = next_ptr;
    }

    // void function, return value is ignored
    0
}

// Exception disposition values
#[allow(dead_code)]
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_CONTINUE_SEARCH: u32 = 0;
#[allow(dead_code)]
const EXCEPTION_EXECUTE_HANDLER: u32 = 1;

/// `_except_handler4_common` is the SEH exception handler for /GS protected code.
///
/// Signature:
/// ```c
/// EXCEPTION_DISPOSITION __cdecl _except_handler4_common(
///     DWORD* CookiePointer,           // +4:  Pointer to __security_cookie
///     void (*CookieCheckFunction)(DWORD), // +8:  __security_check_cookie
///     EXCEPTION_RECORD* ExceptionRecord,  // +12: Exception info
///     EXCEPTION_REGISTRATION_RECORD* EstablisherFrame, // +16
///     CONTEXT* ContextRecord,             // +20
///     void* DispatcherContext             // +24
/// );
/// ```
///
/// This is called when an exception occurs in /GS protected code.
/// For our VM, we provide a minimal implementation that continues searching
/// for handlers, as we don't fully emulate Windows SEH.
fn except_handler4_common(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (
        cookie_ptr,
        _cookie_check_fn,
        exception_record_ptr,
        _establisher_frame,
        _context_record,
        _dispatcher_context,
    ) = vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);

    // Read security cookie if available
    let _security_cookie = if cookie_ptr != 0 {
        vm.read_u32(cookie_ptr).unwrap_or(0)
    } else {
        0
    };

    // Read exception code from EXCEPTION_RECORD
    let exception_code = if exception_record_ptr != 0 {
        vm.read_u32(exception_record_ptr).unwrap_or(0)
    } else {
        0
    };

    // Read exception flags from EXCEPTION_RECORD + 4
    let exception_flags = if exception_record_ptr != 0 {
        vm.read_u32(exception_record_ptr.wrapping_add(4))
            .unwrap_or(0)
    } else {
        0
    };

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] _except_handler4_common: code=0x{:08X} flags=0x{:08X}",
            exception_code, exception_flags
        );
    }

    // Check if this is an unwinding operation (flag 0x2)
    const EXCEPTION_UNWINDING: u32 = 0x2;
    if (exception_flags & EXCEPTION_UNWINDING) != 0 {
        // During unwind, execute cleanup handlers
        // For now, just continue the search
        return EXCEPTION_CONTINUE_SEARCH;
    }

    // For non-unwinding exceptions, we continue searching for a handler
    // A full implementation would:
    // 1. Decode the scope table using XOR with security cookie
    // 2. Walk the scope table to find matching handlers
    // 3. Execute filter functions
    // 4. Transfer control to the handler if filter returns EXCEPTION_EXECUTE_HANDLER
    //
    // For our VM, we return EXCEPTION_CONTINUE_SEARCH to let the exception propagate
    EXCEPTION_CONTINUE_SEARCH
}
