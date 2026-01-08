use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

/// Counter for registered window messages (starts above WM_USER range)
static NEXT_REGISTERED_MSG: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0xC000);

/// ScreenToClient - Converts screen coordinates to client-area coordinates
/// Parameters:
///   - hWnd: Handle to the window
///   - lpPoint: Pointer to POINT structure with screen coordinates
/// Returns: TRUE if succeeded, FALSE if failed
fn screen_to_client(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, point_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    // In our VM, we don't track actual window positions
    // Just succeed without modifying the point (assumes 0,0 client offset)
    if hwnd != 0 && point_ptr != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// ClientToScreen - Converts client-area coordinates to screen coordinates
/// Parameters:
///   - hWnd: Handle to the window
///   - lpPoint: Pointer to POINT structure with client coordinates
/// Returns: TRUE if succeeded, FALSE if failed
fn client_to_screen(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, point_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    // In our VM, we don't track actual window positions
    // Just succeed without modifying the point (assumes 0,0 screen offset)
    if hwnd != 0 && point_ptr != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// SetWindowRgn - Sets the window region of a window
/// Parameters:
///   - hWnd: Handle to the window
///   - hRgn: Handle to the region
///   - bRedraw: Specifies whether to redraw the window
/// Returns: TRUE if succeeded, FALSE if failed
fn set_window_rgn(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _hrgn, _redraw) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if hwnd != 0 {
        1 // TRUE - we don't actually track regions
    } else {
        0 // FALSE
    }
}

/// SetWindowContextHelpId - Associates a Help context identifier with the window
/// Parameters:
///   - hWnd: Handle to the window
///   - dwContextHelpId: Help context identifier
/// Returns: TRUE if succeeded, FALSE if failed
fn set_window_context_help_id(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _help_id) = vm_args!(vm, stack_ptr; u32, u32);
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// SetForegroundWindow - Brings window to foreground and activates it
/// Parameters:
///   - hWnd: Handle to the window
/// Returns: TRUE if succeeded, FALSE if failed
fn set_foreground_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd,) = vm_args!(vm, stack_ptr; u32);
    if hwnd != 0 {
        1 // TRUE
    } else {
        0 // FALSE
    }
}

/// InvalidateRect - Adds a rectangle to the window's update region
/// Parameters:
///   - hWnd: Handle to the window (NULL = entire screen)
///   - lpRect: Pointer to RECT structure (NULL = entire client area)
///   - bErase: Whether to erase background
/// Returns: TRUE if succeeded, FALSE if failed
fn invalidate_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Always succeed - we don't track update regions
    1 // TRUE
}

/// InvalidateRgn - Adds a region to the window's update region
/// Parameters:
///   - hWnd: Handle to the window
///   - hRgn: Handle to the region (NULL = entire client area)
///   - bErase: Whether to erase background
/// Returns: TRUE if succeeded (always returns TRUE per MSDN)
fn invalidate_rgn(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Always succeeds per Windows spec
    1 // TRUE
}

/// RedrawWindow - Updates the specified rectangle or region in a window's client area
/// Parameters:
///   - hWnd: Handle to the window (NULL = desktop window)
///   - lprcUpdate: Pointer to RECT structure
///   - hrgnUpdate: Handle to the update region
///   - flags: Redraw flags
/// Returns: TRUE if succeeded, FALSE if failed
fn redraw_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track window painting
    1 // TRUE
}

/// PostMessageA - Places a message in the message queue and returns immediately
/// Parameters:
///   - hWnd: Handle to the window
///   - Msg: Message to post
///   - wParam: Additional message-specific information
///   - lParam: Additional message-specific information
/// Returns: TRUE if succeeded, FALSE if failed
fn post_message_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, _msg, _wparam, _lparam) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    // We don't have a real message queue, but return success for valid windows
    if hwnd != 0 || hwnd == 0xFFFF {
        // 0xFFFF = HWND_BROADCAST
        1 // TRUE
    } else {
        1 // TRUE - posting to NULL hwnd goes to thread message queue
    }
}

/// SendMessageA - Sends a message to a window procedure
/// Parameters:
///   - hWnd: Handle to the window
///   - Msg: Message to send
///   - wParam: Additional message-specific information
///   - lParam: Additional message-specific information
/// Returns: Result of message processing (depends on the message)
fn send_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Without a real window procedure, we just return 0 (common success value)
    0
}

/// RegisterWindowMessageA - Defines a new window message guaranteed to be unique
/// Parameters:
///   - lpString: Pointer to null-terminated string for the message name
/// Returns: Message identifier (0xC000-0xFFFF range), or 0 on failure
fn register_window_message_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (string_ptr,) = vm_args!(vm, stack_ptr; u32);
    if string_ptr == 0 {
        return 0; // Failure - NULL string
    }
    // Return a unique message ID in the registered message range
    // Real Windows starts at 0xC000 and goes up to 0xFFFF
    let msg_id = NEXT_REGISTERED_MSG.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if msg_id > 0xFFFF {
        0 // Out of message IDs (shouldn't happen in practice)
    } else {
        msg_id
    }
}

/// CallWindowProcA - Passes message to the specified window procedure
/// Parameters:
///   - lpPrevWndFunc: Previous window procedure
///   - hWnd: Handle to the window
///   - Msg: Message
///   - wParam: Additional message-specific information
///   - lParam: Additional message-specific information
/// Returns: Result of message processing
fn call_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We can't actually call the previous window procedure
    // Return 0 as default
    0
}

/// DefWindowProcA - Default window procedure for messages not handled
/// Parameters:
///   - hWnd: Handle to the window
///   - Msg: Message
///   - wParam: Additional message-specific information
///   - lParam: Additional message-specific information
/// Returns: Result of message processing (depends on the message)
fn def_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Default processing - return 0 for most messages
    0
}

/// UnregisterClassA - Unregisters a window class
/// Parameters:
///   - lpClassName: Pointer to class name or atom
///   - hInstance: Handle to instance that created the class
/// Returns: TRUE if succeeded, FALSE if failed
fn unregister_class_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // We don't track registered classes, just return success
    1 // TRUE
}

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "ScreenToClient",
        crate::vm::stdcall_args(2),
        screen_to_client,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "ClientToScreen",
        crate::vm::stdcall_args(2),
        client_to_screen,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetWindowRgn",
        crate::vm::stdcall_args(3),
        set_window_rgn,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetWindowContextHelpId",
        crate::vm::stdcall_args(2),
        set_window_context_help_id,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SetForegroundWindow",
        crate::vm::stdcall_args(1),
        set_foreground_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InvalidateRect",
        crate::vm::stdcall_args(3),
        invalidate_rect,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "InvalidateRgn",
        crate::vm::stdcall_args(3),
        invalidate_rgn,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RedrawWindow",
        crate::vm::stdcall_args(4),
        redraw_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "PostMessageA",
        crate::vm::stdcall_args(4),
        post_message_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SendMessageA",
        crate::vm::stdcall_args(4),
        send_message_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "RegisterWindowMessageA",
        crate::vm::stdcall_args(1),
        register_window_message_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "CallWindowProcA",
        crate::vm::stdcall_args(5),
        call_window_proc_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "DefWindowProcA",
        crate::vm::stdcall_args(4),
        def_window_proc_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "UnregisterClassA",
        crate::vm::stdcall_args(2),
        unregister_class_a,
    );
}
