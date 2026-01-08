//! COM object instantiation and IDispatch negotiation.

use crate::pe::PeFile;
use crate::vm::windows::macros::read_wide_or_utf16le_str;
use crate::vm::{Value, Vm, VmError};

use super::super::object::{vtable_fn, InProcObject};
use super::helpers::{alloc_guid, read_hex_window};
use super::scan::recover_dispatch_from_heap;
use super::{IID_ICLASSFACTORY, IID_IDISPATCH, IID_IUNKNOWN};
use crate::vm::windows::oleaut32::typelib::TypeLib;

// Instantiate a COM object by calling DllGetClassObject and IClassFactory::CreateInstance.
pub(super) fn create_inproc_object(
    vm: &mut Vm,
    file: &PeFile,
    clsid: &str,
    typelib: Option<TypeLib>,
) -> Result<InProcObject, VmError> {
    let entry = file
        .export_rva("DllGetClassObject")
        .ok_or_else(|| VmError::MissingExport("DllGetClassObject".to_string()))?;
    let entry = vm.base().wrapping_add(entry);

    let clsid_ptr = alloc_guid(vm, clsid)?;
    let iid_factory = alloc_guid(vm, IID_ICLASSFACTORY)?;
    let factory_out = vm.alloc_bytes(&[0u8; 4], 4)?;

    let hr = vm.execute_at_with_stack(
        entry,
        &[
            Value::U32(clsid_ptr),
            Value::U32(iid_factory),
            Value::U32(factory_out),
        ],
    )?;
    if std::env::var("PE_VM_TRACE").is_ok() {
        let out = vm.read_u32(factory_out).unwrap_or(0);
        eprintln!("[pe_vm] DllGetClassObject hr=0x{hr:08X} out=0x{out:08X}");
    }
    if hr != 0 {
        return Err(VmError::Com(hr));
    }
    let class_factory = vm.read_u32(factory_out)?;
    if class_factory == 0 {
        return Err(VmError::InvalidConfig("class factory is null"));
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        let vtable_ptr = vm.read_u32(class_factory).unwrap_or(0);
        let qi = vm.read_u32(vtable_ptr).unwrap_or(0);
        let add_ref = vm.read_u32(vtable_ptr.wrapping_add(4)).unwrap_or(0);
        let release = vm.read_u32(vtable_ptr.wrapping_add(8)).unwrap_or(0);
        let create = vm.read_u32(vtable_ptr.wrapping_add(12)).unwrap_or(0);
        let internal_create = vm.read_u32(class_factory.wrapping_add(0x24)).unwrap_or(0);
        eprintln!(
            "[pe_vm] class_factory=0x{class_factory:08X} vtable=0x{vtable_ptr:08X} qi=0x{qi:08X} addref=0x{add_ref:08X} release=0x{release:08X} create=0x{create:08X} internal_create=0x{internal_create:08X}"
        );
    }

    let create_instance = vtable_fn(vm, class_factory, 3)?;
    let create_instance_thiscall = detect_thiscall(vm, create_instance);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] IClassFactory::CreateInstance at 0x{create_instance:08X} thiscall={create_instance_thiscall}"
        );
    }
    let internal_create = vm.read_u32(class_factory.wrapping_add(0x24)).unwrap_or(0);

    let candidates = [IID_IDISPATCH, IID_IUNKNOWN];
    let mut i_dispatch = 0u32;
    let mut last_hr = 0u32;
    let mut selected = None;
    for iid in candidates {
        let (hr, out) = create_instance_with_iid(
            vm,
            create_instance,
            class_factory,
            iid,
            create_instance_thiscall,
        )?;
        last_hr = hr;
        if hr == 0 && out != 0 {
            i_dispatch = out;
            selected = Some(iid);
            break;
        }
    }
    if selected == Some(IID_IUNKNOWN) {
        let out = query_interface(vm, i_dispatch, IID_IDISPATCH)?;
        if out != 0 {
            i_dispatch = out;
        }
    }
    if i_dispatch == 0 && last_hr == 0 {
        if let Some(recovered) = recover_dispatch_from_heap(vm, file, internal_create) {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] recovered IDispatch pointer 0x{recovered:08X} from heap scan");
            }
            i_dispatch = recovered;
        }
    }
    if i_dispatch == 0 {
        if last_hr != 0 {
            return Err(VmError::Com(last_hr));
        }
        return Err(VmError::InvalidConfig("IDispatch is null"));
    }
    Ok(InProcObject::new(i_dispatch, typelib))
}

