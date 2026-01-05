//! VM execution core.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use crate::pe::PeFile;

use super::*;

const NULL_PAGE_LIMIT: u32 = 0x1000;
const NESTED_STACK_SLICE_SIZE: u32 = 0x20000;

// Register index helpers to avoid repeating match ladders.
macro_rules! reg32_read {
    ($regs:expr, $index:expr) => {
        match $index {
            REG_EAX => $regs.eax,
            REG_ECX => $regs.ecx,
            REG_EDX => $regs.edx,
            REG_EBX => $regs.ebx,
            REG_ESP => $regs.esp,
            REG_EBP => $regs.ebp,
            REG_ESI => $regs.esi,
            _ => $regs.edi,
        }
    };
}

macro_rules! reg32_write {
    ($regs:expr, $index:expr, $value:expr) => {
        match $index {
            REG_EAX => $regs.eax = $value,
            REG_ECX => $regs.ecx = $value,
            REG_EDX => $regs.edx = $value,
            REG_EBX => $regs.ebx = $value,
            REG_ESP => $regs.esp = $value,
            REG_EBP => $regs.ebp = $value,
            REG_ESI => $regs.esi = $value,
            _ => $regs.edi = $value,
        }
    };
}

macro_rules! reg8_read {
    ($regs:expr, $index:expr) => {
        match $index {
            REG_AL => $regs.eax as u8,
            REG_CL => $regs.ecx as u8,
            REG_DL => $regs.edx as u8,
            REG_BL => $regs.ebx as u8,
            REG_AH => ($regs.eax >> 8) as u8,
            REG_CH => ($regs.ecx >> 8) as u8,
            REG_DH => ($regs.edx >> 8) as u8,
            REG_BH => ($regs.ebx >> 8) as u8,
            _ => ($regs.ebx >> 8) as u8,
        }
    };
}

