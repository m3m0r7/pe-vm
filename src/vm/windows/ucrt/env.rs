//! UCRT environment stubs.

use crate::vm::Vm;

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
        "_configure_narrow_argv",
        configure_narrow_argv,
    );
}

fn configure_narrow_argv(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn initialize_narrow_environment(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn seh_filter_dll(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
