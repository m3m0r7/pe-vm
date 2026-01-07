//! mscoree stubs (CLR loader entry points).
#![allow(non_snake_case)]

use crate::define_stub_fn;
use crate::vm::Vm;

pub const DLL_NAME: &str = "mscoree.dll";

define_stub_fn!(DLL_NAME, _CorExeMain, 0);
define_stub_fn!(DLL_NAME, _CorExeMain2, 0);
define_stub_fn!(DLL_NAME, _CorImageUnloading, 0);
define_stub_fn!(DLL_NAME, _CorValidateImage, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "_CorDllMain",
        crate::vm::stdcall_args(3),
        cor_dll_main,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "_CorExeMain",
        crate::vm::stdcall_args(2),
        _CorExeMain,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "_CorExeMain2",
        crate::vm::stdcall_args(5),
        _CorExeMain2,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "_CorImageUnloading",
        crate::vm::stdcall_args(1),
        _CorImageUnloading,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "_CorValidateImage",
        crate::vm::stdcall_args(1),
        _CorValidateImage,
    );
}

fn cor_dll_main(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