macro_rules! reg8_write {
    ($regs:expr, $index:expr, $value:expr) => {
        match $index {
            REG_AL => $regs.eax = ($regs.eax & 0xFFFF_FF00) | $value as u32,
            REG_CL => $regs.ecx = ($regs.ecx & 0xFFFF_FF00) | $value as u32,
            REG_DL => $regs.edx = ($regs.edx & 0xFFFF_FF00) | $value as u32,
            REG_BL => $regs.ebx = ($regs.ebx & 0xFFFF_FF00) | $value as u32,
            REG_AH => $regs.eax = ($regs.eax & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_CH => $regs.ecx = ($regs.ecx & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_DH => $regs.edx = ($regs.edx & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_BH => $regs.ebx = ($regs.ebx & 0xFFFF_00FF) | (($value as u32) << 8),
            _ => $regs.ebx = ($regs.ebx & 0xFFFF_00FF) | (($value as u32) << 8),
        }
    };
}

impl Vm {
    pub fn new(config: VmConfig) -> Result<Self, VmError> {
        if config.architecture != Architecture::X86 {
            return Err(VmError::InvalidConfig("only x86 is supported"));
        }
        let os_state = match config.os {
            Os::Windows => OsState::Windows(windows::WindowsState::new(&config)?),
            Os::Unix => OsState::Unix,
            Os::Mac => OsState::Mac,
        };
        Ok(Self {
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
            image_path: None,
            dispatch_instance: None,
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
            stdout: Arc::new(Mutex::new(Vec::new())),
            executor: X86Executor::new(),
        })
    }

    pub fn config(&self) -> &VmConfig {
        &self.config
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

    pub(crate) fn set_last_error(&mut self, value: u32) {
        self.last_error = value;
    }

    pub(crate) fn last_error(&self) -> u32 {
        self.last_error
    }

    pub(crate) fn registry_open_handle(&mut self, path: String) -> u32 {
        let handle = self.registry_next_handle;
        self.registry_next_handle = self.registry_next_handle.wrapping_add(4);
        self.registry_handles.insert(handle, path);
        handle
    }

    pub(crate) fn registry_handle_path(&self, handle: u32) -> Option<&str> {
        self.registry_handles.get(&handle).map(|value| value.as_str())
    }

    pub(crate) fn registry_close_handle(&mut self, handle: u32) {
        self.registry_handles.remove(&handle);
    }

    pub(crate) fn file_open(
        &mut self,
        guest_path: &str,
        readable: bool,
        writable: bool,
        create: bool,
        truncate: bool,
    ) -> Result<u32, VmError> {
        let host_path = self.map_path(guest_path);
        if !self.virtual_files.contains_key(&host_path) {
            match std::fs::read(&host_path) {
                Ok(bytes) => {
                    self.virtual_files.insert(host_path.clone(), bytes);
                }
                Err(_) if create => {
                    self.virtual_files.insert(host_path.clone(), Vec::new());
                }
                Err(err) => {
                    return Err(VmError::Io(err));
                }
            }
        }
        if truncate {
            self.virtual_files.insert(host_path.clone(), Vec::new());
        }
        let handle = self.file_next_handle;
        self.file_next_handle = self.file_next_handle.wrapping_add(4);
        self.file_handles.insert(
            handle,
            FileHandle {
                path: host_path,
                cursor: 0,
                readable,
                writable,
            },
        );
        Ok(handle)
    }

    pub(crate) fn file_close(&mut self, handle: u32) -> bool {
        self.file_handles.remove(&handle).is_some()
    }

    pub(crate) fn file_exists(&self, guest_path: &str) -> bool {
        let host_path = self.map_path(guest_path);
        if self.virtual_files.contains_key(&host_path) {
            return true;
        }
        std::fs::metadata(host_path).is_ok()
    }

    pub(crate) fn file_delete(&mut self, guest_path: &str) -> bool {
        let host_path = self.map_path(guest_path);
        self.virtual_files.remove(&host_path).is_some()
    }

    pub(crate) fn file_read(&mut self, handle: u32, len: usize) -> Option<Vec<u8>> {
        let file = self.file_handles.get_mut(&handle)?;
        if !file.readable {
            return Some(Vec::new());
        }
        let data = self.virtual_files.get(&file.path)?;
        if file.cursor >= data.len() {
            return Some(Vec::new());
        }
        let end = usize::min(file.cursor + len, data.len());
        let chunk = data[file.cursor..end].to_vec();
        file.cursor = end;
        Some(chunk)
    }

    pub(crate) fn file_write(&mut self, handle: u32, bytes: &[u8]) -> Option<usize> {
        let file = self.file_handles.get_mut(&handle)?;
        if !file.writable {
            return Some(0);
        }
        let data = self.virtual_files.entry(file.path.clone()).or_default();
        let start = file.cursor;
        let end = start + bytes.len();
        if data.len() < end {
            data.resize(end, 0);
        }
        data[start..end].copy_from_slice(bytes);
        file.cursor = end;
        Some(bytes.len())
    }

    pub(crate) fn file_size(&self, handle: u32) -> Option<u32> {
        let file = self.file_handles.get(&handle)?;
        let data = self.virtual_files.get(&file.path)?;
        u32::try_from(data.len()).ok()
    }

    pub(crate) fn file_seek(&mut self, handle: u32, offset: i64, method: u32) -> Option<u64> {
        let file = self.file_handles.get_mut(&handle)?;
        let len = self
            .virtual_files
            .get(&file.path)
            .map(|data| data.len() as i64)
            .unwrap_or(0);
        let base = match method {
            0 => 0,
            1 => file.cursor as i64,
            2 => len,
            _ => return None,
        };
        let mut next = base + offset;
        if next < 0 {
            next = 0;
        }
        file.cursor = next as usize;
        Some(next as u64)
    }

    pub(crate) fn tls_alloc(&mut self) -> u32 {
        let index = self.tls_next_index;
        self.tls_next_index = self.tls_next_index.wrapping_add(1);
        self.tls_values.insert(index, 0);
        index
    }

    pub(crate) fn tls_set(&mut self, index: u32, value: u32) -> bool {
        if let Some(slot) = self.tls_values.get_mut(&index) {
            *slot = value;
            true
        } else {
            false
        }
    }

    pub(crate) fn tls_get(&self, index: u32) -> u32 {
        self.tls_values.get(&index).copied().unwrap_or(0)
    }

    pub(crate) fn tls_free(&mut self, index: u32) -> bool {
        self.tls_values.remove(&index).is_some()
    }

    pub fn map_path(&self, path: &str) -> String {
        if self.config.paths.is_empty() {
            return path.to_string();
        }
        let is_windows = matches!(self.config.os, Os::Windows);
        let input = normalize_path(path, is_windows);
        let mut best_match: Option<(&String, &String)> = None;
        for (guest_prefix, host_prefix) in &self.config.paths {
            let guest = normalize_path(guest_prefix, is_windows);
            if path_starts_with(&input, &guest, is_windows) {
                let is_better = best_match
                    .as_ref()
                    .map(|(best, _)| guest.len() > normalize_path(best, is_windows).len())
                    .unwrap_or(true);
                if is_better {
                    best_match = Some((guest_prefix, host_prefix));
                }
            }
        }
        let Some((guest_prefix, host_prefix)) = best_match else {
            return input;
        };
        let guest = normalize_path(guest_prefix, is_windows);
        let remainder = input.get(guest.len()..).unwrap_or("");
        let remainder = remainder.trim_start_matches(['\\', '/']);
        if remainder.is_empty() {
            return host_prefix.clone();
        }
        let sep = if host_prefix.contains('\\') { '\\' } else { '/' };
        let needs_sep = !host_prefix.ends_with(['\\', '/']);
        if needs_sep {
            format!("{host_prefix}{sep}{remainder}")
        } else {
            format!("{host_prefix}{remainder}")
        }
    }

    pub fn load(pe: &PeFile, image: &[u8]) -> Result<Self, VmError> {
        let mut vm = Vm::new(VmConfig::new())?;
        vm.load_image(pe, image)?;
        Ok(vm)
    }

    pub fn load_image(&mut self, pe: &PeFile, image: &[u8]) -> Result<(), VmError> {
        let mut loaded = pe.load_image(image, None)?;
        let fs_size = 0x1000usize;
        let heap_size = 0x200000usize;
        let stack_size = 0x100000usize;
        let image_size = loaded.memory.len();
        let fs_start = image_size;
        let heap_start = fs_start + fs_size;
        let heap_end = heap_start + heap_size;

        loaded
            .memory
            .resize(image_size + fs_size + heap_size + stack_size, 0);
        let base = loaded.base;
        let stack_top = base + loaded.memory.len() as u32;

        self.base = base;
        self.memory = loaded.memory;
        self.regs = Registers {
            esp: stack_top,
            ..Registers::default()
        };
        self.xmm = [[0u8; 16]; 8];
        self.flags = Flags::default();
        self.stack_top = stack_top;
        self.stack_depth = 0;
        self.heap_start = heap_start;
        self.heap_end = heap_end;
        self.heap_cursor = heap_start;
        self.heap_allocs.clear();
        self.fs_base = base + fs_start as u32;
        self.gs_base = 0;
        self.imports_by_iat.clear();
        self.dynamic_imports.clear();
        self.dynamic_import_next = 0x7000_0000;
        Ok(())
    }

    pub fn register_import(&mut self, module: &str, name: &str, func: HostCall) {
        self.register_import_with_cleanup(module, name, func, 0);
    }

    pub fn register_import_stdcall(
        &mut self,
        module: &str,
        name: &str,
        stack_cleanup: u32,
        func: HostCall,
    ) {
        self.register_import_with_cleanup(module, name, func, stack_cleanup);
    }

    fn register_import_with_cleanup(
        &mut self,
        module: &str,
        name: &str,
        func: HostCall,
        stack_cleanup: u32,
    ) {
        self.imports_by_name.insert(
            import_key(module, name),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn register_import_any(&mut self, name: &str, func: HostCall) {
        self.register_import_any_with_cleanup(name, func, 0);
    }

    pub fn register_import_any_stdcall(
        &mut self,
        name: &str,
        stack_cleanup: u32,
        func: HostCall,
    ) {
        self.register_import_any_with_cleanup(name, func, stack_cleanup);
    }

    fn register_import_any_with_cleanup(
        &mut self,
        name: &str,
        func: HostCall,
        stack_cleanup: u32,
    ) {
        self.imports_by_any.insert(
            name.to_ascii_lowercase(),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn register_import_ordinal(&mut self, module: &str, ordinal: u16, func: HostCall) {
        self.register_import_ordinal_with_cleanup(module, ordinal, func, 0);
    }

    pub fn register_import_ordinal_stdcall(
        &mut self,
        module: &str,
        ordinal: u16,
        stack_cleanup: u32,
        func: HostCall,
    ) {
        self.register_import_ordinal_with_cleanup(module, ordinal, func, stack_cleanup);
    }

    fn register_import_ordinal_with_cleanup(
        &mut self,
        module: &str,
        ordinal: u16,
        func: HostCall,
        stack_cleanup: u32,
    ) {
        self.imports_by_ordinal.insert(
            import_ordinal_key(module, ordinal),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn resolve_imports(&mut self, pe: &PeFile) {
        self.imports_by_iat.clear();
        self.imports_by_iat_name.clear();
        for import in &pe.imports {
            let mut resolved = None;
            let label = if let Some(name) = &import.name {
                format!("{}!{}", import.module, name)
            } else if let Some(ordinal) = import.ordinal {
                format!("{}!#{}", import.module, ordinal)
            } else {
                format!("{}!<unknown>", import.module)
            };
            if let Some(name) = &import.name {
                if let Some(func) = self
                    .imports_by_name
                    .get(&import_key(&import.module, name))
                    .copied()
                {
                    resolved = Some(func);
                } else if let Some(func) = self
                    .imports_by_any
                    .get(&name.to_ascii_lowercase())
                    .copied()
                {
                    resolved = Some(func);
                }
            } else if let Some(ordinal) = import.ordinal {
                if let Some(func) = self
                    .imports_by_ordinal
                    .get(&import_ordinal_key(&import.module, ordinal))
                    .copied()
                {
                    resolved = Some(func);
                }
            }

            if let Some(func) = resolved {
                let addr = self.base + import.iat_rva;
                self.imports_by_iat.insert(addr, func);
                self.imports_by_iat_name.insert(addr, label.clone());
                if let Ok(value) = self.read_u32(addr) {
                    if value != 0 {
                        self.imports_by_iat.insert(value, func);
                        self.imports_by_iat_name.insert(value, label.clone());
                    }
                }
            } else if std::env::var("PE_VM_TRACE").is_ok() {
                if let Some(name) = &import.name {
                    eprintln!("[pe_vm] Unresolved import: {}!{}", import.module, name);
                } else if let Some(ordinal) = import.ordinal {
                    eprintln!("[pe_vm] Unresolved import: {}!#{}", import.module, ordinal);
                }
                let addr = self.base + import.iat_rva;
                self.imports_by_iat_name.insert(addr, label.clone());
                if let Ok(value) = self.read_u32(addr) {
                    if value != 0 {
                        self.imports_by_iat_name.insert(value, label);
                    }
                }
            }
        }
    }

    pub fn stdout_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
        self.stdout.clone()
    }

    pub fn call_export(&mut self, pe: &PeFile, name: &str) -> Result<(), VmError> {
        let rva = pe
            .export_rva(name)
            .ok_or_else(|| VmError::MissingExport(name.to_string()))?;
        self.execute(self.base + rva)
    }

    pub fn execute_export_with_values(
        &mut self,
        pe: &PeFile,
        name: &str,
        values: &[Value],
        options: ExecuteOptions,
    ) -> Result<u32, VmError> {
        let rva = pe
            .export_rva(name)
            .ok_or_else(|| VmError::MissingExport(name.to_string()))?;
        self.reset_stack();
        if let Some(env) = options.env {
            self.set_env(env);
        }
        self.apply_values(values)?;
        self.execute(self.base + rva)?;
        Ok(self.regs.eax)
    }

    pub fn execute(&mut self, entry: u32) -> Result<(), VmError> {
        if self.memory.is_empty() {
            return Err(VmError::NoImage);
        }
        self.regs.eip = entry;
        self.push(0)?;

        let limit = self.config.execution_limit_value();
        let mut steps = 0u64;
        while self.regs.eip != 0 {
            if limit != 0 && steps > limit {
                if std::env::var("PE_VM_TRACE").is_ok() {
                    let eip = self.regs.eip;
                    let start = eip.wrapping_sub(8);
                    let mut bytes = [0u8; 40];
                    for (idx, slot) in bytes.iter_mut().enumerate() {
                        *slot = self.read_u8(start.wrapping_add(idx as u32)).unwrap_or(0);
                    }
                    let hex = bytes
                        .iter()
                        .map(|value| format!("{value:02X}"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    eprintln!(
                        "[pe_vm] execution limit at eip=0x{eip:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} edi=0x{:08X} bytes@0x{start:08X}={hex}",
                        self.regs.eax,
                        self.regs.ecx,
                        self.regs.edx,
                        self.regs.edi
                    );
                }
                return Err(VmError::ExecutionLimit);
            }
            let executor = self.executor;
            if let Err(err) = executor.step(self) {
                if std::env::var("PE_VM_TRACE_UNSUPPORTED").is_ok() {
                    let eip = self.regs.eip;
                    let start = eip.wrapping_sub(8);
                    let mut bytes = [0u8; 32];
                    for (idx, slot) in bytes.iter_mut().enumerate() {
                        *slot = self.read_u8(start.wrapping_add(idx as u32)).unwrap_or(0);
                    }
                    let hex = bytes
                        .iter()
                        .map(|value| format!("{value:02X}"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    eprintln!(
                        "[pe_vm] step error at eip=0x{eip:08X} err={err:?} bytes@0x{start:08X}={hex}"
                    );
                }
                return Err(err);
            }
            steps += 1;
        }

        Ok(())
    }

    pub fn read_c_string(&self, addr: u32) -> Result<String, VmError> {
        let mut bytes = Vec::new();
        let mut cursor = addr;
        loop {
            let value = self.read_u8(cursor)?;
            if value == 0 {
                break;
            }
            bytes.push(value);
            cursor = cursor.wrapping_add(1);
        }
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u32::from_le_bytes([
            self.memory[offset],
            self.memory[offset + 1],
            self.memory[offset + 2],
            self.memory[offset + 3],
        ]))
    }

    pub fn write_stdout(&self, text: &str) {
        if let Ok(mut buffer) = self.stdout.lock() {
            buffer.extend_from_slice(text.as_bytes());
        }
    }

    pub fn set_env(&mut self, env: BTreeMap<String, String>) {
        self.env = env;
    }

    pub(crate) fn env_value(&self, key: &str) -> Option<&str> {
        self.env.get(key).map(|value| value.as_str())
    }

    pub(crate) fn set_env_entry(&mut self, key: String, value: Option<String>) {
        match value {
            Some(value) => {
                self.env.insert(key, value);
            }
            None => {
                self.env.remove(&key);
            }
        }
    }

    pub(crate) fn eip(&self) -> u32 {
        self.regs.eip
    }

    pub(crate) fn set_eip(&mut self, value: u32) {
        self.regs.eip = value;
    }

    pub(crate) fn reg32(&self, index: u8) -> u32 {
        reg32_read!(self.regs, index)
    }

    pub(crate) fn set_reg32(&mut self, index: u8, value: u32) {
        reg32_write!(self.regs, index, value);
    }

    pub(crate) fn reg16(&self, index: u8) -> u16 {
        self.reg32(index) as u16
    }

    pub(crate) fn set_reg16(&mut self, index: u8, value: u16) {
        let reg = self.reg32(index);
        let next = (reg & 0xFFFF_0000) | value as u32;
        self.set_reg32(index, next);
    }

    pub(crate) fn reg8(&self, index: u8) -> u8 {
        reg8_read!(self.regs, index)
    }

    pub(crate) fn set_reg8(&mut self, index: u8, value: u8) {
        reg8_write!(self.regs, index, value);
    }

    // SSE registers (xmm0-xmm7) used by a subset of SIMD instructions.
    pub(crate) fn xmm(&self, index: u8) -> [u8; 16] {
        self.xmm[index as usize]
    }

    pub(crate) fn set_xmm(&mut self, index: u8, value: [u8; 16]) {
        self.xmm[index as usize] = value;
    }

    pub(crate) fn zf(&self) -> bool {
        self.flags.zf
    }

    pub(crate) fn sf(&self) -> bool {
        self.flags.sf
    }

    pub(crate) fn of(&self) -> bool {
        self.flags.of
    }

    pub(crate) fn cf(&self) -> bool {
        self.flags.cf
    }

    pub(crate) fn set_flags(&mut self, zf: bool, sf: bool, of: bool, cf: bool) {
        self.flags = Flags { cf, zf, sf, of };
    }

    pub(crate) fn fs_base(&self) -> u32 {
        self.fs_base
    }

    pub(crate) fn gs_base(&self) -> u32 {
        self.gs_base
    }

    pub(crate) fn unhandled_exception_filter(&self) -> u32 {
        self.unhandled_exception_filter
    }

    pub(crate) fn set_unhandled_exception_filter(&mut self, value: u32) {
        self.unhandled_exception_filter = value;
    }

    pub fn message_box_mode(&self) -> MessageBoxMode {
        self.message_box_mode
    }

    pub fn set_message_box_mode(&mut self, mode: MessageBoxMode) {
        self.message_box_mode = mode;
    }

    pub fn supported_opcodes(&self) -> (Vec<u8>, Vec<u8>) {
        self.executor.supported_opcodes()
    }

    pub(crate) fn onexit_table_mut(&mut self, table_ptr: u32) -> &mut Vec<u32> {
        self.onexit_tables.entry(table_ptr).or_default()
    }

    pub(crate) fn take_onexit_table(&mut self, table_ptr: u32) -> Vec<u32> {
        self.onexit_tables.remove(&table_ptr).unwrap_or_default()
    }

    pub(crate) fn default_onexit_table(&self) -> u32 {
        self.default_onexit_table
    }

    pub(crate) fn set_default_onexit_table(&mut self, value: u32) {
        self.default_onexit_table = value;
    }

    fn reset_stack(&mut self) {
        if self.stack_top != 0 {
            self.regs.esp = self.stack_top;
        }
    }

    fn apply_values(&mut self, values: &[Value]) -> Result<(), VmError> {
        let mut args = Vec::new();
        for value in values {
            match value {
                Value::Env(env) => self.set_env(env.clone()),
                _ => args.push(value),
            }
        }

        for value in args.into_iter().rev() {
            self.push_value(value)?;
        }
        Ok(())
    }

    fn push_value(&mut self, value: &Value) -> Result<(), VmError> {
        match value {
            Value::U32(v) => self.push(*v),
            Value::U64(v) => {
                let low = *v as u32;
                let high = (*v >> 32) as u32;
                self.push(high)?;
                self.push(low)
            }
            Value::String(text) => {
                let mut bytes = text.as_bytes().to_vec();
                bytes.push(0);
                let addr = self.alloc_bytes(&bytes, 1)?;
                self.push(addr)
            }
            Value::Env(_) => Ok(()),
        }
    }

    // Allocate bytes in the VM heap for host-side helpers (COM/BSTR/etc).
    pub(crate) fn alloc_bytes(&mut self, bytes: &[u8], align: usize) -> Result<u32, VmError> {
        if self.memory.is_empty() {
            return Err(VmError::NoImage);
        }
        let align = align.max(1);
        let mask = align - 1;
        let mut offset = self.heap_cursor;
        if offset & mask != 0 {
            offset = (offset + mask) & !mask;
        }
        let end = offset + bytes.len();
        if end > self.heap_end {
            return Err(VmError::OutOfMemory);
        }
        self.memory[offset..end].copy_from_slice(bytes);
        self.heap_cursor = end;
        Ok(self.base + offset as u32)
    }

    pub(crate) fn heap_alloc(&mut self, size: usize) -> u32 {
        let ptr = self.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0);
        if ptr != 0 {
            self.heap_allocs.insert(ptr, size);
        }
        ptr
    }

    pub(crate) fn heap_realloc(&mut self, ptr: u32, size: usize) -> u32 {
        if ptr == 0 {
            return self.heap_alloc(size);
        }
        let new_ptr = self.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0);
        if new_ptr == 0 {
            return 0;
        }
        if let Some(old_size) = self.heap_allocs.get(&ptr).copied() {
            let copy_len = old_size.min(size);
            for offset in 0..copy_len {
                if let Ok(value) = self.read_u8(ptr.wrapping_add(offset as u32)) {
                    let _ = self.write_u8(new_ptr.wrapping_add(offset as u32), value);
                }
            }
        }
        self.heap_allocs.remove(&ptr);
        self.heap_allocs.insert(new_ptr, size);
        new_ptr
    }

    pub(crate) fn heap_free(&mut self, ptr: u32) -> bool {
        self.heap_allocs.remove(&ptr).is_some()
    }

    pub(crate) fn heap_size(&self, ptr: u32) -> Option<usize> {
        self.heap_allocs.get(&ptr).copied()
    }

    pub(crate) fn resolve_dynamic_import(&mut self, name: &str) -> Option<u32> {
        let key = name.to_ascii_lowercase();
        if let Some(addr) = self.dynamic_imports.get(&key) {
            return Some(*addr);
        }
        let host = self.imports_by_any.get(&key).copied()?;
        let addr = self.dynamic_import_next;
        self.dynamic_import_next = self.dynamic_import_next.wrapping_add(4);
        self.imports_by_iat.insert(addr, host);
        self.imports_by_iat_name
            .insert(addr, format!("dynamic!{name}"));
        self.dynamic_imports.insert(key, addr);
        Some(addr)
    }

    pub(crate) fn execute_at_with_stack(
        &mut self,
        entry: u32,
        values: &[Value],
    ) -> Result<u32, VmError> {
        let saved_regs = self.regs.clone();
        let saved_flags = self.flags;
        let saved_xmm = self.xmm;
        let saved_stack_top = self.stack_top;
        let saved_stack_depth = self.stack_depth;

        let result = (|| {
            if self.stack_top == 0 {
                return Err(VmError::NoImage);
            }
            let depth = saved_stack_depth.checked_add(1).ok_or(VmError::OutOfMemory)?;
            let slice_offset = NESTED_STACK_SLICE_SIZE
                .checked_mul(depth)
                .ok_or(VmError::OutOfMemory)?;
            let stack_top = saved_stack_top
                .checked_sub(slice_offset)
                .ok_or(VmError::OutOfMemory)?;
            let stack_bottom = stack_top
                .checked_sub(NESTED_STACK_SLICE_SIZE)
                .ok_or(VmError::OutOfMemory)?;
            let stack_base = self.base + self.heap_end as u32;
            if stack_bottom < stack_base {
                return Err(VmError::OutOfMemory);
            }
            self.stack_depth = depth;
            self.regs = Registers {
                esp: stack_top,
                ..Registers::default()
            };
            self.xmm = [[0u8; 16]; 8];
            self.flags = Flags::default();

            self.apply_values(values)?;
            if std::env::var("PE_VM_TRACE_STACK").is_ok() {
                let mut line = format!("[pe_vm] stack prep esp=0x{:08X}", self.regs.esp);
                for idx in 0..6 {
                    let addr = self.regs.esp.wrapping_add((idx * 4) as u32);
                    let value = self.read_u32(addr).unwrap_or(0);
                    line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
                }
                eprintln!("{line}");
            }
            self.execute(entry)?;
            Ok(self.regs.eax)
        })();

        self.regs = saved_regs;
        self.flags = saved_flags;
        self.xmm = saved_xmm;
        self.stack_top = saved_stack_top;
        self.stack_depth = saved_stack_depth;

        result
    }

    pub(crate) fn execute_at_with_stack_with_ecx(
        &mut self,
        entry: u32,
        ecx: u32,
        values: &[Value],
    ) -> Result<u32, VmError> {
        let saved_regs = self.regs.clone();
        let saved_flags = self.flags;
        let saved_xmm = self.xmm;
        let saved_stack_top = self.stack_top;
        let saved_stack_depth = self.stack_depth;

        let result = (|| {
            if self.stack_top == 0 {
                return Err(VmError::NoImage);
            }
            let depth = saved_stack_depth.checked_add(1).ok_or(VmError::OutOfMemory)?;
            let slice_offset = NESTED_STACK_SLICE_SIZE
                .checked_mul(depth)
                .ok_or(VmError::OutOfMemory)?;
            let stack_top = saved_stack_top
                .checked_sub(slice_offset)
                .ok_or(VmError::OutOfMemory)?;
            let stack_bottom = stack_top
                .checked_sub(NESTED_STACK_SLICE_SIZE)
                .ok_or(VmError::OutOfMemory)?;
            let stack_base = self.base + self.heap_end as u32;
            if stack_bottom < stack_base {
                return Err(VmError::OutOfMemory);
            }
            self.stack_depth = depth;
            self.regs = Registers {
                ecx,
                esp: stack_top,
                ..Registers::default()
            };
            self.xmm = [[0u8; 16]; 8];
            self.flags = Flags::default();

            self.apply_values(values)?;
            if std::env::var("PE_VM_TRACE_STACK").is_ok() {
                let mut line = format!("[pe_vm] stack prep esp=0x{:08X}", self.regs.esp);
                for idx in 0..6 {
                    let addr = self.regs.esp.wrapping_add((idx * 4) as u32);
                    let value = self.read_u32(addr).unwrap_or(0);
                    line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
                }
                eprintln!("{line}");
            }
            self.execute(entry)?;
            Ok(self.regs.eax)
        })();

        self.regs = saved_regs;
        self.flags = saved_flags;
        self.xmm = saved_xmm;
        self.stack_top = saved_stack_top;
        self.stack_depth = saved_stack_depth;

        result
    }

    pub(crate) fn try_call_import(&mut self, addr: u32, return_eip: u32) -> Result<bool, VmError> {
        if let Some(host) = self.imports_by_iat.get(&addr).copied() {
            if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!("[pe_vm] Import call: {name} addr=0x{addr:08X}");
                }
            }
            self.call_host(host, return_eip)?;
            if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!(
                        "[pe_vm] Import return: {name} eax=0x{:08X}",
                        self.regs.eax
                    );
                }
            }
            Ok(true)
        } else {
            if std::env::var("PE_VM_TRACE").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!(
                        "[pe_vm] Missing import call: {name} addr=0x{addr:08X}"
                    );
                }
            }
            if std::env::var("PE_VM_ABORT_ON_MISSING_IMPORT").is_ok()
                && self.imports_by_iat_name.contains_key(&addr)
            {
                return Err(VmError::InvalidConfig("missing import"));
            }
            Ok(false)
        }
    }

    pub(crate) fn try_jump_import(&mut self, addr: u32) -> Result<bool, VmError> {
        if let Some(host) = self.imports_by_iat.get(&addr).copied() {
            self.call_host_tail(host)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn call_host(&mut self, host: HostFunction, return_eip: u32) -> Result<(), VmError> {
        self.push(return_eip)?;
        let stack_ptr = self.regs.esp;
        let ret = (host.func)(self, stack_ptr);
        self.regs.eax = ret;
        let ret_addr = self.pop()?;
        self.regs.esp = self.regs.esp.wrapping_add(host.stack_cleanup);
        self.regs.eip = ret_addr;
        Ok(())
    }

    fn call_host_tail(&mut self, host: HostFunction) -> Result<(), VmError> {
        // The return address is already on the stack (import thunk jump).
        let stack_ptr = self.regs.esp;
        let ret = (host.func)(self, stack_ptr);
        self.regs.eax = ret;
        let ret_addr = self.pop()?;
        self.regs.esp = self.regs.esp.wrapping_add(host.stack_cleanup);
        self.regs.eip = ret_addr;
        Ok(())
    }

    fn addr_to_offset(&self, addr: u32) -> Result<usize, VmError> {
        if addr < self.base {
            self.log_memory_error(addr);
            return Err(VmError::MemoryOutOfRange);
        }
        let offset = (addr - self.base) as usize;
        if offset >= self.memory.len() {
            self.log_memory_error(addr);
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(offset)
    }

    fn log_memory_error(&self, addr: u32) {
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!(
                "[pe_vm] memory out of range: addr=0x{addr:08X} eip=0x{:08X} base=0x{:08X} size=0x{:08X}",
                self.regs.eip,
                self.base,
                self.memory.len()
            );
            eprintln!(
                "[pe_vm] regs: eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} ebx=0x{:08X} esp=0x{:08X} ebp=0x{:08X} esi=0x{:08X} edi=0x{:08X}",
                self.regs.eax,
                self.regs.ecx,
                self.regs.edx,
                self.regs.ebx,
                self.regs.esp,
                self.regs.ebp,
                self.regs.esi,
                self.regs.edi
            );
            if let Ok(value) = self.read_u32(self.regs.edi) {
                eprintln!("[pe_vm] mem[edi]=0x{value:08X}");
            }
            if let Ok(value) = self.read_u32(self.regs.edi.wrapping_add(0x0C)) {
                eprintln!("[pe_vm] mem[edi+0x0C]=0x{value:08X}");
            }
            if let Ok(value) = self.read_u32(self.regs.ebp.wrapping_add(0x08)) {
                eprintln!("[pe_vm] mem[ebp+0x08]=0x{value:08X}");
            }
            if let Ok(value) = self.read_u32(self.regs.ebp.wrapping_add(0x0C)) {
                eprintln!("[pe_vm] mem[ebp+0x0C]=0x{value:08X}");
            }
            if let Ok(value) = self.read_u32(self.regs.ebp.wrapping_add(0x10)) {
                eprintln!("[pe_vm] mem[ebp+0x10]=0x{value:08X}");
            }
        }
    }

    pub(crate) fn read_u8(&self, addr: u32) -> Result<u8, VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        self.memory
            .get(offset)
            .copied()
            .ok_or(VmError::MemoryOutOfRange)
    }

    pub(crate) fn read_u16(&self, addr: u32) -> Result<u16, VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 2 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u16::from_le_bytes([
            self.memory[offset],
            self.memory[offset + 1],
        ]))
    }

    pub(crate) fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if let Some(slot) = self.memory.get_mut(offset) {
            *slot = value;
            Ok(())
        } else {
            Err(VmError::MemoryOutOfRange)
        }
    }

    pub(crate) fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 2 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let bytes = value.to_le_bytes();
        self.memory[offset..offset + 2].copy_from_slice(&bytes);
        Ok(())
    }

    pub(crate) fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let bytes = value.to_le_bytes();
        self.memory[offset..offset + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub(crate) fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        let end = offset.saturating_add(bytes.len());
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        self.memory[offset..end].copy_from_slice(bytes);
        Ok(())
    }

    pub(crate) fn memset(&mut self, addr: u32, value: u8, len: usize) -> Result<(), VmError> {
        if addr < self.base && addr < NULL_PAGE_LIMIT {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        let end = offset.saturating_add(len);
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        self.memory[offset..end].fill(value);
        Ok(())
    }

    pub(crate) fn push(&mut self, value: u32) -> Result<(), VmError> {
        let new_esp = self.regs.esp.wrapping_sub(4);
        self.write_u32(new_esp, value)?;
        self.regs.esp = new_esp;
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Result<u32, VmError> {
        let value = self.read_u32(self.regs.esp)?;
        self.regs.esp = self.regs.esp.wrapping_add(4);
        Ok(value)
    }
}

fn normalize_path(path: &str, windows: bool) -> String {
    if windows {
        path.replace('/', "\\")
    } else {
        path.to_string()
    }
}

fn path_starts_with(path: &str, prefix: &str, windows: bool) -> bool {
    if windows {
        path.to_ascii_lowercase()
            .starts_with(&prefix.to_ascii_lowercase())
    } else {
        path.starts_with(prefix)
    }
}

fn import_key(module: &str, name: &str) -> String {
    format!("{}!{}", module.to_ascii_lowercase(), name.to_ascii_lowercase())
}

fn import_ordinal_key(module: &str, ordinal: u16) -> String {
    format!("{}!#{}", module.to_ascii_lowercase(), ordinal)
}
