//! IMAGEHLP.dll stubs.

pub const DLL_NAME: &str = "imagehlp.dll";

use crate::register_func_stub;
use crate::vm::Vm;

register_func_stub!(DLL_NAME, make_sure_directory_path_exists, 1);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "MakeSureDirectoryPathExists",
        crate::vm::stdcall_args(1),
        make_sure_directory_path_exists,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BypassSettings;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut bypass = BypassSettings::new();
        bypass.not_implemented_module = true;
        let mut vm = Vm::new(
            VmConfig::new()
                .architecture(Architecture::X86)
                .bypass(bypass),
        )
        .expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_make_sure_directory_path_exists_returns_true() {
        let mut vm = create_test_vm();
        let result = make_sure_directory_path_exists(&mut vm, 0);
        // Returns TRUE (success)
        assert_eq!(result, 1);
    }
}
