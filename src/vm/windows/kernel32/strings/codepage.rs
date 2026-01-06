use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetACP", crate::vm::stdcall_args(0), get_acp);
    vm.register_import_stdcall("KERNEL32.dll", "GetOEMCP", crate::vm::stdcall_args(0), get_oemcp);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "AreFileApisANSI",
        crate::vm::stdcall_args(0),
        are_file_apis_ansi,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "IsValidCodePage",
        crate::vm::stdcall_args(1),
        is_valid_code_page,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetCPInfo", crate::vm::stdcall_args(2), get_cp_info);
}

fn get_acp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn get_oemcp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    65001
}

fn are_file_apis_ansi(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn is_valid_code_page(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_cp_info(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let _ = vm.write_u32(info_ptr, 1);
    let _ = vm.write_u8(info_ptr + 4, 0);
    for idx in 0..12 {
        let _ = vm.write_u8(info_ptr + 6 + idx, 0);
    }
    1
}
