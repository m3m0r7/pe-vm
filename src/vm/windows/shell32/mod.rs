//! SHELL32 stubs for shell execution helpers.

pub const DLL_NAME: &str = "SHELL32.dll";

use crate::register_func_stub;
use crate::vm::Vm;
use crate::vm_args;

register_func_stub!(DLL_NAME, shell_execute_a, 33);
register_func_stub!(DLL_NAME, shell_execute_ex_a, 1);
register_func_stub!(DLL_NAME, sh_browse_for_folder_a, 0);
register_func_stub!(DLL_NAME, sh_get_file_info_a, 0);

// Register shell entry points that may be imported by GUI DLLs.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "ShellExecuteA", crate::vm::stdcall_args(6), shell_execute_a);
    vm.register_import_stdcall(DLL_NAME, "ShellExecuteExA", crate::vm::stdcall_args(1), shell_execute_ex_a);
    vm.register_import_stdcall(DLL_NAME, "SHBrowseForFolderA", crate::vm::stdcall_args(1), sh_browse_for_folder_a);
    vm.register_import_stdcall(DLL_NAME, "SHGetPathFromIDListA", crate::vm::stdcall_args(2), sh_get_path_from_id_list_a);
    vm.register_import_stdcall(DLL_NAME, "SHGetFileInfoA", crate::vm::stdcall_args(5), sh_get_file_info_a);
}

fn sh_get_path_from_id_list_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    crate::vm::windows::check_stub(vm, DLL_NAME, "SHGetPathFromIDListA");
    let (_, buffer) = vm_args!(vm, stack_ptr; u32, u32);
    if buffer != 0 {
        let _ = vm.write_bytes(buffer, b"C:\\\0");
        1
    } else {
        0
    }
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
    fn test_shell_execute_a_returns_success() {
        let mut vm = create_test_vm();
        let result = shell_execute_a(&mut vm, 0);
        // ShellExecuteA returns > 32 on success
        assert!(result > 32);
    }

    #[test]
    fn test_shell_execute_ex_a_returns_success() {
        let mut vm = create_test_vm();
        let result = shell_execute_ex_a(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sh_browse_for_folder_a_returns_null() {
        let mut vm = create_test_vm();
        let result = sh_browse_for_folder_a(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sh_get_path_from_id_list_a_null_buffer() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 8, 0).unwrap(); // null buffer
        let result = sh_get_path_from_id_list_a(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sh_get_path_from_id_list_a_with_buffer() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let buffer = vm.heap_start as u32;
        vm.write_u32(stack + 8, buffer).unwrap();
        let result = sh_get_path_from_id_list_a(&mut vm, stack);
        assert_eq!(result, 1);
        // Should write "C:\" to buffer
        let path = vm.read_c_string(buffer).unwrap();
        assert_eq!(path, "C:\\");
    }

    #[test]
    fn test_sh_get_file_info_a_returns_zero() {
        let mut vm = create_test_vm();
        let result = sh_get_file_info_a(&mut vm, 0);
        assert_eq!(result, 0);
    }
}
