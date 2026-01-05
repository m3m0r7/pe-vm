//! PE inspection and execution C ABI.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::pe::{PeFile, ResourceDirectory, ResourceId, ResourceNode};
use crate::vm::{windows, Architecture, ExecuteOptions, Value, Vm, VmConfig};

use super::error::{alloc_string, clear_last_error, set_last_error};

struct ResourceEntry {
    path: String,
    size: u32,
    codepage: u32,
}

#[repr(C)]
pub struct PeHandle {
    file: PeFile,
    image: Vec<u8>,
    resources: Vec<ResourceEntry>,
}

fn resource_id_to_string(id: &ResourceId) -> String {
    match id {
        ResourceId::Id(value) => value.to_string(),
        ResourceId::Name(name) => format!("name:{name}"),
    }
}

fn collect_resource_entries(dir: &ResourceDirectory) -> Vec<ResourceEntry> {
    let mut entries = Vec::new();
    let mut path = Vec::new();
    for node in &dir.roots {
        collect_node_entries(node, &mut path, &mut entries);
    }
    entries
}

fn collect_node_entries(node: &ResourceNode, path: &mut Vec<String>, out: &mut Vec<ResourceEntry>) {
    path.push(resource_id_to_string(&node.id));
    if let Some(data) = &node.data {
        out.push(ResourceEntry {
            path: path.join("/"),
            size: data.size,
            codepage: data.codepage,
        });
    }
    for child in &node.children {
        collect_node_entries(child, path, out);
    }
    path.pop();
}

fn handle_from_ptr<'a>(ptr: *const PeHandle) -> Option<&'a PeHandle> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_ref() }
    }
}

/// # Safety
/// `path` must be a valid, null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn pevm_pe_open(path: *const c_char) -> *mut PeHandle {
    clear_last_error();
    if path.is_null() {
        set_last_error("path is null");
        return std::ptr::null_mut();
    }
    let path = CStr::from_ptr(path);
    let path = match path.to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("path is not valid UTF-8");
            return std::ptr::null_mut();
        }
    };
    let image = match std::fs::read(path) {
        Ok(data) => data,
        Err(err) => {
            set_last_error(format!("failed to read file: {err}"));
            return std::ptr::null_mut();
        }
    };
    let file = match PeFile::parse(&image) {
        Ok(file) => file,
        Err(err) => {
            set_last_error(format!("failed to parse PE: {err}"));
            return std::ptr::null_mut();
        }
    };
    let resources = file
        .directories
        .resource
        .as_ref()
        .map(collect_resource_entries)
        .unwrap_or_default();
    let handle = PeHandle {
        file,
        image,
        resources,
    };
    Box::into_raw(Box::new(handle))
}

