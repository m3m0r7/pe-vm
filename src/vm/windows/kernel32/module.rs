//! Kernel32 module/loader stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("KERNEL32.dll", "GetModuleHandleA", crate::vm::stdcall_args(1), get_module_handle_a);
    vm.register_import_stdcall("KERNEL32.dll", "GetModuleHandleW", crate::vm::stdcall_args(1), get_module_handle_w);
    vm.register_import_stdcall("KERNEL32.dll", "GetModuleHandleExW", crate::vm::stdcall_args(3), get_module_handle_ex_w);
    vm.register_import_stdcall("KERNEL32.dll", "GetModuleFileNameA", crate::vm::stdcall_args(3), get_module_file_name_a);
    vm.register_import_stdcall("KERNEL32.dll", "GetModuleFileNameW", crate::vm::stdcall_args(3), get_module_file_name_w);
    vm.register_import_stdcall("KERNEL32.dll", "LoadLibraryA", crate::vm::stdcall_args(1), load_library_a);
    vm.register_import_stdcall("KERNEL32.dll", "LoadLibraryExA", crate::vm::stdcall_args(3), load_library_ex_a);
    vm.register_import_stdcall("KERNEL32.dll", "LoadLibraryExW", crate::vm::stdcall_args(3), load_library_ex_w);
    vm.register_import_stdcall("KERNEL32.dll", "FreeLibrary", crate::vm::stdcall_args(1), free_library);
    vm.register_import_stdcall("KERNEL32.dll", "GetProcAddress", crate::vm::stdcall_args(2), get_proc_address);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "DisableThreadLibraryCalls",
        crate::vm::stdcall_args(1),
        disable_thread_library_calls,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetCommandLineA", crate::vm::stdcall_args(0), get_command_line_a);
    vm.register_import_stdcall("KERNEL32.dll", "FindResourceA", crate::vm::stdcall_args(3), find_resource_a);
    vm.register_import_stdcall("KERNEL32.dll", "FindResourceW", crate::vm::stdcall_args(3), find_resource_w);
    vm.register_import_stdcall("KERNEL32.dll", "FindResourceExW", crate::vm::stdcall_args(4), find_resource_ex_w);
    vm.register_import_stdcall("KERNEL32.dll", "LoadResource", crate::vm::stdcall_args(2), load_resource);
    vm.register_import_stdcall("KERNEL32.dll", "LockResource", crate::vm::stdcall_args(1), lock_resource);
    vm.register_import_stdcall("KERNEL32.dll", "SizeofResource", crate::vm::stdcall_args(2), sizeof_resource);
}

fn get_module_handle_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if name == 0 {
        return vm.base();
    }
    vm.base()
}

fn get_module_handle_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if name == 0 {
        return vm.base();
    }
    vm.base()
}

fn get_module_handle_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, vm.base());
    }
    1
}

fn get_module_file_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if buffer == 0 || size == 0 {
        return 0;
    }
    let path = vm
        .image_path()
        .unwrap_or("C:\\pe_vm\\module.dll");
    let mut bytes = path.as_bytes().to_vec();
    if bytes.len() >= size {
        bytes.truncate(size.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(buffer, &bytes);
    (bytes.len().saturating_sub(1)) as u32
}

fn get_module_file_name_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if buffer == 0 || size == 0 {
        return 0;
    }
    let path = vm
        .image_path()
        .unwrap_or("C:\\pe_vm\\module.dll");
    let mut utf16: Vec<u16> = path.encode_utf16().collect();
    if utf16.len() >= size {
        utf16.truncate(size.saturating_sub(1));
    }
    for (i, unit) in utf16.iter().enumerate() {
        let _ = vm.write_u16(buffer + (i as u32) * 2, *unit);
    }
    let _ = vm.write_u16(buffer + (utf16.len() as u32) * 2, 0);
    utf16.len() as u32
}

fn load_library_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.base()
}

fn load_library_ex_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.base()
}

fn load_library_ex_w(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.base()
}

fn free_library(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_proc_address(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let module = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let name_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let name = if name_ptr & 0xFFFF_0000 == 0 {
        format!("#{}", name_ptr & 0xFFFF)
    } else {
        vm.read_c_string(name_ptr).unwrap_or_default()
    };
    if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() || std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] GetProcAddress: module=0x{module:08X} name={name}"
        );
    }
    vm.resolve_dynamic_import(&name).unwrap_or(0)
}

fn disable_thread_library_calls(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_command_line_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let line = "pe_vm.exe\0";
    vm.alloc_bytes(line.as_bytes(), 1).unwrap_or(0)
}

fn find_resource_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if name != 0 {
        return name;
    }
    1
}

fn find_resource_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if name != 0 {
        return name;
    }
    1
}

fn find_resource_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let name = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if name != 0 {
        return name;
    }
    1
}

fn load_resource(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 8).unwrap_or(0)
}

fn lock_resource(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 4).unwrap_or(0)
}

fn sizeof_resource(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}
