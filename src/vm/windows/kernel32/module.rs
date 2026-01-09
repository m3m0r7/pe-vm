//! Kernel32 module/loader stubs.

use crate::pe::{ResourceData, ResourceId, ResourceNode};
use crate::vm::windows::kernel32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "GetModuleHandleA",
        crate::vm::stdcall_args(1),
        get_module_handle_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetModuleHandleW",
        crate::vm::stdcall_args(1),
        get_module_handle_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetModuleHandleExW",
        crate::vm::stdcall_args(3),
        get_module_handle_ex_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetModuleFileNameA",
        crate::vm::stdcall_args(3),
        get_module_file_name_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetModuleFileNameW",
        crate::vm::stdcall_args(3),
        get_module_file_name_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadLibraryA",
        crate::vm::stdcall_args(1),
        load_library_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadLibraryExA",
        crate::vm::stdcall_args(3),
        load_library_ex_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadLibraryExW",
        crate::vm::stdcall_args(3),
        load_library_ex_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "FreeLibrary",
        crate::vm::stdcall_args(1),
        free_library,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetProcAddress",
        crate::vm::stdcall_args(2),
        get_proc_address,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "DisableThreadLibraryCalls",
        crate::vm::stdcall_args(1),
        disable_thread_library_calls,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCommandLineA",
        crate::vm::stdcall_args(0),
        get_command_line_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "GetCommandLineW",
        crate::vm::stdcall_args(0),
        get_command_line_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "FindResourceA",
        crate::vm::stdcall_args(3),
        find_resource_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "FindResourceW",
        crate::vm::stdcall_args(3),
        find_resource_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "FindResourceExW",
        crate::vm::stdcall_args(4),
        find_resource_ex_w,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadResource",
        crate::vm::stdcall_args(2),
        load_resource,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LockResource",
        crate::vm::stdcall_args(1),
        lock_resource,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "SizeofResource",
        crate::vm::stdcall_args(2),
        sizeof_resource,
    );
}

fn get_module_handle_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (name,) = vm_args!(vm, stack_ptr; u32);
    if name == 0 {
        return vm.main_module_handle();
    }
    let name = read_wide_or_utf16le_str!(vm, name);
    vm.module_handle_by_name(&name).unwrap_or(0)
}

fn get_module_handle_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (name,) = vm_args!(vm, stack_ptr; u32);
    if name == 0 {
        return vm.main_module_handle();
    }
    let name = read_wide_or_utf16le_str!(vm, name);
    vm.module_handle_by_name(&name).unwrap_or(0)
}

fn get_module_handle_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, name, out) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let handle = if name == 0 {
        vm.main_module_handle()
    } else {
        let name = read_wide_or_utf16le_str!(vm, name);
        vm.module_handle_by_name(&name).unwrap_or(0)
    };
    if out != 0 {
        let _ = vm.write_u32(out, handle);
    }
    if handle == 0 {
        0
    } else {
        1
    }
}

fn get_module_file_name_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (module, buffer, size) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let size = size as usize;
    if buffer == 0 || size == 0 {
        return 0;
    }
    let path = vm
        .module_by_handle(module)
        .and_then(|module| {
            if module.guest_path.is_empty() {
                None
            } else {
                Some(module.guest_path.as_str())
            }
        })
        .or_else(|| vm.image_path())
        .unwrap_or("C:\\pe_vm\\module.dll");
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetModuleFileNameA: {path}");
    }
    let mut bytes = path.as_bytes().to_vec();
    if bytes.len() >= size {
        bytes.truncate(size.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(buffer, &bytes);
    (bytes.len().saturating_sub(1)) as u32
}

fn get_module_file_name_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (module, buffer, size) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let size = size as usize;
    if buffer == 0 || size == 0 {
        return 0;
    }
    let path = vm
        .module_by_handle(module)
        .and_then(|module| {
            if module.guest_path.is_empty() {
                None
            } else {
                Some(module.guest_path.as_str())
            }
        })
        .or_else(|| vm.image_path())
        .unwrap_or("C:\\pe_vm\\module.dll");
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetModuleFileNameW: {path}");
    }
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
    let (path_ptr,) = vm_args!(vm, _stack_ptr; u32);
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    if path.is_empty() {
        return 0;
    }
    vm.load_library(&path).unwrap_or(0)
}

fn load_library_ex_a(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let (path_ptr, _, _) = vm_args!(vm, _stack_ptr; u32, u32, u32);
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    if path.is_empty() {
        return 0;
    }
    vm.load_library(&path).unwrap_or(0)
}

fn load_library_ex_w(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let (path_ptr, _, _) = vm_args!(vm, _stack_ptr; u32, u32, u32);
    let path = read_wide_or_utf16le_str!(vm, path_ptr);
    if path.is_empty() {
        return 0;
    }
    vm.load_library(&path).unwrap_or(0)
}

