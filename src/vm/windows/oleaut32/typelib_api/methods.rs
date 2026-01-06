use crate::vm::Vm;
use crate::vm::windows::guid::format_guid;

use super::super::constants::{
    E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, IID_ITYPELIB, IID_IUNKNOWN, S_OK,
};
use super::super::guid::{guid_matches, read_guid_bytes};
use super::super::typelib;
use super::object::build_typeinfo_object;

pub(super) fn typelib_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
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

pub(super) fn typelib_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn typelib_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn typelib_get_typeinfo_count(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        return 0;
    };
    lib.typeinfos.len() as u32
}

pub(super) fn typelib_get_typeinfo(vm: &mut Vm, stack_ptr: u32) -> u32 {
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

pub(super) fn typelib_get_typeinfo_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
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

pub(super) fn typelib_get_typeinfo_of_guid(vm: &mut Vm, stack_ptr: u32) -> u32 {
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

pub(super) fn typelib_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}
