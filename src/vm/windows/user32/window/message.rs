use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "USER32.dll",
        "ScreenToClient",
        crate::vm::stdcall_args(2),
        screen_to_client,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "ClientToScreen",
        crate::vm::stdcall_args(2),
        client_to_screen,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetWindowRgn",
        crate::vm::stdcall_args(3),
        set_window_rgn,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetWindowContextHelpId",
        crate::vm::stdcall_args(2),
        set_window_context_help_id,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SetForegroundWindow",
        crate::vm::stdcall_args(1),
        set_foreground_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "InvalidateRect",
        crate::vm::stdcall_args(3),
        invalidate_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "InvalidateRgn",
        crate::vm::stdcall_args(3),
        invalidate_rgn,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "RedrawWindow",
        crate::vm::stdcall_args(4),
        redraw_window,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "PostMessageA",
        crate::vm::stdcall_args(4),
        post_message_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "SendMessageA",
        crate::vm::stdcall_args(4),
        send_message_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "RegisterWindowMessageA",
        crate::vm::stdcall_args(1),
        register_window_message_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "CallWindowProcA",
        crate::vm::stdcall_args(5),
        call_window_proc_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "DefWindowProcA",
        crate::vm::stdcall_args(4),
        def_window_proc_a,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "UnregisterClassA",
        crate::vm::stdcall_args(2),
        unregister_class_a,
    );
}

pub(super) fn screen_to_client(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn client_to_screen(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn set_window_rgn(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn set_window_context_help_id(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn set_foreground_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn invalidate_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn invalidate_rgn(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn redraw_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn post_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn send_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn register_window_message_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn call_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn def_window_proc_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn unregister_class_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
