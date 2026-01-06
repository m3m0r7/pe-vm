use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use crate::architecture::intel::x86::X86Executor;
use crate::pe::ResourceDirectory;

use super::super::*;

impl Vm {
    pub fn new(config: VmConfig) -> Result<Self, VmError> {
        if config.architecture_value() != Architecture::X86 {
            return Err(VmError::InvalidConfig("only x86 is supported"));
        }
        let os_state = match config.os_value() {
            Os::Windows => OsState::Windows(windows::WindowsState::new(&config)?),
            Os::Unix => OsState::Unix,
            Os::Mac => OsState::Mac,
        };
        let mut vm = Self {
            config,
            os_state,
            base: 0,
            memory: Vec::new(),
            regs: Registers::default(),
            xmm: [[0u8; 16]; 8],
            flags: Flags::default(),
            stack_top: 0,
            stack_depth: 0,
            heap_start: 0,
            heap_end: 0,
            heap_cursor: 0,
            heap_allocs: HashMap::new(),
            fs_base: 0,
            gs_base: 0,
            env: BTreeMap::new(),
            string_overlays: HashMap::new(),
            image_path: None,
            resource_dir: None,
            dispatch_instance: None,
            last_com_out_params: Vec::new(),
            last_error: 0,
            registry_handles: HashMap::new(),
            registry_next_handle: 0x1000_0000,
            file_handles: HashMap::new(),
            file_next_handle: 0x2000,
            virtual_files: HashMap::new(),
            tls_values: HashMap::new(),
            tls_next_index: 1,
            unhandled_exception_filter: 0,
            message_box_mode: MessageBoxMode::default(),
            onexit_tables: BTreeMap::new(),
            default_onexit_table: 0,
            imports_by_name: HashMap::new(),
            imports_by_any: HashMap::new(),
            imports_by_ordinal: HashMap::new(),
            imports_by_iat: HashMap::new(),
            imports_by_iat_name: HashMap::new(),
            dynamic_imports: HashMap::new(),
            dynamic_import_next: 0x7000_0000,
            pending_threads: Vec::new(),
            next_thread_handle: 0x6000_0000,
            stdout: Arc::new(Mutex::new(Vec::new())),
            executor: X86Executor::new(),
        };
        // Register default Windows stubs up front for import resolution.
        if matches!(vm.config.os_value(), Os::Windows) {
            windows::register_default(&mut vm);
        }
        Ok(vm)
    }

    pub fn config(&self) -> &VmConfig {
        &self.config
    }

    pub(crate) fn resource_dir(&self) -> Option<&ResourceDirectory> {
        self.resource_dir.as_ref()
    }

    pub fn network_allowed(&self, _host: &str) -> bool {
        // Allow all traffic unless a sandbox policy disables it.
        let Some(sandbox) = self.config.sandbox_config() else {
            return true;
        };
        sandbox.network_enabled()
    }

    pub fn insert_path_mapping(&mut self, guest: impl Into<String>, host: impl Into<String>) {
        // Allow late-bound path mappings for external callers such as the C ABI.
        self.config.paths_mut().insert(guest.into(), host.into());
    }

    pub fn set_registry(&mut self, registry: windows::registry::Registry) -> Result<(), VmError> {
        // Replace the Windows registry model after VM construction.
        let Some(target) = windows::get_registry_mut(self) else {
            return Err(VmError::InvalidConfig("registry requires Windows VM"));
        };
        *target = registry;
        Ok(())
    }

    pub(crate) fn base(&self) -> u32 {
        self.base
    }

    pub(crate) fn contains_addr(&self, addr: u32) -> bool {
        addr >= self.base && addr.wrapping_sub(self.base) < self.memory.len() as u32
    }

    pub(crate) fn set_image_path(&mut self, path: impl Into<String>) {
        self.image_path = Some(path.into());
    }

    pub(crate) fn image_path(&self) -> Option<&str> {
        self.image_path.as_deref()
    }

    pub(crate) fn set_dispatch_instance(&mut self, value: Option<u32>) -> Option<u32> {
        let prev = self.dispatch_instance;
        self.dispatch_instance = value;
        prev
    }

    pub(crate) fn dispatch_instance(&self) -> Option<u32> {
        self.dispatch_instance
    }
}