fn create_instance_with_iid(
    vm: &mut Vm,
    create_instance: u32,
    class_factory: u32,
    iid: &str,
    thiscall: bool,
) -> Result<(u32, u32), VmError> {
    let iid_ptr = alloc_guid(vm, iid)?;
    let out_ptr = vm.alloc_bytes(&[0u8; 4], 4)?;
    if std::env::var("PE_VM_TRACE").is_ok() {
        let mut bytes = [0u8; 16];
        for (idx, b) in bytes.iter_mut().enumerate() {
            *b = vm.read_u8(iid_ptr.wrapping_add(idx as u32)).unwrap_or(0);
        }
        if iid == IID_IDISPATCH {
            let snapshot = read_hex_window(vm, class_factory, 64);
            eprintln!("[pe_vm] CreateInstance class_factory bytes before: {snapshot}");
        }
        let before = vm.read_u32(out_ptr).unwrap_or(0);
        let internal_create = vm.read_u32(class_factory.wrapping_add(0x24)).unwrap_or(0);
        eprintln!(
            "[pe_vm] CreateInstance prep iid={iid} iid_ptr=0x{iid_ptr:08X} bytes={:02X?} out_ptr=0x{out_ptr:08X} before=0x{before:08X} internal_create=0x{internal_create:08X} thiscall={thiscall}",
            bytes,
        );
    }
    let hr = if thiscall {
        vm.execute_at_with_stack_with_ecx(
            create_instance,
            class_factory,
            &[Value::U32(0), Value::U32(iid_ptr), Value::U32(out_ptr)],
        )?
    } else {
        vm.execute_at_with_stack(
            create_instance,
            &[
                Value::U32(class_factory),
                Value::U32(0),
                Value::U32(iid_ptr),
                Value::U32(out_ptr),
            ],
        )?
    };
    let out = vm.read_u32(out_ptr).unwrap_or(0);
    if std::env::var("PE_VM_TRACE").is_ok() {
        let after_internal = vm.read_u32(class_factory.wrapping_add(0x24)).unwrap_or(0);
        let delta = out_ptr.wrapping_sub(class_factory);
        if iid == IID_IDISPATCH {
            let snapshot = read_hex_window(vm, class_factory, 64);
            eprintln!("[pe_vm] CreateInstance class_factory bytes after: {snapshot}");
        }
        eprintln!(
            "[pe_vm] CreateInstance iid={iid} hr=0x{hr:08X} out=0x{out:08X} internal_create_after=0x{after_internal:08X} out_ptr_delta=0x{delta:08X}"
        );
        if vm.contains_addr(hr) {
            let mut line = String::from("[pe_vm] CreateInstance ptr?");
            for offset in [0u32, 4, 8, 12, 16, 20, 24, 32, 36, 40, 44, 48, 52, 56, 60] {
                let value = vm.read_u32(hr.wrapping_add(offset)).unwrap_or(0);
                line.push_str(&format!(" +0x{offset:02X}=0x{value:08X}"));
            }
            eprintln!("{line}");
            let text = read_wide_or_utf16le_str(vm, hr.wrapping_add(0x20));
            if !text.is_empty() {
                eprintln!("[pe_vm] CreateInstance text: {text}");
            }
        }
    }
    Ok((hr, out))
}

fn detect_thiscall(vm: &Vm, entry: u32) -> bool {
    let mut bytes = [0u8; 96];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(entry.wrapping_add(idx as u32)).unwrap_or(0);
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        let hex = bytes
            .iter()
            .map(|value| format!("{value:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[pe_vm] CreateInstance bytes: {hex}");
    }

    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx] == 0x8B && bytes[idx + 2] == 0x24 && bytes[idx + 3] == 0x04 {
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
    }

    for idx in 0..bytes.len().saturating_sub(1) {
        if bytes[idx] != 0x8B {
            continue;
        }
        let modrm = bytes[idx + 1];
        if matches!(modrm, 0xF1 | 0xF9 | 0xD9 | 0xC1 | 0xC9) {
            return true;
        }
    }
    false
}

pub(super) fn query_interface(vm: &mut Vm, obj_ptr: u32, iid: &str) -> Result<u32, VmError> {
    let query = vtable_fn(vm, obj_ptr, 0)?;
    let iid_ptr = alloc_guid(vm, iid)?;
    let out_ptr = vm.alloc_bytes(&[0u8; 4], 4)?;
    let hr = vm.execute_at_with_stack(
        query,
        &[
            Value::U32(obj_ptr),
            Value::U32(iid_ptr),
            Value::U32(out_ptr),
        ],
    )?;
    if hr != 0 {
        return Err(VmError::Com(hr));
    }
    vm.read_u32(out_ptr)
}
