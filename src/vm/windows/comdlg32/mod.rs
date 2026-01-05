//! COMDLG32 stubs for common dialog usage.

use crate::vm::Vm;

// Register minimal common dialog APIs used by JVLink.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "COMDLG32.dll",
        "GetOpenFileNameA",
        crate::vm::stdcall_args(1),
        get_open_file_name_a,
    );
}

fn get_open_file_name_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    // Report "cancel" so callers can fall back without crashing.
    0
}
