use crate::pe::PeFile;
use crate::vm::state::LoadedModule;
use crate::vm::{Value, Vm, VmError};

const MODULE_ALIGN: u32 = 0x1000;

pub(crate) fn module_filename(path: &str) -> String {
    let trimmed = path.trim_end_matches(['\\', '/']);
    let name = trimmed
        .rsplit(['\\', '/'])
        .next()
        .unwrap_or(trimmed);
    if name.is_empty() {
        "module.dll".to_string()
    } else {
        name.to_ascii_lowercase()
    }
}

pub(crate) fn module_name_from_pe(pe: &PeFile) -> String {
    pe.directories
        .export
        .as_ref()
        .and_then(|dir| dir.name.as_ref())
        .map(|name| name.to_ascii_lowercase())
        .unwrap_or_else(|| "module.dll".to_string())
}

fn normalize_module_path(path: &str) -> String {
    path.replace('/', "\\").to_ascii_lowercase()
}

fn looks_absolute_guest_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.get(1) == Some(&b':') || path.starts_with("\\\\") || path.starts_with("//")
        || path.starts_with('\\')
        || path.starts_with('/')
}

fn module_dir(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches(['\\', '/']);
    let (dir, _) = trimmed.rsplit_once(['\\', '/'])?;
    if dir.is_empty() {
        None
    } else {
        Some(dir.to_string())
    }
}

impl Vm {
    pub(crate) fn main_module_handle(&self) -> u32 {
        self.main_module.unwrap_or(self.base)
    }

    pub(crate) fn module_by_handle(&self, handle: u32) -> Option<&LoadedModule> {
        let handle = if handle == 0 {
            self.main_module_handle()
        } else {
            handle
        };
        self.modules.iter().find(|module| module.base == handle)
    }

    pub(crate) fn module_handle_by_name(&self, name: &str) -> Option<u32> {
        let needle = module_filename(name);
        self.modules
            .iter()
            .find(|module| module.name.eq_ignore_ascii_case(&needle))
            .map(|module| module.base)
    }

    pub(crate) fn module_handle_by_path(&self, path: &str) -> Option<u32> {
        let needle = normalize_module_path(path);
        self.modules
            .iter()
            .find(|module| normalize_module_path(&module.guest_path) == needle)
            .map(|module| module.base)
    }

    pub(crate) fn resolve_library_path(&self, name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        if looks_absolute_guest_path(trimmed) {
            return trimmed.to_string();
        }
        let base_dir = self
            .module_by_handle(self.main_module_handle())
            .and_then(|module| {
                if module.guest_path.is_empty() {
                    None
                } else {
                    module_dir(&module.guest_path)
                }
            })
            .or_else(|| self.image_path().and_then(module_dir))
            .unwrap_or_else(|| "C:\\pe_vm".to_string());
        let mut combined = base_dir;
        if !combined.ends_with(['\\', '/']) {
            combined.push('\\');
        }
        combined.push_str(trimmed);
        combined
    }

    pub(crate) fn load_library(&mut self, name: &str) -> Result<u32, VmError> {
        if name.trim().is_empty() {
            return Err(VmError::InvalidConfig("library name is empty"));
        }
        if let Some(handle) = self.module_handle_by_path(name) {
            return Ok(handle);
        }
        if let Some(handle) = self.module_handle_by_name(name) {
            return Ok(handle);
        }
        let resolved = self.resolve_library_path(name);
        if let Some(handle) = self.module_handle_by_path(&resolved) {
            return Ok(handle);
        }
        self.load_module_from_path(&resolved)
    }

    pub(crate) fn load_module_from_path(&mut self, guest_path: &str) -> Result<u32, VmError> {
        if self.memory.is_empty() {
            return Err(VmError::NoImage);
        }
        if let Some(handle) = self.module_handle_by_path(guest_path) {
            return Ok(handle);
        }
        let host_path = self.map_path(guest_path);
        let image = std::fs::read(&host_path)?;
        let pe = PeFile::parse(&image)?;
        let load_base = self.next_module_base();
        let loaded = pe.load_image(&image, Some(load_base))?;
        let module_base = loaded.base;
        let module_size = loaded.memory.len() as u32;
        self.map_module_memory(module_base, &loaded.memory)?;
        let module_name = if guest_path.is_empty() {
            module_name_from_pe(&pe)
        } else {
            module_filename(guest_path)
        };
        let pe_clone = pe.clone();
        self.modules.push(LoadedModule {
            name: module_name,
            guest_path: guest_path.to_string(),
            host_path,
            base: module_base,
            size: module_size,
            pe,
        });
        self.resolve_imports_at(&pe_clone, module_base, false)?;
        self.init_module_entry(module_base, &pe_clone)?;
        Ok(module_base)
    }

    fn next_module_base(&self) -> u32 {
        let end = self.base.wrapping_add(self.memory.len() as u32);
        align_up(end, MODULE_ALIGN)
    }

    fn map_module_memory(&mut self, module_base: u32, memory: &[u8]) -> Result<(), VmError> {
        if module_base < self.base {
            return Err(VmError::InvalidConfig("module base below VM base"));
        }
        let offset = (module_base - self.base) as usize;
        let end = offset.saturating_add(memory.len());
        if end > self.memory.len() {
            self.memory.resize(end, 0);
        }
        self.memory[offset..end].copy_from_slice(memory);
        Ok(())
    }

    fn init_module_entry(&mut self, module_base: u32, pe: &PeFile) -> Result<(), VmError> {
        let entry_rva = pe.optional_header.address_of_entry_point;
        if entry_rva == 0 {
            return Ok(());
        }
        let entry = module_base.wrapping_add(entry_rva);
        let result = self.execute_at_with_stack(
            entry,
            &[Value::U32(module_base), Value::U32(1), Value::U32(0)],
        )?;
        if result == 0 {
            return Err(VmError::InvalidConfig("DllMain returned failure"));
        }
        Ok(())
    }
}

fn align_up(value: u32, align: u32) -> u32 {
    if align == 0 {
        return value;
    }
    let mask = align - 1;
    if value & mask == 0 {
        value
    } else {
        value.wrapping_add(align).wrapping_sub(value & mask)
    }
}
