use std::collections::BTreeMap;
use std::path::Path;

use super::windows;
use super::VmError;

#[derive(Debug, Clone, Copy)]
pub enum Os {
    Windows,
    Unix,
    Mac,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
}

pub type PathMapping = BTreeMap<String, String>;

pub(crate) const fn stdcall_args(args: u32) -> u32 {
    args * 4
}

#[derive(Debug, Clone)]
pub struct VmConfig {
    os: Os,
    architecture: Architecture,
    properties: Option<windows::registry::Registry>,
    paths: PathMapping,
    font_path: Option<String>,
    execution_limit: u64,
    sandbox: Option<SandboxConfig>,
}

impl VmConfig {
    pub fn new() -> Self {
        Self {
            os: Os::Windows,
            architecture: Architecture::X86,
            properties: None,
            paths: PathMapping::new(),
            font_path: None,
            execution_limit: 1_000_000,
            sandbox: None,
        }
    }

    pub fn from_settings(path: impl AsRef<Path>) -> Result<Self, VmError> {
        let settings = crate::settings::load_settings(path)?;
        crate::settings::apply_vm_settings(Self::new(), &settings)
    }

    pub fn from_default_settings() -> Result<Self, VmError> {
        let settings = crate::settings::load_auto_settings()?;
        crate::settings::apply_vm_settings(Self::new(), &settings)
    }

    pub fn os(self, os: Os) -> Self {
        let mut config = self;
        config.os = os;
        config
    }

    pub fn os_value(&self) -> Os {
        self.os
    }

    pub fn architecture(self, architecture: Architecture) -> Self {
        let mut config = self;
        config.architecture = architecture;
        config
    }

    pub fn architecture_value(&self) -> Architecture {
        self.architecture
    }

    pub fn properties(self, properties: windows::registry::Registry) -> Self {
        let mut config = self;
        config.properties = Some(properties);
        config
    }

    pub(crate) fn properties_cloned(&self) -> Option<windows::registry::Registry> {
        self.properties.clone()
    }

    pub fn paths(self, paths: PathMapping) -> Self {
        let mut config = self;
        config.paths = paths;
        config
    }

    pub(crate) fn paths_ref(&self) -> &PathMapping {
        &self.paths
    }

    pub(crate) fn paths_mut(&mut self) -> &mut PathMapping {
        &mut self.paths
    }

    pub fn font_path(self, path: impl Into<String>) -> Self {
        let mut config = self;
        config.font_path = Some(path.into());
        config
    }

    pub fn font_path_opt(&self) -> Option<&str> {
        self.font_path.as_deref()
    }

    pub fn execution_limit(self, limit: u64) -> Self {
        let mut config = self;
        config.execution_limit = limit;
        config
    }

    pub fn execution_limit_value(&self) -> u64 {
        self.execution_limit
    }

    pub fn sandbox(self, sandbox: SandboxConfig) -> Self {
        let mut config = self;
        config.sandbox = Some(sandbox);
        config
    }

    pub fn sandbox_config(&self) -> Option<&SandboxConfig> {
        self.sandbox.as_ref()
    }
}

impl Default for VmConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Sandbox configuration for host-side controls like network access.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    network_enabled: bool,
    network_fallback_host: Option<String>,
}

impl SandboxConfig {
    pub fn new() -> Self {
        Self {
            network_enabled: false,
            network_fallback_host: None,
        }
    }

    pub fn enable_network(self, host: impl Into<String>) -> Self {
        let mut config = self;
        config.network_enabled = true;
        let host = host.into();
        config.network_fallback_host = if host.is_empty() { None } else { Some(host) };
        config
    }

    pub fn disable_network(self) -> Self {
        let mut config = self;
        config.network_enabled = false;
        config.network_fallback_host = None;
        config
    }

    pub fn network_enabled(&self) -> bool {
        self.network_enabled
    }

    pub fn network_fallback_host(&self) -> Option<&str> {
        self.network_fallback_host.as_deref()
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageBoxMode {
    Stdout,
    #[default]
    Dialog,
    Silent,
}
