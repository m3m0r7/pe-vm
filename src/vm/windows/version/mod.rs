//! VERSION.dll stubs.

pub const DLL_NAME: &str = "VERSION.dll";

use crate::define_stub_fn;
use crate::vm::windows::check_stub;
use crate::vm::Vm;
use crate::vm_args;

define_stub_fn!(DLL_NAME, get_file_version_info_a, 0);
define_stub_fn!(DLL_NAME, ver_query_value_a, 0);

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "GetFileVersionInfoSizeA",
        crate::vm::stdcall_args(2),
        get_file_version_info_size_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetFileVersionInfoA",
        crate::vm::stdcall_args(4),
        get_file_version_info_a,
    );
    vm.register_import_stdcall(DLL_NAME, "VerQueryValueA", crate::vm::stdcall_args(4), ver_query_value_a);
}

fn get_file_version_info_size_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    check_stub(vm, DLL_NAME, "GetFileVersionInfoSizeA");
    let (_, handle_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if handle_ptr != 0 {
        let _ = vm.write_u32(handle_ptr, 0);
    }
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
    fn test_get_file_version_info_size_a_returns_zero() {
        let mut vm = create_test_vm();
        let result = get_file_version_info_size_a(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_file_version_info_size_a_clears_handle() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let handle_ptr = vm.heap_start as u32;
        vm.write_u32(handle_ptr, 0xDEADBEEF).unwrap();
        vm.write_u32(stack + 8, handle_ptr).unwrap();
        let result = get_file_version_info_size_a(&mut vm, stack);
        assert_eq!(result, 0);
        assert_eq!(vm.read_u32(handle_ptr).unwrap(), 0);
    }

    #[test]
    fn test_get_file_version_info_a_returns_zero() {
        let mut vm = create_test_vm();
        let result = get_file_version_info_a(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ver_query_value_a_returns_zero() {
        let mut vm = create_test_vm();
        let result = ver_query_value_a(&mut vm, 0);
        assert_eq!(result, 0);
    }
}
