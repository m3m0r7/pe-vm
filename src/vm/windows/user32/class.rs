//! User32 class registration stubs.

use crate::define_stub_fn;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

const WNDCLASSEX_SIZE: u32 = 48;

define_stub_fn!(DLL_NAME, register_class_ex_a, 1);
define_stub_fn!(DLL_NAME, register_class_ex_w, 1);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetClassInfoExA", crate::vm::stdcall_args(3), get_class_info_ex_a);
    vm.register_import_stdcall(DLL_NAME, "GetClassInfoExW", crate::vm::stdcall_args(3), get_class_info_ex_w);
    vm.register_import_stdcall(DLL_NAME, "RegisterClassExA", crate::vm::stdcall_args(1), register_class_ex_a);
    vm.register_import_stdcall(DLL_NAME, "RegisterClassExW", crate::vm::stdcall_args(1), register_class_ex_w);
}

fn get_class_info_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, WNDCLASSEX_SIZE);
        let _ = vm.write_bytes(out_ptr + 4, &[0u8; 44]);
    }
    1
}

fn get_class_info_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, WNDCLASSEX_SIZE);
        let _ = vm.write_bytes(out_ptr + 4, &[0u8; 44]);
    }
    1
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
    fn test_get_class_info_ex_a_writes_wndclassex() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = get_class_info_ex_a(&mut vm, stack);
        assert_eq!(result, 1);
        // Check that cbSize was written
        assert_eq!(vm.read_u32(out_ptr).unwrap(), WNDCLASSEX_SIZE);
    }

    #[test]
    fn test_get_class_info_ex_w_writes_wndclassex() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = get_class_info_ex_w(&mut vm, stack);
        assert_eq!(result, 1);
        assert_eq!(vm.read_u32(out_ptr).unwrap(), WNDCLASSEX_SIZE);
    }

    #[test]
    fn test_get_class_info_ex_a_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 12, 0).unwrap(); // null output
        let result = get_class_info_ex_a(&mut vm, stack);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_register_class_ex_a_returns_atom() {
        let mut vm = create_test_vm();
        let result = register_class_ex_a(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_register_class_ex_w_returns_atom() {
        let mut vm = create_test_vm();
        let result = register_class_ex_w(&mut vm, 0);
        assert_eq!(result, 1);
    }
}
