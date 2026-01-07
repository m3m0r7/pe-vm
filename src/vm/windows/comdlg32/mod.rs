//! COMDLG32 stubs for common dialog usage.

use crate::vm::Vm;
use crate::vm::windows::check_stub;

// Register minimal common dialog APIs used by dialog clients.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "COMDLG32.dll",
        "GetOpenFileNameA",
        crate::vm::stdcall_args(1),
        get_open_file_name_a,
    );
}

fn get_open_file_name_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    check_stub(vm, "COMDLG32.dll", "GetOpenFileNameA");
    0
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
    fn test_get_open_file_name_a_returns_zero() {
        let mut vm = create_test_vm();
        let result = get_open_file_name_a(&mut vm, 0);
        // Returns 0 (FALSE) since dialog is not shown
        assert_eq!(result, 0);
    }
}
