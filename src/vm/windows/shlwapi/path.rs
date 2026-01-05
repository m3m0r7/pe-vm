//! Path-related SHLWAPI stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("SHLWAPI.dll", "PathFileExistsA", crate::vm::stdcall_args(1), path_file_exists_a);
}

// BOOL PathFileExistsA(LPCSTR pszPath)
fn path_file_exists_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return 0;
    }
    let path = vm.read_c_string(ptr).unwrap_or_default();
    let host_path = vm.map_path(&path);
    if std::path::Path::new(&host_path).exists() {
        1
    } else {
        0
    }
}
