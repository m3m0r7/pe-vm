//! IMAGEHLP.dll stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "imagehlp.dll",
        "MakeSureDirectoryPathExists",
        crate::vm::stdcall_args(1),
        make_sure_directory_path_exists,
    );
}

fn make_sure_directory_path_exists(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
