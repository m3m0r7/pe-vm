//! ActiveX site object builders.

use crate::vm::{Vm, VmError};

use super::thunks::register_site_thunks;

pub(super) fn build_site_objects(vm: &mut Vm) -> Result<u32, VmError> {
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
    let in_place_site = build_object(vm, in_place_site_vtable, &[0, in_place_frame, in_place_ui])?;
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
