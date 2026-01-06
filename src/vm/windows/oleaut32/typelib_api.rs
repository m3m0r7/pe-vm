//! ITypeLib stubs and registration helpers.

use crate::vm::{Vm, VmError};

use crate::vm::windows::get_registry;
use crate::vm::windows::guid::format_guid;
use crate::vm::windows::registry::RegistryValue;

use super::bstr::read_utf16_z;
use super::constants::{
    E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, IID_ITYPELIB, IID_IUNKNOWN, OleMethod, S_OK,
    TYPE_E_LIBNOTREGISTERED,
};
use super::guid::{guid_matches, read_guid_bytes};
use super::typeinfo::TYPEINFO_METHODS;
use super::typelib;

pub(super) const TYPELIB_METHODS: &[OleMethod] = &[
    ("pe_vm.typelib.QueryInterface", 3, typelib_query_interface),
    ("pe_vm.typelib.AddRef", 1, typelib_add_ref),
    ("pe_vm.typelib.Release", 1, typelib_release),
    ("pe_vm.typelib.GetTypeInfoCount", 1, typelib_get_typeinfo_count),
    ("pe_vm.typelib.GetTypeInfo", 3, typelib_get_typeinfo),
    ("pe_vm.typelib.GetTypeInfoType", 3, typelib_get_typeinfo_type),
    ("pe_vm.typelib.GetTypeInfoOfGuid", 3, typelib_get_typeinfo_of_guid),
    ("pe_vm.typelib.GetLibAttr", 2, typelib_not_impl),
    ("pe_vm.typelib.GetTypeComp", 2, typelib_not_impl),
    ("pe_vm.typelib.GetDocumentation", 6, typelib_not_impl),
    ("pe_vm.typelib.IsName", 4, typelib_not_impl),
    ("pe_vm.typelib.FindName", 6, typelib_not_impl),
    ("pe_vm.typelib.ReleaseTLibAttr", 2, typelib_not_impl),
];

// RegisterTypeLib(...)
pub(super) fn register_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    S_OK
}

// LoadTypeLib(...)
pub(super) fn load_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out == 0 {
        return E_INVALIDARG;
    }
    let text = match read_utf16_z(vm, path_ptr) {
        Ok(units) => String::from_utf16_lossy(&units),
        Err(_) => return E_INVALIDARG,
    };
    let path = text.trim().trim_matches('"');
    if path.is_empty() {
        return E_INVALIDARG;
    }
    match load_typelib_from_path(vm, path, None) {
        Ok(ptr) => {
            let _ = vm.write_u32(out, ptr);
            S_OK
        }
        Err(_) => E_NOTIMPL,
    }
}

// LoadRegTypeLib(...)
pub(super) fn load_reg_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let guid_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let major = vm.read_u32(stack_ptr + 8).unwrap_or(0) as u16;
    let minor = vm.read_u32(stack_ptr + 12).unwrap_or(0) as u16;
    let out = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if out == 0 {
        return E_INVALIDARG;
    }
    let guid_bytes = match read_guid_bytes(vm, guid_ptr) {
        Some(value) => value,
        None => return E_INVALIDARG,
    };
    let guid = format_guid(&guid_bytes);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] LoadRegTypeLib guid={guid} version={major}.{minor}");
    }
    let path = match resolve_typelib_path(vm, &guid, major, minor) {
        Ok(Some(value)) => value,
        Ok(None) => match vm.image_path() {
            Some(path) => path.to_string(),
            None => return TYPE_E_LIBNOTREGISTERED,
        },
        Err(_) => return E_NOTIMPL,
    };
    match load_typelib_from_path(vm, &path, Some(guid_bytes)) {
        Ok(ptr) => {
            let _ = vm.write_u32(out, ptr);
            S_OK
        }
        Err(_) => E_NOTIMPL,
    }
}

fn load_typelib_from_path(
    vm: &mut Vm,
    path: &str,
    expected_guid: Option<[u8; 16]>,
) -> Result<u32, VmError> {
    let host_path = vm.map_path(path);
    let bytes = std::fs::read(host_path)?;
    let lib = match typelib::load_from_bytes(&bytes) {
        Ok(lib) => lib,
        Err(err) => {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!("[pe_vm] TypeLib load failed: {err}");
            }
            return Err(err);
        }
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] TypeLib loaded guid={}", format_guid(&lib.guid));
    }
    if let Some(expected) = expected_guid {
        if lib.guid != expected {
            return Err(VmError::InvalidConfig("typelib guid mismatch"));
        }
    }
    alloc_typelib(vm, lib)
}

