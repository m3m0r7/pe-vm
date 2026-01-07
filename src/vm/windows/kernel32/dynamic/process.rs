use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "FlushProcessWriteBuffers",
        crate::vm::stdcall_args(0),
        flush_process_write_buffers,
    );
    vm.register_import_any_stdcall(
        "GetCurrentProcessorNumber",
        crate::vm::stdcall_args(0),
        get_current_processor_number,
    );
    vm.register_import_any_stdcall(
        "GetLogicalProcessorInformation",
        crate::vm::stdcall_args(2),
        get_logical_processor_information,
    );
    vm.register_import_any_stdcall("CreateSymbolicLinkW", crate::vm::stdcall_args(3), create_symbolic_link_w);
    vm.register_import_any_stdcall(
        "SetDefaultDllDirectories",
        crate::vm::stdcall_args(1),
        set_default_dll_directories,
    );
}

fn flush_process_write_buffers(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_current_processor_number(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_logical_processor_information(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (buffer_ptr, len_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if len_ptr != 0 {
        let _ = vm.write_u32(len_ptr, 0);
    }
    if buffer_ptr == 0 {
        return 0;
    }
    0
}

fn create_symbolic_link_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_default_dll_directories(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
