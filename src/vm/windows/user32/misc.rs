//! Miscellaneous User32 helpers.

use crate::register_func_stub;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

register_func_stub!(DLL_NAME, enable_window, 1);

// Register smaller helpers that don't warrant their own module.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "EnableWindow", crate::vm::stdcall_args(2), enable_window);
    vm.register_import_stdcall(DLL_NAME, "CharNextA", crate::vm::stdcall_args(1), char_next_a);
    vm.register_import_stdcall(DLL_NAME, "CharNextW", crate::vm::stdcall_args(1), char_next_w);
}

fn char_next_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [ptr] = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    ptr.wrapping_add(1)
}

fn char_next_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let [ptr] = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    ptr.wrapping_add(2)
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
    fn test_enable_window_returns_one() {
        let mut vm = create_test_vm();
        let result = enable_window(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_char_next_a_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = char_next_a(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_char_next_a_advances_by_one() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let ptr = 0x1000u32;
        vm.write_u32(stack + 4, ptr).unwrap();
        let result = char_next_a(&mut vm, stack);
        assert_eq!(result, 0x1001);
    }

    #[test]
    fn test_char_next_w_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = char_next_w(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_char_next_w_advances_by_two() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let ptr = 0x1000u32;
        vm.write_u32(stack + 4, ptr).unwrap();
        let result = char_next_w(&mut vm, stack);
        assert_eq!(result, 0x1002);
    }
}
