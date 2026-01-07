//! ITypeLib stubs and registration helpers.

mod loader;
mod methods;
mod object;

use crate::vm::Vm;
use crate::vm_args;
use crate::vm::windows::guid::format_guid;

use super::bstr::read_utf16_z;
use super::constants::{
    E_INVALIDARG, E_NOTIMPL, OleMethod, S_OK, TYPE_E_LIBNOTREGISTERED,
};
use super::guid::read_guid_bytes;

use loader::{load_typelib_from_path, resolve_typelib_path};
use methods::{
    typelib_add_ref, typelib_get_typeinfo, typelib_get_typeinfo_count, typelib_get_typeinfo_of_guid,
    typelib_get_typeinfo_type, typelib_not_impl, typelib_query_interface, typelib_release,
};

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
    let (_, _, out) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    S_OK
}

// LoadTypeLib(...)
pub(super) fn load_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr, out) = vm_args!(vm, stack_ptr; u32, u32);
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
    let (guid_ptr, major, minor, _, out) = vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);
    let major = major as u16;
    let minor = minor as u16;
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

// UnRegisterTypeLib(...)
pub(super) fn unregister_type_lib(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}
