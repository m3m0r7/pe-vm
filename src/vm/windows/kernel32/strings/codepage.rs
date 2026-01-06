use crate::vm::{Os, Vm};

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
    default_ansi_codepage(_vm)
}

fn get_oemcp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    default_ansi_codepage(_vm)
}

fn are_file_apis_ansi(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn is_valid_code_page(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_cp_info(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let code_page = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let info_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let resolved = resolve_code_page(vm, code_page);
    if resolved == 932 {
        let _ = vm.write_u32(info_ptr, 2);
        let _ = vm.write_u8(info_ptr + 4, b'?');
        let _ = vm.write_u8(info_ptr + 5, 0);
        let lead_bytes = [(0x81, 0x9F), (0xE0, 0xFC)];
        let mut cursor = info_ptr + 6;
        for (start, end) in lead_bytes {
            let _ = vm.write_u8(cursor, start);
            let _ = vm.write_u8(cursor + 1, end);
            cursor += 2;
        }
        for idx in 0..(12 - lead_bytes.len() * 2) {
            let _ = vm.write_u8(cursor + idx as u32, 0);
        }
    } else {
        let _ = vm.write_u32(info_ptr, 1);
        let _ = vm.write_u8(info_ptr + 4, b'?');
        let _ = vm.write_u8(info_ptr + 5, 0);
        for idx in 0..12 {
            let _ = vm.write_u8(info_ptr + 6 + idx, 0);
        }
    }
    1
}

pub(crate) fn resolve_code_page(vm: &Vm, code_page: u32) -> u32 {
    if code_page == 0 {
        default_ansi_codepage(vm)
    } else {
        code_page
    }
}

fn default_ansi_codepage(vm: &Vm) -> u32 {
    match vm.config().os_value() {
        Os::Windows => 932,
        _ => 65001,
    }
}
