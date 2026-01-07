//! Minimal ActiveX site hooks for in-proc COM controls.

mod builder;
mod constants;
mod handlers;
mod thunks;
mod utils;

use crate::vm::{Value, Vm, VmError};

use super::super::object::vtable_fn;
use super::instance::query_interface;
use constants::{
    E_NOINTERFACE, E_NOTIMPL, IID_IOLEOBJECT, IID_IPERSISTSTREAMINIT, OLEIVERB_INPLACEACTIVATE,
};
use utils::detect_thiscall;

// Attach a stub IOleClientSite to an in-proc COM object if it supports IOleObject.
pub(super) fn attach_client_site(vm: &mut Vm, i_dispatch: u32) -> Result<(), VmError> {
    let ole_object = match query_interface(vm, i_dispatch, IID_IOLEOBJECT) {
        Ok(ptr) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] IOleObject QueryInterface ok=0x{ptr:08X} from IDispatch=0x{i_dispatch:08X}"
                );
            }
            ptr
        }
        Err(VmError::Com(code)) if code == E_NOINTERFACE => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] IOleObject QueryInterface not supported for IDispatch=0x{i_dispatch:08X}"
                );
            }
            return Ok(());
        }
        Err(err) => return Err(err),
    };
    if ole_object == 0 {
        return Ok(());
    }
    let site_ptr = builder::build_site_objects(vm)?;
    let set_client_site = vtable_fn(vm, ole_object, 3)?;
    let set_thiscall = detect_thiscall(vm, set_client_site);
    let hr = if set_thiscall {
        vm.execute_at_with_stack_with_ecx(set_client_site, ole_object, &[Value::U32(site_ptr)])?
    } else {
        vm.execute_at_with_stack(
            set_client_site,
            &[Value::U32(ole_object), Value::U32(site_ptr)],
        )?
    };
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] IOleObject::SetClientSite hr=0x{hr:08X} site=0x{:08X}",
            site_ptr
        );
    }
    if hr == E_NOTIMPL {
        return Ok(());
    }
    if hr != 0 {
        return Err(VmError::Com(hr));
    }
    init_persist_stream(vm, i_dispatch)?;
    let do_verb = vtable_fn(vm, ole_object, 11)?;
    let do_thiscall = detect_thiscall(vm, do_verb);
    let hr = if do_thiscall {
        vm.execute_at_with_stack_with_ecx(
            do_verb,
            ole_object,
            &[
                Value::U32(OLEIVERB_INPLACEACTIVATE),
                Value::U32(0),
                Value::U32(site_ptr),
                Value::U32(0),
                Value::U32(0),
                Value::U32(0),
            ],
        )?
    } else {
        vm.execute_at_with_stack(
            do_verb,
            &[
                Value::U32(ole_object),
                Value::U32(OLEIVERB_INPLACEACTIVATE),
                Value::U32(0),
                Value::U32(site_ptr),
                Value::U32(0),
                Value::U32(0),
                Value::U32(0),
            ],
        )?
    };
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] IOleObject::DoVerb hr=0x{hr:08X} verb=0x{OLEIVERB_INPLACEACTIVATE:08X}");
    }
    Ok(())
}

fn init_persist_stream(vm: &mut Vm, i_dispatch: u32) -> Result<(), VmError> {
    let persist = match query_interface(vm, i_dispatch, IID_IPERSISTSTREAMINIT) {
        Ok(ptr) => ptr,
        Err(VmError::Com(code)) if code == E_NOINTERFACE => return Ok(()),
        Err(err) => return Err(err),
    };
    if persist == 0 {
        return Ok(());
    }
    let init_new = vtable_fn(vm, persist, 8)?;
    let init_thiscall = detect_thiscall(vm, init_new);
    let hr = if init_thiscall {
        vm.execute_at_with_stack_with_ecx(init_new, persist, &[])?
    } else {
        vm.execute_at_with_stack(init_new, &[Value::U32(persist)])?
    };
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] IPersistStreamInit::InitNew hr=0x{hr:08X}");
    }
    if hr == E_NOTIMPL {
        return Ok(());
    }
    if hr != 0 {
        return Err(VmError::Com(hr));
    }
    Ok(())
}
