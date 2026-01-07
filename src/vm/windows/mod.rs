//! Windows API stub registry.

pub mod kernel32;
pub mod ntdll;
pub mod com;
pub mod advapi32;
mod guid;
pub mod gdi32;
pub mod imagehlp;
pub mod imm32;
pub mod comdlg32;
pub mod ole32;
pub mod oleaut32;
pub mod registry;
pub mod shell32;
pub mod shlwapi;
pub mod user32;
pub mod ucrt;
pub mod vcruntime;
pub mod version;
pub mod wininet;
pub mod ws2_32;
pub mod wtsapi32;

use crate::vm::{OsState, Vm, VmConfig, VmError};

/// Check if not_implemented_module bypass is enabled.
/// If bypass is disabled, panics with a message indicating the function is not implemented.
/// If bypass is enabled, logs a warning and returns normally.
#[inline]
pub fn check_stub(vm: &Vm, dll: &str, function: &str) {
    if vm.config().bypass_settings().not_implemented_module {
        eprintln!("[pe_vm] stub: {dll}!{function} (not implemented, bypassed)");
    } else {
        panic!(
            "Not implemented: {dll}!{function}. \
            Set bypass.not_implemented_module: true in settings to bypass."
        );
    }
}

// Holds Windows-specific VM state such as registry data.
#[derive(Debug, Clone)]
pub struct WindowsState {
    registry: registry::Registry,
}

impl WindowsState {
    pub fn new(config: &VmConfig) -> Result<Self, VmError> {
        let registry = config
            .properties_cloned()
            .unwrap_or_else(registry::Registry::with_defaults);
        Ok(Self { registry })
    }

    pub fn registry(&self) -> &registry::Registry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut registry::Registry {
        &mut self.registry
    }
}

pub fn get_registry(vm: &Vm) -> Option<&registry::Registry> {
    match &vm.os_state {
        OsState::Windows(state) => Some(state.registry()),
        _ => None,
    }
}

pub fn get_registry_mut(vm: &mut Vm) -> Option<&mut registry::Registry> {
    match &mut vm.os_state {
        OsState::Windows(state) => Some(state.registry_mut()),
        _ => None,
    }
}

pub fn register_default(vm: &mut Vm) {
    kernel32::register(vm);
    advapi32::register(vm);
    user32::register(vm);
    ntdll::register(vm);
    imm32::register(vm);
    gdi32::register(vm);
    comdlg32::register(vm);
    shell32::register(vm);
    vcruntime::register(vm);
    ucrt::register(vm);
    ole32::register(vm);
    oleaut32::register(vm);
    shlwapi::register(vm);
    imagehlp::register(vm);
    version::register(vm);
    ws2_32::register(vm);
    wtsapi32::register(vm);
    wininet::register(vm);
}
