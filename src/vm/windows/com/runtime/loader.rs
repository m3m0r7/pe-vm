//! COM loader helpers for resolving registry paths and initializing DLLs.

use crate::pe::PeFile;
use crate::vm::{Value, Vm, VmError};
use crate::vm::windows::registry::RegistryValue;
use crate::vm::windows::{get_registry, registry::Registry};

use super::helpers::normalize_clsid;

pub(super) fn resolve_inproc_path(vm: &mut Vm, clsid: &str) -> Result<(String, String, String), VmError> {
    let normalized = normalize_clsid(clsid);
    let registry =
        get_registry(vm).ok_or(VmError::InvalidConfig("windows registry unavailable"))?;
    let dll_path = resolve_registry_path(registry, &normalized)?
        .ok_or(VmError::InvalidConfig("missing InprocServer32 value"))?;
    let host_path = vm.map_path(&dll_path);
    Ok((normalized, dll_path, host_path))
}

pub(super) fn register_server(vm: &mut Vm, file: &PeFile) -> Result<(), VmError> {
    if std::env::var("PE_VM_REGISTER_SERVER").is_err() {
        return Ok(());
    }
    let Some(rva) = file.export_rva("DllRegisterServer") else {
        return Ok(());
    };
    let entry = vm.base().wrapping_add(rva);
    let _ = vm.execute_at_with_stack(entry, &[])?;
    Ok(())
}

pub(super) fn init_dll(vm: &mut Vm, file: &PeFile) -> Result<(), VmError> {
    let entry_rva = file.optional_header.address_of_entry_point;
    if entry_rva == 0 {
        return Ok(());
    }
    let entry = vm.base().wrapping_add(entry_rva);
    let result = vm.execute_at_with_stack(
        entry,
        &[Value::U32(vm.base()), Value::U32(1), Value::U32(0)],
    )?;
    if result == 0 {
        return Err(VmError::InvalidConfig("DllMain returned failure"));
    }
    Ok(())
}

fn resolve_registry_path(registry: &Registry, normalized: &str) -> Result<Option<String>, VmError> {
    let candidates = [
        format!(r"HKCR\CLSID\{}\InprocServer32", normalized),
        format!(r"HKLM\Software\Classes\CLSID\{}\InprocServer32", normalized),
        format!(r"HKCU\Software\Classes\CLSID\{}\InprocServer32", normalized),
    ];
    for key in candidates {
        if let Some(value) = resolve_registry_value(registry, &key)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn resolve_registry_value(registry: &Registry, key: &str) -> Result<Option<String>, VmError> {
    let value = registry
        .get(key)
        .map_err(|_| VmError::InvalidConfig("registry key"))?;
    let Some(RegistryValue::String(value)) = value else {
        return Ok(None);
    };
    Ok(Some(value.to_string()))
}
