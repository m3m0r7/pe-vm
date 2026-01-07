//! ActiveX site interface handlers.

use crate::vm::windows::guid::parse_guid;
use crate::vm::{Vm, VmError};
use crate::vm_args;

use super::super::IID_IUNKNOWN;
use super::constants::{
    E_NOINTERFACE, E_NOTIMPL, IID_IOLECLIENTSITE, IID_IOLEINPLACEFRAME, IID_IOLEINPLACESITE,
    IID_IOLEINPLACEUIWINDOW, IID_IOLEWINDOW, S_OK,
};

pub(super) fn site_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let in_place_site = read_ptr(vm, this, 4);
    let in_place_frame = read_ptr(vm, this, 8);
    let in_place_ui = read_ptr(vm, this, 12);
    if guid_matches(vm, iid_ptr, IID_IUNKNOWN) || guid_matches(vm, iid_ptr, IID_IOLECLIENTSITE) {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLEINPLACESITE) {
        let _ = vm.write_u32(out_ptr, in_place_site);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLEINPLACEFRAME) {
        let _ = vm.write_u32(out_ptr, in_place_frame);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLEINPLACEUIWINDOW) {
        let _ = vm.write_u32(out_ptr, in_place_ui);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

pub(super) fn in_place_site_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let client_site = read_ptr(vm, this, 4);
    let in_place_frame = read_ptr(vm, this, 8);
    let in_place_ui = read_ptr(vm, this, 12);
    if guid_matches(vm, iid_ptr, IID_IUNKNOWN) || guid_matches(vm, iid_ptr, IID_IOLEINPLACESITE) {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLECLIENTSITE) {
        let _ = vm.write_u32(out_ptr, client_site);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLEINPLACEFRAME) {
        let _ = vm.write_u32(out_ptr, in_place_frame);
        return S_OK;
    }
    if guid_matches(vm, iid_ptr, IID_IOLEINPLACEUIWINDOW) {
        let _ = vm.write_u32(out_ptr, in_place_ui);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

pub(super) fn in_place_ui_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    if guid_matches(vm, iid_ptr, IID_IUNKNOWN)
        || guid_matches(vm, iid_ptr, IID_IOLEINPLACEUIWINDOW)
        || guid_matches(vm, iid_ptr, IID_IOLEWINDOW)
    {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

pub(super) fn in_place_frame_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    if guid_matches(vm, iid_ptr, IID_IUNKNOWN)
        || guid_matches(vm, iid_ptr, IID_IOLEINPLACEFRAME)
        || guid_matches(vm, iid_ptr, IID_IOLEINPLACEUIWINDOW)
        || guid_matches(vm, iid_ptr, IID_IOLEWINDOW)
    {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

pub(super) fn site_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn site_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

pub(super) fn site_save_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn site_get_moniker(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

pub(super) fn site_get_container(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    E_NOINTERFACE
}

pub(super) fn site_show_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn site_on_show_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn site_request_new_object_layout(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

pub(super) fn ole_get_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

pub(super) fn ole_context_sensitive_help(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn in_place_site_get_window_context(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, frame_out, doc_out, pos_rect, clip_rect, frame_info) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);

    let in_place_frame = read_ptr(vm, this, 8);
    let in_place_ui = read_ptr(vm, this, 12);
    if frame_out != 0 {
        let _ = vm.write_u32(frame_out, in_place_frame);
    }
    if doc_out != 0 {
        let _ = vm.write_u32(doc_out, in_place_ui);
    }
    let _ = write_rect(vm, pos_rect);
    let _ = write_rect(vm, clip_rect);
    let _ = write_frame_info(vm, frame_info);
    S_OK
}

pub(super) fn ole_get_border(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    let _ = write_rect(vm, out_ptr);
    S_OK
}

pub(super) fn ole_simple_ok(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

pub(super) fn ole_translate_accelerator(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

pub(super) fn write_rect(vm: &mut Vm, ptr: u32) -> Result<(), VmError> {
    if ptr == 0 {
        return Ok(());
    }
    vm.write_u32(ptr, 0)?;
    vm.write_u32(ptr + 4, 0)?;
    vm.write_u32(ptr + 8, 0)?;
    vm.write_u32(ptr + 12, 0)?;
    Ok(())
}

pub(super) fn write_frame_info(vm: &mut Vm, ptr: u32) -> Result<(), VmError> {
    if ptr == 0 {
        return Ok(());
    }
    vm.write_u32(ptr, 20)?; // cb size
    vm.write_u32(ptr + 4, 0)?; // fMDIApp
    vm.write_u32(ptr + 8, 0)?; // hwndFrame
    vm.write_u32(ptr + 12, 0)?; // haccel
    vm.write_u32(ptr + 16, 0)?; // cAccelEntries
    Ok(())
}

fn read_ptr(vm: &Vm, base: u32, offset: u32) -> u32 {
    vm.read_u32(base.wrapping_add(offset)).unwrap_or(0)
}

fn guid_matches(vm: &Vm, ptr: u32, guid: &str) -> bool {
    let Some(expected) = parse_guid(guid) else {
        return false;
    };
    let mut actual = [0u8; 16];
    for (idx, slot) in actual.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr.wrapping_add(idx as u32)).unwrap_or(0);
    }
    actual == expected
}
