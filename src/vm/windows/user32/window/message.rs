use crate::define_stub_fn;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;

define_stub_fn!(DLL_NAME, screen_to_client, 1);
define_stub_fn!(DLL_NAME, client_to_screen, 1);
define_stub_fn!(DLL_NAME, set_window_rgn, 1);
define_stub_fn!(DLL_NAME, set_window_context_help_id, 1);
define_stub_fn!(DLL_NAME, set_foreground_window, 1);
define_stub_fn!(DLL_NAME, invalidate_rect, 1);
define_stub_fn!(DLL_NAME, invalidate_rgn, 1);
define_stub_fn!(DLL_NAME, redraw_window, 1);
define_stub_fn!(DLL_NAME, post_message_a, 1);
define_stub_fn!(DLL_NAME, send_message_a, 0);
define_stub_fn!(DLL_NAME, register_window_message_a, 1);
define_stub_fn!(DLL_NAME, call_window_proc_a, 0);
define_stub_fn!(DLL_NAME, def_window_proc_a, 0);
define_stub_fn!(DLL_NAME, unregister_class_a, 1);

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
