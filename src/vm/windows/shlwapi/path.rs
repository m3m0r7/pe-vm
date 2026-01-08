//! Path-related SHLWAPI stubs.

use crate::vm::windows::shlwapi::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "PathFileExistsA",
        crate::vm::stdcall_args(1),
        path_file_exists_a,
    );
}

// BOOL PathFileExistsA(LPCSTR pszPath)
fn path_file_exists_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    let path = read_wide_or_utf16le_str!(vm, ptr);
    let host_path = vm.map_path(&path);
    let exists = std::path::Path::new(&host_path).exists();
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] PathFileExistsA: {path} -> {exists}");
    }
    if exists {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
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
    fn test_path_file_exists_a_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap(); // null path pointer
        let result = path_file_exists_a(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_path_file_exists_a_nonexistent_path() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let path_ptr = vm.heap_start as u32;
        vm.write_bytes(path_ptr, b"C:\\nonexistent\\path\\file.txt\0")
            .unwrap();
        vm.write_u32(stack + 4, path_ptr).unwrap();
        let result = path_file_exists_a(&mut vm, stack);
        assert_eq!(result, 0);
    }
}
