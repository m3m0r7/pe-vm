//! Minimal ActiveX site hooks for in-proc COM controls.

use crate::vm::{Value, Vm, VmError};
use crate::vm::stdcall_args;
use crate::vm::windows::guid::parse_guid;

use super::instance::query_interface;
use super::super::object::vtable_fn;
use super::IID_IUNKNOWN;

const IID_IOLECLIENTSITE: &str = "{00000118-0000-0000-C000-000000000046}";
const IID_IOLEOBJECT: &str = "{00000112-0000-0000-C000-000000000046}";
const IID_IOLEINPLACESITE: &str = "{00000119-0000-0000-C000-000000000046}";
const IID_IOLEINPLACEFRAME: &str = "{00000116-0000-0000-C000-000000000046}";
const IID_IOLEINPLACEUIWINDOW: &str = "{00000115-0000-0000-C000-000000000046}";
const IID_IOLEWINDOW: &str = "{00000114-0000-0000-C000-000000000046}";

const S_OK: u32 = 0;
const E_NOINTERFACE: u32 = 0x8000_4002;
const E_NOTIMPL: u32 = 0x8000_4001;
const OLEIVERB_INPLACEACTIVATE: u32 = 0xFFFF_FFFB;

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
    let site_ptr = build_site_objects(vm)?;
    let set_client_site = vtable_fn(vm, ole_object, 3)?;
    let set_thiscall = detect_thiscall(vm, set_client_site);
    let hr = if set_thiscall {
        vm.execute_at_with_stack_with_ecx(
            set_client_site,
            ole_object,
            &[Value::U32(site_ptr)],
        )?
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
        eprintln!(
            "[pe_vm] IOleObject::DoVerb hr=0x{hr:08X} verb=0x{OLEIVERB_INPLACEACTIVATE:08X}"
        );
    }
    Ok(())
}

fn build_site_objects(vm: &mut Vm) -> Result<u32, VmError> {
    register_site_thunks(vm);
    let client_vtable = build_vtable(
        vm,
        &[
            "pe_vm.ioleclientsite.QueryInterface",
            "pe_vm.ioleclientsite.AddRef",
            "pe_vm.ioleclientsite.Release",
            "pe_vm.ioleclientsite.SaveObject",
            "pe_vm.ioleclientsite.GetMoniker",
            "pe_vm.ioleclientsite.GetContainer",
            "pe_vm.ioleclientsite.ShowObject",
            "pe_vm.ioleclientsite.OnShowWindow",
            "pe_vm.ioleclientsite.RequestNewObjectLayout",
        ],
    )?;
    let in_place_site_vtable = build_vtable(
        vm,
        &[
            "pe_vm.ioleinplacesite.QueryInterface",
            "pe_vm.ioleinplacesite.AddRef",
            "pe_vm.ioleinplacesite.Release",
            "pe_vm.ioleinplacesite.GetWindow",
            "pe_vm.ioleinplacesite.ContextSensitiveHelp",
            "pe_vm.ioleinplacesite.CanInPlaceActivate",
            "pe_vm.ioleinplacesite.OnInPlaceActivate",
            "pe_vm.ioleinplacesite.OnUIActivate",
            "pe_vm.ioleinplacesite.GetWindowContext",
            "pe_vm.ioleinplacesite.Scroll",
            "pe_vm.ioleinplacesite.OnUIDeactivate",
            "pe_vm.ioleinplacesite.OnInPlaceDeactivate",
            "pe_vm.ioleinplacesite.DiscardUndoState",
            "pe_vm.ioleinplacesite.DeactivateAndUndo",
            "pe_vm.ioleinplacesite.OnPosRectChange",
        ],
    )?;
    let in_place_ui_vtable = build_vtable(
        vm,
        &[
            "pe_vm.ioleinplaceuiwindow.QueryInterface",
            "pe_vm.ioleinplaceuiwindow.AddRef",
            "pe_vm.ioleinplaceuiwindow.Release",
            "pe_vm.ioleinplaceuiwindow.GetWindow",
            "pe_vm.ioleinplaceuiwindow.ContextSensitiveHelp",
            "pe_vm.ioleinplaceuiwindow.GetBorder",
            "pe_vm.ioleinplaceuiwindow.RequestBorderSpace",
            "pe_vm.ioleinplaceuiwindow.SetBorderSpace",
            "pe_vm.ioleinplaceuiwindow.SetActiveObject",
        ],
    )?;
    let in_place_frame_vtable = build_vtable(
        vm,
        &[
            "pe_vm.ioleinplaceframe.QueryInterface",
            "pe_vm.ioleinplaceframe.AddRef",
            "pe_vm.ioleinplaceframe.Release",
            "pe_vm.ioleinplaceframe.GetWindow",
            "pe_vm.ioleinplaceframe.ContextSensitiveHelp",
            "pe_vm.ioleinplaceframe.GetBorder",
            "pe_vm.ioleinplaceframe.RequestBorderSpace",
            "pe_vm.ioleinplaceframe.SetBorderSpace",
            "pe_vm.ioleinplaceframe.SetActiveObject",
            "pe_vm.ioleinplaceframe.InsertMenus",
            "pe_vm.ioleinplaceframe.SetMenu",
            "pe_vm.ioleinplaceframe.RemoveMenus",
            "pe_vm.ioleinplaceframe.SetStatusText",
            "pe_vm.ioleinplaceframe.EnableModeless",
            "pe_vm.ioleinplaceframe.TranslateAccelerator",
        ],
    )?;

    let in_place_frame = build_object(vm, in_place_frame_vtable, &[])?;
    let in_place_ui = build_object(vm, in_place_ui_vtable, &[])?;
    let in_place_site = build_object(
        vm,
        in_place_site_vtable,
        &[0, in_place_frame, in_place_ui],
    )?;
    let client_site = build_object(
        vm,
        client_vtable,
        &[in_place_site, in_place_frame, in_place_ui],
    )?;

    let _ = vm.write_u32(in_place_site + 4, client_site);

    Ok(client_site)
}

