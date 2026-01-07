//! ITypeInfo method stubs except Invoke.

use crate::vm::Vm;
use crate::vm_args;

use crate::vm::windows::oleaut32::typelib;
use crate::vm::windows::guid::format_guid;

use super::helpers::{resolve_typeinfo_info, resolve_typeinfo_this};
use super::super::constants::{E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, IID_ITYPEINFO, IID_ITYPEINFO2, IID_IUNKNOWN, S_OK};
use super::super::guid::guid_matches;

pub(super) fn typeinfo_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, thiscall) = resolve_typeinfo_this(vm, stack_ptr).unwrap_or((0, false));
    let (iid_ptr, out_ptr) = if thiscall {
        vm_args!(vm, stack_ptr; u32, u32)
    } else {
        let (_, iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
        (iid_ptr, out_ptr)
    };
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
    let out_ptr = if thiscall {
        let (out_ptr,) = vm_args!(vm, stack_ptr; u32);
        out_ptr
    } else {
        let (_, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
        out_ptr
    };
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

pub(super) fn typeinfo_get_func_desc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return E_NOTIMPL;
    };
    let (index, out_ptr) = if thiscall {
        vm_args!(vm, stack_ptr; u32, u32)
    } else {
        let (_, index, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
        (index, out_ptr)
    };
    let index = index as usize;
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let Some(info) = typelib::get_typeinfo(info_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    let Some(func) = info.funcs.get(index) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_INVALIDARG;
    };
    let params_ptr = if func.params.is_empty() {
        0
    } else {
        let mut bytes = vec![0u8; func.params.len() * ELEMDESC_SIZE];
        for (idx, param) in func.params.iter().enumerate() {
            let offset = idx * ELEMDESC_SIZE;
            write_elemdesc(&mut bytes, offset, param.vt, param.flags as u16);
        }
        vm.alloc_bytes(&bytes, 4).unwrap_or(0)
    };

    let mut func_bytes = vec![0u8; FUNCDESC_SIZE];
    func_bytes[0..4].copy_from_slice(&func.memid.to_le_bytes());
    func_bytes[4..8].copy_from_slice(&0u32.to_le_bytes()); // lprgscode
    func_bytes[8..12].copy_from_slice(&params_ptr.to_le_bytes());
    func_bytes[12..16].copy_from_slice(&0u32.to_le_bytes()); // FUNC_VIRTUAL
    func_bytes[16..20].copy_from_slice(&(func.invkind as u32).to_le_bytes());
    func_bytes[20..24].copy_from_slice(&(func.callconv as u32).to_le_bytes());
    func_bytes[24..26].copy_from_slice(&(func.params.len() as u16).to_le_bytes());
    func_bytes[26..28].copy_from_slice(&0u16.to_le_bytes()); // cParamsOpt
    func_bytes[28..30].copy_from_slice(&func.vtable_offset.to_le_bytes());
    func_bytes[30..32].copy_from_slice(&0u16.to_le_bytes()); // cScodes
    write_elemdesc(&mut func_bytes, 32, func.ret_vt, 0);
    func_bytes[48..50].copy_from_slice(&0u16.to_le_bytes()); // wFuncFlags

    let func_ptr = vm.alloc_bytes(&func_bytes, 4).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, func_ptr);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::GetFuncDesc index={index} memid=0x{:08X} params={} vtable=0x{:04X}",
            func.memid,
            func.params.len(),
            func.vtable_offset
        );
    }
    S_OK
}

pub(super) fn typeinfo_release_func_desc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn typeinfo_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

const FUNCDESC_SIZE: usize = 0x34;
const ELEMDESC_SIZE: usize = 0x10;

fn write_elemdesc(bytes: &mut [u8], offset: usize, vt: u16, flags: u16) {
    let end = offset + ELEMDESC_SIZE;
    if end > bytes.len() {
        return;
    }
    bytes[offset..offset + 4].copy_from_slice(&0u32.to_le_bytes());
    bytes[offset + 4..offset + 6].copy_from_slice(&vt.to_le_bytes());
    bytes[offset + 6..offset + 8].copy_from_slice(&0u16.to_le_bytes());
    bytes[offset + 8..offset + 12].copy_from_slice(&0u32.to_le_bytes());
    bytes[offset + 12..offset + 14].copy_from_slice(&flags.to_le_bytes());
    bytes[offset + 14..offset + 16].copy_from_slice(&0u16.to_le_bytes());
}