/// # Safety
/// `handle` must be returned by `pevm_pe_open` and not freed yet.
#[no_mangle]
pub unsafe extern "C" fn pevm_pe_close(handle: *mut PeHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

#[no_mangle]
pub extern "C" fn pevm_pe_entry_point(handle: *const PeHandle) -> u32 {
    handle_from_ptr(handle)
        .map(|handle| handle.file.optional_header.address_of_entry_point)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_image_base(handle: *const PeHandle) -> u32 {
    handle_from_ptr(handle)
        .map(|handle| handle.file.optional_header.image_base)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_section_count(handle: *const PeHandle) -> usize {
    handle_from_ptr(handle)
        .map(|handle| handle.file.sections.len())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_section_name(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.sections.get(index))
        .map(|section| alloc_string(&section.name))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_section_rva(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.sections.get(index))
        .map(|section| section.virtual_address)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_section_vsize(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.sections.get(index))
        .map(|section| section.virtual_size)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_section_raw_ptr(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.sections.get(index))
        .map(|section| section.raw_ptr)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_export_count(handle: *const PeHandle) -> usize {
    handle_from_ptr(handle)
        .map(|handle| handle.file.exports.len())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_symbol_count(handle: *const PeHandle) -> usize {
    pevm_pe_export_count(handle)
}

#[no_mangle]
pub extern "C" fn pevm_pe_export_ordinal(handle: *const PeHandle, index: usize) -> u16 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.exports.get(index))
        .map(|symbol| symbol.ordinal)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_export_rva(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.exports.get(index))
        .map(|symbol| symbol.rva)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_symbol_rva(handle: *const PeHandle, index: usize) -> u32 {
    pevm_pe_export_rva(handle, index)
}

#[no_mangle]
pub extern "C" fn pevm_pe_export_name(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.exports.get(index))
        .and_then(|symbol| symbol.name.as_deref())
        .map(alloc_string)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_symbol_name(handle: *const PeHandle, index: usize) -> *mut c_char {
    pevm_pe_export_name(handle, index)
}

#[no_mangle]
pub extern "C" fn pevm_pe_export_forwarder(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.exports.get(index))
        .and_then(|symbol| symbol.forwarder.as_deref())
        .map(alloc_string)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_import_count(handle: *const PeHandle) -> usize {
    handle_from_ptr(handle)
        .map(|handle| handle.file.imports.len())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_import_module(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.imports.get(index))
        .map(|import| alloc_string(&import.module))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_import_name(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.imports.get(index))
        .and_then(|import| import.name.as_deref())
        .map(alloc_string)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_import_ordinal(handle: *const PeHandle, index: usize) -> u16 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.imports.get(index))
        .and_then(|import| import.ordinal)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_import_iat_rva(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.file.imports.get(index))
        .map(|import| import.iat_rva)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_resource_count(handle: *const PeHandle) -> usize {
    handle_from_ptr(handle)
        .map(|handle| handle.resources.len())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_resource_path(handle: *const PeHandle, index: usize) -> *mut c_char {
    handle_from_ptr(handle)
        .and_then(|handle| handle.resources.get(index))
        .map(|entry| alloc_string(&entry.path))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn pevm_pe_resource_size(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.resources.get(index))
        .map(|entry| entry.size)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pevm_pe_resource_codepage(handle: *const PeHandle, index: usize) -> u32 {
    handle_from_ptr(handle)
        .and_then(|handle| handle.resources.get(index))
        .map(|entry| entry.codepage)
        .unwrap_or(0)
}

/// # Safety
/// `handle` and `symbol` must be valid pointers, and `args` must point to
/// `args_len` elements when `args_len > 0`.
#[no_mangle]
pub unsafe extern "C" fn pevm_pe_execute_symbol_u32(
    handle: *const PeHandle,
    symbol: *const c_char,
    args: *const u32,
    args_len: usize,
) -> u32 {
    clear_last_error();
    let handle = match handle_from_ptr(handle) {
        Some(handle) => handle,
        None => {
            set_last_error("handle is null");
            return 0;
        }
    };
    if symbol.is_null() {
        set_last_error("symbol is null");
        return 0;
    }
    let symbol = CStr::from_ptr(symbol);
    let symbol = match symbol.to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("symbol is not valid UTF-8");
            return 0;
        }
    };
    if args_len > 0 && args.is_null() {
        set_last_error("args is null");
        return 0;
    }
    let values = if args_len == 0 {
        Vec::new()
    } else {
        let slice = std::slice::from_raw_parts(args, args_len);
        slice.iter().copied().map(Value::U32).collect()
    };

    let mut vm = match Vm::new(VmConfig::new().architecture(Architecture::X86)) {
        Ok(vm) => vm,
        Err(err) => {
            set_last_error(format!("failed to create VM: {err}"));
            return 0;
        }
    };
    if let Err(err) = vm.load_image(&handle.file, &handle.image) {
        set_last_error(format!("failed to load image: {err}"));
        return 0;
    }
    windows::register_default(&mut vm);
    if let Err(err) = vm.resolve_imports(&handle.file) {
        set_last_error(format!("failed to resolve imports: {err}"));
        return 0;
    }
    match vm.execute_export_with_values(&handle.file, symbol, &values, ExecuteOptions::new()) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("execution failed: {err}"));
            0
        }
    }
}