fn build_vtable(vm: &mut Vm, entries: &[&str]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(entries.len() * 4);
    for name in entries {
        let entry = resolve_site_entry(vm, name)?;
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

fn resolve_site_entry(vm: &mut Vm, name: &str) -> Result<u32, VmError> {
    vm.resolve_dynamic_import(name)
        .ok_or(VmError::InvalidConfig("missing import"))
}

fn register_site_thunks(vm: &mut Vm) {
    // IOleClientSite
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.QueryInterface",
        stdcall_args(3),
        site_query_interface,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.AddRef",
        stdcall_args(1),
        site_add_ref,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.Release",
        stdcall_args(1),
        site_release,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.SaveObject",
        stdcall_args(1),
        site_save_object,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.GetMoniker",
        stdcall_args(4),
        site_get_moniker,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.GetContainer",
        stdcall_args(2),
        site_get_container,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.ShowObject",
        stdcall_args(1),
        site_show_object,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.OnShowWindow",
        stdcall_args(2),
        site_on_show_window,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleclientsite.RequestNewObjectLayout",
        stdcall_args(1),
        site_request_new_object_layout,
    );

    // IOleInPlaceSite
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.QueryInterface",
        stdcall_args(3),
        in_place_site_query_interface,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.AddRef",
        stdcall_args(1),
        site_add_ref,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.Release",
        stdcall_args(1),
        site_release,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.GetWindow",
        stdcall_args(2),
        ole_get_window,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.ContextSensitiveHelp",
        stdcall_args(2),
        ole_context_sensitive_help,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.CanInPlaceActivate",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.OnInPlaceActivate",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.OnUIActivate",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.GetWindowContext",
        stdcall_args(6),
        in_place_site_get_window_context,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.Scroll",
        stdcall_args(3),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.OnUIDeactivate",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.OnInPlaceDeactivate",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.DiscardUndoState",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.DeactivateAndUndo",
        stdcall_args(1),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplacesite.OnPosRectChange",
        stdcall_args(2),
        ole_simple_ok,
    );

    // IOleInPlaceUIWindow
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.QueryInterface",
        stdcall_args(3),
        in_place_ui_query_interface,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.AddRef",
        stdcall_args(1),
        site_add_ref,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.Release",
        stdcall_args(1),
        site_release,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.GetWindow",
        stdcall_args(2),
        ole_get_window,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.ContextSensitiveHelp",
        stdcall_args(2),
        ole_context_sensitive_help,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.GetBorder",
        stdcall_args(2),
        ole_get_border,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.RequestBorderSpace",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.SetBorderSpace",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceuiwindow.SetActiveObject",
        stdcall_args(3),
        ole_simple_ok,
    );

    // IOleInPlaceFrame
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.QueryInterface",
        stdcall_args(3),
        in_place_frame_query_interface,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.AddRef",
        stdcall_args(1),
        site_add_ref,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.Release",
        stdcall_args(1),
        site_release,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.GetWindow",
        stdcall_args(2),
        ole_get_window,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.ContextSensitiveHelp",
        stdcall_args(2),
        ole_context_sensitive_help,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.GetBorder",
        stdcall_args(2),
        ole_get_border,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.RequestBorderSpace",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.SetBorderSpace",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.SetActiveObject",
        stdcall_args(3),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.InsertMenus",
        stdcall_args(3),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.SetMenu",
        stdcall_args(4),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.RemoveMenus",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.SetStatusText",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.EnableModeless",
        stdcall_args(2),
        ole_simple_ok,
    );
    vm.register_import_any_stdcall(
        "pe_vm.ioleinplaceframe.TranslateAccelerator",
        stdcall_args(3),
        ole_translate_accelerator,
    );
}

