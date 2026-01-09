use crate::pe::PeFile;

use crate::vm::*;

const NULL_PAGE_LIMIT: u32 = 0x1000;
const HIGH_NULL_PAGE_START: u32 = 0xFFFF_F000;

fn is_null_page(addr: u32, base: u32) -> bool {
    (addr < base && addr < NULL_PAGE_LIMIT) || addr >= HIGH_NULL_PAGE_START
}

impl Vm {
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
        self.atl_string_mgr = None;
        self.string_overlays.clear();
        self.resource_dir = pe.directories.resource.clone();
        self.resource_sizes.clear();
        self.fpu_reset();
        Ok(())
    }

    pub fn set_string_overlay(&mut self, addr: u32, value: impl Into<String>) {
        self.string_overlays.insert(addr, value.into());
    }

    pub fn clear_string_overlay(&mut self, addr: u32) {
        self.string_overlays.remove(&addr);
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8, VmError> {
        if is_null_page(addr, self.base) {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        let value = self
            .memory
            .get(offset)
            .copied()
            .ok_or(VmError::MemoryOutOfRange)?;
        self.trace_read("read_u8", addr, 1, value as u32);
        Ok(value)
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, VmError> {
        if is_null_page(addr, self.base) {
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

    pub fn read_u32(&self, addr: u32) -> Result<u32, VmError> {
        if is_null_page(addr, self.base) {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let value = u32::from_le_bytes([
            self.memory[offset],
            self.memory[offset + 1],
            self.memory[offset + 2],
            self.memory[offset + 3],
        ]);
        self.trace_read("read_u32", addr, 4, value);
        Ok(value)
    }

    pub fn read_u64(&self, addr: u32) -> Result<u64, VmError> {
        if is_null_page(addr, self.base) {
            return Ok(0);
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 8 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u64::from_le_bytes([
            self.memory[offset],
            self.memory[offset + 1],
            self.memory[offset + 2],
            self.memory[offset + 3],
            self.memory[offset + 4],
            self.memory[offset + 5],
            self.memory[offset + 6],
            self.memory[offset + 7],
        ]))
    }

    pub(crate) fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        self.trace_write("write_u8", addr, 1, Some(&[value]));
        if let Some(slot) = self.memory.get_mut(offset) {
            *slot = value;
            Ok(())
        } else {
            Err(VmError::MemoryOutOfRange)
        }
    }

    pub(crate) fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 2 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let bytes = value.to_le_bytes();
        self.trace_write("write_u16", addr, bytes.len(), Some(&bytes));
        self.memory[offset..offset + 2].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let bytes = value.to_le_bytes();
        self.trace_write("write_u32", addr, bytes.len(), Some(&bytes));
        self.memory[offset..offset + 4].copy_from_slice(&bytes);
        Ok(())
    }

    pub(crate) fn write_u64(&mut self, addr: u32, value: u64) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        if offset + 8 > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        let bytes = value.to_le_bytes();
        self.trace_write("write_u64", addr, bytes.len(), Some(&bytes));
        self.memory[offset..offset + 8].copy_from_slice(&bytes);
        Ok(())
    }

    pub(crate) fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        let end = offset.saturating_add(bytes.len());
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        self.trace_write("write_bytes", addr, bytes.len(), Some(bytes));
        self.memory[offset..end].copy_from_slice(bytes);
        Ok(())
    }

    pub(crate) fn memset(&mut self, addr: u32, value: u8, len: usize) -> Result<(), VmError> {
        if is_null_page(addr, self.base) {
            return Ok(());
        }
        let offset = self.addr_to_offset(addr)?;
        let end = offset.saturating_add(len);
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        self.trace_write("memset", addr, len, Some(&[value]));
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
            // Print instruction bytes at EIP
            let mut inst_bytes = Vec::new();
            for i in 0..16u32 {
                if let Ok(b) = self.read_u8(self.regs.eip.wrapping_add(i)) {
                    inst_bytes.push(format!("{b:02X}"));
                }
            }
            eprintln!("[pe_vm] inst at eip: {}", inst_bytes.join(" "));
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
            // Print stack trace (return addresses)
            eprintln!("[pe_vm] stack trace (potential return addrs):");
            for i in 0..8u32 {
                let stack_addr = self.regs.esp.wrapping_add(i * 4);
                if let Ok(value) = self.read_u32(stack_addr) {
                    if value >= self.base && value < self.base + self.memory.len() as u32 {
                        eprintln!("[pe_vm]   esp+0x{:02X}: 0x{:08X}", i * 4, value);
                    }
                }
            }
            // Print ebp chain
            let mut ebp = self.regs.ebp;
            for _ in 0..4 {
                if ebp == 0 || ebp < self.base {
                    break;
                }
                if let Ok(ret_addr) = self.read_u32(ebp.wrapping_add(4)) {
                    eprintln!("[pe_vm]   [ebp+4]=0x{:08X} (from ebp=0x{:08X})", ret_addr, ebp);
                }
                if let Ok(next_ebp) = self.read_u32(ebp) {
                    if next_ebp <= ebp || next_ebp >= self.base + self.memory.len() as u32 {
                        break;
                    }
                    ebp = next_ebp;
                } else {
                    break;
                }
            }
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

    pub(crate) fn trace_read(&self, label: &str, addr: u32, len: usize, value: u32) {
        let Ok(watch) = std::env::var("PE_VM_TRACE_ADDR") else {
            return;
        };
        let mut hit = false;
        for token in watch.split(|ch: char| ch == ',' || ch.is_whitespace()) {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let (start, end) = match parse_watch_range(token) {
                Some(range) => range,
                None => continue,
            };
            let read_end = addr.saturating_add(len.saturating_sub(1) as u32);
            if read_end < start || addr > end {
                continue;
            }
            hit = true;
            break;
        }
        if !hit {
            return;
        }
        let end = addr.saturating_add(len.saturating_sub(1) as u32);
        let inst = read_inst_bytes(self, self.regs.eip, 8);
        eprintln!(
            "[pe_vm] trace_read {label} addr=0x{addr:08X}..0x{end:08X} len={len} eip=0x{:08X} eax=0x{:08X} value=0x{value:08X} inst={inst}",
            self.regs.eip,
            self.regs.eax
        );
    }

    pub(crate) fn trace_write(&self, label: &str, addr: u32, len: usize, bytes: Option<&[u8]>) {
        let Ok(watch) = std::env::var("PE_VM_TRACE_ADDR") else {
            return;
        };
        let mut hit = false;
        for token in watch.split(|ch: char| ch == ',' || ch.is_whitespace()) {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let (start, end) = match parse_watch_range(token) {
                Some(range) => range,
                None => continue,
            };
            let write_end = addr.saturating_add(len.saturating_sub(1) as u32);
            if write_end < start || addr > end {
                continue;
            }
            hit = true;
            break;
        }
        if !hit {
            return;
        }
        let end = addr.saturating_add(len.saturating_sub(1) as u32);
        let preview = bytes
            .map(format_bytes)
            .unwrap_or_else(|| "<none>".to_string());
        let inst = read_inst_bytes(self, self.regs.eip, 8);
        eprintln!(
            "[pe_vm] trace_write {label} addr=0x{addr:08X}..0x{end:08X} len={len} eip=0x{:08X} esp=0x{:08X} eax=0x{:08X} inst={inst} preview={preview}",
            self.regs.eip,
            self.regs.esp,
            self.regs.eax
        );
    }
}

fn parse_watch_range(token: &str) -> Option<(u32, u32)> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    if let Some((start, end)) = token.split_once('-') {
        let start = parse_watch_addr(start)?;
        let end = parse_watch_addr(end)?;
        let (min, max) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        return Some((min, max));
    }
    let addr = parse_watch_addr(token)?;
    Some((addr, addr))
}

fn parse_watch_addr(token: &str) -> Option<u32> {
    let token = token.trim();
    if let Some(hex) = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
    {
        return u32::from_str_radix(hex, 16).ok();
    }
    token.parse::<u32>().ok()
}

fn format_bytes(bytes: &[u8]) -> String {
    let limit = bytes.len().min(16);
    let mut out = String::new();
    for (idx, byte) in bytes.iter().take(limit).enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02X}"));
    }
    if bytes.len() > limit {
        out.push_str(" ...");
    }
    out
}

fn read_inst_bytes(vm: &Vm, addr: u32, len: usize) -> String {
    let mut out = String::new();
    for idx in 0..len {
        let byte = vm.read_u8(addr.wrapping_add(idx as u32)).unwrap_or(0);
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02X}"));
    }
    out
}
