//! COM runtime entry points backed by registry/path mappings.

mod helpers;
mod instance;
mod loader;
mod scan;
mod activex;

use crate::pe::PeFile;
use crate::vm::{Vm, VmError};
use crate::vm::windows;
use super::{ComObject, DispatchHandle, DispatchTable};

pub(super) const IID_ICLASSFACTORY: &str = "{00000001-0000-0000-C000-000000000046}";
pub(super) const IID_IDISPATCH: &str = "{00020400-0000-0000-C000-000000000046}";
pub(super) const IID_IUNKNOWN: &str = "{00000000-0000-0000-C000-000000000046}";

// COM entry point for creating objects.
pub struct Com;

impl Com {
    pub fn new() -> Self {
        Self
    }

    pub fn lookup_dispatch(&self, clsid: &str, table: DispatchTable) -> DispatchHandle {
        let key = helpers::normalize_clsid(clsid);
        DispatchHandle::new(key, table)
    }

    pub fn create_instance(
        &self,
        vm: &mut Vm,
        dispatch: &DispatchHandle,
    ) -> Result<ComObject, VmError> {
        let (normalized, dll_path, host_path) = loader::resolve_inproc_path(vm, dispatch.clsid())?;
        let image = std::fs::read(&host_path)?;
        let _ = PeFile::parse(&image)?;

        Ok(ComObject::new_dispatch(
            normalized,
            dll_path,
            host_path,
            dispatch.dispatch(),
        ))
    }

    pub fn create_instance_inproc(&self, vm: &mut Vm, clsid: &str) -> Result<ComObject, VmError> {
        let (normalized, dll_path, host_path) = loader::resolve_inproc_path(vm, clsid)?;
        let image = std::fs::read(&host_path)?;
        let file = PeFile::parse(&image)?;

        vm.load_image(&file, &image)?;
        vm.set_image_path(dll_path.to_string());
        windows::register_default(vm);
        vm.resolve_imports(&file)?;
        loader::register_server(vm, &file)?;
        loader::init_dll(vm, &file)?;

        let inproc = instance::create_inproc_object(vm, &file, &normalized)?;
        let _ = activex::attach_client_site(vm, inproc.dispatch_ptr());
        Ok(ComObject::new_inproc(
            normalized,
            dll_path,
            host_path,
            inproc,
        ))
    }
}

impl Default for Com {
    fn default() -> Self {
        Self::new()
    }
}
