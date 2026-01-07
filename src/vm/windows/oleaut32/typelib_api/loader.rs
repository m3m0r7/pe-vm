use crate::vm::windows::get_registry;
use crate::vm::windows::guid::format_guid;
use crate::vm::windows::registry::RegistryValue;
use crate::vm::{Vm, VmError};

use super::super::typelib;
use super::object::alloc_typelib;

pub(super) fn load_typelib_from_path(
    vm: &mut Vm,
    path: &str,
    expected_guid: Option<[u8; 16]>,
) -> Result<u32, VmError> {
    let host_path = vm.map_path(path);
    let bytes = std::fs::read(host_path)?;
    let lib = match typelib::load_from_bytes(&bytes) {
        Ok(lib) => lib,
        Err(err) => {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!("[pe_vm] TypeLib load failed: {err}");
            }
            return Err(err);
        }
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] TypeLib loaded guid={}", format_guid(&lib.guid));
    }
    if let Some(expected) = expected_guid {
        if lib.guid != expected {
            return Err(VmError::InvalidConfig("typelib guid mismatch"));
        }
    }
    alloc_typelib(vm, lib)
}

pub(super) fn resolve_typelib_path(
    vm: &Vm,
    guid: &str,
    major: u16,
    minor: u16,
) -> Result<Option<String>, VmError> {
    let registry =
        get_registry(vm).ok_or(VmError::InvalidConfig("windows registry unavailable"))?;
    let version = format!("{major}.{minor}");
    let candidates = [
        format!(r"HKCR\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKLM\\Software\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKCU\\Software\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKLM\\Software\\Classes\\WOW6432Node\\TypeLib\\{guid}\\{version}\\0\\win32"),
        format!(r"HKLM\\Software\\WOW6432Node\\Classes\\TypeLib\\{guid}\\{version}\\0\\win32"),
    ];
    for key in candidates {
        let value = registry
            .get(&key)
            .map_err(|_| VmError::InvalidConfig("registry key"))?;
        let Some(RegistryValue::String(path)) = value else {
            continue;
        };
        return Ok(Some(path.to_string()));
    }
    Ok(None)
}
