use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "CloseHandle", crate::vm::stdcall_args(1), close_handle);
}

fn close_handle(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if handle != 0 {
        vm.file_close(handle);
    }
    1
}