fn site_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
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

fn in_place_site_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
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

fn in_place_ui_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
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

fn in_place_frame_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
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

fn site_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn site_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn site_save_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn site_get_moniker(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn site_get_container(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    E_NOINTERFACE
}

fn site_show_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn site_on_show_window(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn site_request_new_object_layout(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn ole_get_window(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

fn ole_context_sensitive_help(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn in_place_site_get_window_context(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let frame_out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let doc_out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let pos_rect = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let clip_rect = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let frame_info = vm.read_u32(stack_ptr + 24).unwrap_or(0);

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

fn ole_get_border(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let _ = write_rect(vm, out_ptr);
    S_OK
}

fn ole_simple_ok(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn ole_translate_accelerator(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn write_rect(vm: &mut Vm, ptr: u32) -> Result<(), VmError> {
    if ptr == 0 {
        return Ok(());
    }
    vm.write_u32(ptr, 0)?;
    vm.write_u32(ptr + 4, 0)?;
    vm.write_u32(ptr + 8, 0)?;
    vm.write_u32(ptr + 12, 0)?;
    Ok(())
}

fn write_frame_info(vm: &mut Vm, ptr: u32) -> Result<(), VmError> {
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

fn detect_thiscall(vm: &Vm, entry: u32) -> bool {
    let mut bytes = [0u8; 96];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(entry.wrapping_add(idx as u32)).unwrap_or(0);
    }

    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x44 && bytes[idx + 2] == 0x24 && bytes[idx + 3] == 0x04 {
            return false;
        }
    }
    for idx in 0..bytes.len().saturating_sub(2) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x45 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x75 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x4D && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x55 && bytes[idx + 2] == 0x08 {
            return false;
        }
    }

    for idx in 0..bytes.len().saturating_sub(1) {
        let opcode = bytes[idx];
        if !matches!(opcode, 0x8B | 0x89 | 0x8A | 0x8D) {
            continue;
        }
        let modrm = bytes[idx + 1];
        let mod_bits = modrm & 0xC0;
        let rm = modrm & 0x07;
        if mod_bits != 0xC0 && rm == 0x01 {
            return true;
        }
    }
    false
}
