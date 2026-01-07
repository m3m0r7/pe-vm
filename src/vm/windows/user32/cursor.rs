//! User32 cursor-related stubs.

use crate::register_func_stub;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;

register_func_stub!(DLL_NAME, load_cursor_a, 1);
register_func_stub!(DLL_NAME, load_cursor_w, 1);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "LoadCursorA", crate::vm::stdcall_args(2), load_cursor_a);
    vm.register_import_stdcall(DLL_NAME, "LoadCursorW", crate::vm::stdcall_args(2), load_cursor_w);
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
    fn test_load_cursor_a_returns_handle() {
        let mut vm = create_test_vm();
        let result = load_cursor_a(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_load_cursor_w_returns_handle() {
        let mut vm = create_test_vm();
        let result = load_cursor_w(&mut vm, 0);
        assert_eq!(result, 1);
    }
}
