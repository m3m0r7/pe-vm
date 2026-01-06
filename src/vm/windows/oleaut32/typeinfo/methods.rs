//! ITypeInfo method stubs except Invoke.

use crate::vm::Vm;

use crate::vm::windows::oleaut32::typelib;
use crate::vm::windows::guid::format_guid;

use super::helpers::{resolve_typeinfo_info, resolve_typeinfo_this};
use super::super::constants::{E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, IID_ITYPEINFO, IID_ITYPEINFO2, IID_IUNKNOWN, S_OK};
use super::super::guid::guid_matches;

pub(super) fn typeinfo_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, thiscall) = resolve_typeinfo_this(vm, stack_ptr).unwrap_or((0, false));
    let iid_ptr = vm.read_u32(stack_ptr + if thiscall { 4 } else { 8 }).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + if thiscall { 8 } else { 12 }).unwrap_or(0);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let ok = guid_matches(vm, iid_ptr, IID_IUNKNOWN)
        || guid_matches(vm, iid_ptr, IID_ITYPEINFO)
        || guid_matches(vm, iid_ptr, IID_ITYPEINFO2);
    if ok {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

pub(super) fn typeinfo_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn typeinfo_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn typeinfo_get_type_attr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return E_NOTIMPL;
    };
    let out_ptr = vm.read_u32(stack_ptr + if thiscall { 4 } else { 8 }).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let Some(info) = typelib::get_typeinfo(info_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };

    let mut bytes = vec![0u8; 0x4C];
    bytes[0..16].copy_from_slice(&info.guid);
    bytes[24..28].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    bytes[28..32].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    bytes[40..44].copy_from_slice(&info.typekind.to_le_bytes());
    bytes[44..46].copy_from_slice(&info.c_funcs.to_le_bytes());
    bytes[46..48].copy_from_slice(&info.c_vars.to_le_bytes());
    bytes[48..50].copy_from_slice(&info.c_impl_types.to_le_bytes());
    bytes[50..52].copy_from_slice(&info.cb_size_vft.to_le_bytes());
    bytes[54..56].copy_from_slice(&(info.flags as u16).to_le_bytes());
    let attr_ptr = vm.alloc_bytes(&bytes, 4).unwrap_or(0);
    if attr_ptr == 0 {
        return E_NOTIMPL;
    }
    let _ = vm.write_u32(out_ptr, attr_ptr);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::GetTypeAttr guid={} kind=0x{:X}",
            format_guid(&info.guid),
            info.typekind
        );
    }
    S_OK
}

pub(super) fn typeinfo_release_type_attr(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn typeinfo_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}