fn free_library(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_proc_address(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (module, name_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    let name = if name_ptr & 0xFFFF_0000 == 0 {
        format!("#{}", name_ptr & 0xFFFF)
    } else {
        read_wide_or_utf16le_str!(vm, name_ptr)
    };
    if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() || std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] GetProcAddress: module=0x{module:08X} name={name}");
    }
    if module != 0 {
        if let Some(module) = vm.module_by_handle(module) {
            if let Some(stripped) = name.strip_prefix('#') {
                if let Ok(ordinal) = stripped.parse::<u16>() {
                    if let Some(symbol) =
                        module.pe.exports.iter().find(|sym| sym.ordinal == ordinal)
                    {
                        return module.base.wrapping_add(symbol.rva);
                    }
                }
            } else if let Some(rva) = module.pe.export_rva(&name) {
                return module.base.wrapping_add(rva);
            }
            return 0;
        }
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

fn get_command_line_w(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let line = "pe_vm.exe";
    let mut utf16: Vec<u16> = line.encode_utf16().collect();
    utf16.push(0);
    let mut bytes = Vec::with_capacity(utf16.len() * 2);
    for unit in utf16 {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 2).unwrap_or(0)
}

fn find_resource_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, name, r#type) = vm_args!(vm, stack_ptr; u32, u32, u32);
    find_resource(vm, name, r#type, false)
}

fn find_resource_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, name, r#type) = vm_args!(vm, stack_ptr; u32, u32, u32);
    find_resource(vm, name, r#type, true)
}

fn find_resource_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, r#type, name) = vm_args!(vm, stack_ptr; u32, u32, u32);
    find_resource(vm, name, r#type, true)
}

fn load_resource(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, handle) = vm_args!(vm, stack_ptr; u32, u32);
    handle
}

fn lock_resource(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (handle,) = vm_args!(vm, stack_ptr; u32);
    handle
}

fn sizeof_resource(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, handle) = vm_args!(vm, stack_ptr; u32, u32);
    vm.resource_sizes.get(&handle).copied().unwrap_or(0)
}

fn find_resource(vm: &mut Vm, name: u32, r#type: u32, wide: bool) -> u32 {
    let (bytes, size) = {
        let Some(dir) = vm.resource_dir() else {
            return 0;
        };
        let name_id = read_resource_id(vm, name, wide);
        let type_id = read_resource_id(vm, r#type, wide);
        let (Some(name_id), Some(type_id)) = (name_id, type_id) else {
            return 0;
        };
        let Some(data) = lookup_resource(dir.roots.as_slice(), &type_id, &name_id) else {
            return 0;
        };
        (data.data.clone(), data.size)
    };
    let ptr = vm.alloc_bytes(&bytes, 1).unwrap_or(0);
    if ptr != 0 {
        vm.resource_sizes.insert(ptr, size);
    }
    ptr
}

fn read_resource_id(vm: &Vm, value: u32, wide: bool) -> Option<ResourceId> {
    if value == 0 {
        return None;
    }
    if value & 0xFFFF_0000 == 0 {
        return Some(ResourceId::Id(value));
    }
    if wide {
        let name = read_w_string(vm, value);
        if name.is_empty() {
            None
        } else {
            Some(ResourceId::Name(name))
        }
    } else {
        let name = read_wide_or_utf16le_str!(vm, value);
        if name.is_empty() {
            None
        } else {
            Some(ResourceId::Name(name))
        }
    }
}

fn lookup_resource<'a>(
    roots: &'a [ResourceNode],
    type_id: &ResourceId,
    name_id: &ResourceId,
) -> Option<&'a ResourceData> {
    let type_node = roots
        .iter()
        .find(|node| resource_id_eq(&node.id, type_id))?;
    let name_node = type_node
        .children
        .iter()
        .find(|node| resource_id_eq(&node.id, name_id))?;
    if let Some(data) = name_node.data.as_ref() {
        return Some(data);
    }
    for child in &name_node.children {
        if let Some(data) = child.data.as_ref() {
            return Some(data);
        }
    }
    None
}

fn resource_id_eq(left: &ResourceId, right: &ResourceId) -> bool {
    match (left, right) {
        (ResourceId::Id(a), ResourceId::Id(b)) => a == b,
        (ResourceId::Name(a), ResourceId::Name(b)) => a.eq_ignore_ascii_case(b),
        _ => false,
    }
}

fn read_w_string(vm: &Vm, ptr: u32) -> String {
    let mut units = Vec::new();
    let mut cursor = ptr;
    loop {
        let unit = vm.read_u16(cursor).unwrap_or(0);
        if unit == 0 {
            break;
        }
        units.push(unit);
        cursor = cursor.wrapping_add(2);
    }
    String::from_utf16_lossy(&units)
}