fn resolve_typelib_path(
    vm: &Vm,
    guid: &str,
    major: u16,
    minor: u16,
) -> Result<Option<String>, VmError> {
    let registry =
        get_registry(vm).ok_or(VmError::InvalidConfig("windows registry unavailable"))?;
    let version = format!("{major}.{minor}");
    let candidates = [
        format!(r"HKCR\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKLM\\Software\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKCU\\Software\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(
            r"HKLM\\Software\\Classes\\WOW6432Node\\TypeLib\\{guid}\\{version}\\0\\win32"
        ),
        format!(
            r"HKLM\\Software\\WOW6432Node\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"
        ),
    ];
    for key in candidates {
        let value = registry
            .get(&key)
            .map_err(|_| VmError::InvalidConfig("registry key"))?;
        let Some(RegistryValue::String(path)) = value else {
            continue;
        };
        return Ok(Some(path.to_string()));
    }
    Ok(None)
}

fn alloc_typelib(vm: &mut Vm, lib: typelib::TypeLib) -> Result<u32, VmError> {
    register_typelib_thunks(vm);
    register_typeinfo_thunks(vm);
    let lib_id = typelib::store_typelib(lib);
    let vtable = build_vtable(vm, TYPELIB_METHODS)?;
    build_object(vm, vtable, &[lib_id])
}

fn build_typeinfo_object(vm: &mut Vm, typeinfo_id: u32) -> Result<u32, VmError> {
    let vtable = build_vtable(vm, TYPEINFO_METHODS)?;
    build_object(vm, vtable, &[typeinfo_id])
}

fn build_vtable(vm: &mut Vm, methods: &[OleMethod]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(methods.len() * 4);
    for &(name, _, _) in methods {
        let entry = vm
            .resolve_dynamic_import(name)
            .ok_or(VmError::InvalidConfig("missing import"))?;
        bytes.extend_from_slice(&entry.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn build_object(vm: &mut Vm, vtable_ptr: u32, extras: &[u32]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity((1 + extras.len()) * 4);
    bytes.extend_from_slice(&vtable_ptr.to_le_bytes());
    for extra in extras {
        bytes.extend_from_slice(&extra.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn register_typelib_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPELIB_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn register_typeinfo_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPEINFO_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn typelib_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let ok = guid_matches(vm, iid_ptr, IID_IUNKNOWN) || guid_matches(vm, iid_ptr, IID_ITYPELIB);
    if ok {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

fn typelib_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typelib_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typelib_get_typeinfo_count(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        return 0;
    };
    lib.typeinfos.len() as u32
}

fn typelib_get_typeinfo(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let index = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if index >= lib.typeinfos.len() {
        let _ = vm.write_u32(out_ptr, 0);
        return E_INVALIDARG;
    }
    let Some(typeinfo_id) = typelib::store_typeinfo(lib_id, index) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        if let Some(info) = lib.typeinfos.get(index) {
            eprintln!(
                "[pe_vm] ITypeLib::GetTypeInfo index={index} guid={}",
                format_guid(&info.guid)
            );
        }
    }
    let typeinfo = build_typeinfo_object(vm, typeinfo_id).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, typeinfo);
    if typeinfo == 0 {
        return E_NOTIMPL;
    }
    S_OK
}

fn typelib_get_typeinfo_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let index = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    let kind = lib
        .typeinfos
        .get(index)
        .map(|info| info.typekind)
        .unwrap_or(0);
    let _ = vm.write_u32(out_ptr, kind);
    S_OK
}

fn typelib_get_typeinfo_of_guid(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let guid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let guid = match read_guid_bytes(vm, guid_ptr) {
        Some(value) => value,
        None => {
            let _ = vm.write_u32(out_ptr, 0);
            return E_INVALIDARG;
        }
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeLib::GetTypeInfoOfGuid guid={}",
            format_guid(&guid)
        );
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    let index = match lib.typeinfos.iter().position(|info| info.guid == guid) {
        Some(value) => value,
        None => {
            let _ = vm.write_u32(out_ptr, 0);
            return E_NOTIMPL;
        }
    };
    let Some(typeinfo_id) = typelib::store_typeinfo(lib_id, index) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        if let Some(info) = lib.typeinfos.get(index) {
            eprintln!(
                "[pe_vm] ITypeLib::GetTypeInfoOfGuid match index={index} guid={}",
                format_guid(&info.guid)
            );
        }
    }
    let typeinfo = build_typeinfo_object(vm, typeinfo_id).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, typeinfo);
    if typeinfo == 0 {
        return E_NOTIMPL;
    }
    S_OK
}

fn typelib_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// UnRegisterTypeLib(...)
pub(super) fn unregister_type_lib(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}
