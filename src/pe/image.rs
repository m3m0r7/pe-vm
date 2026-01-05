//! PE image loading and relocation.

use super::error::PeParseError;
use super::io::{read_u16, read_u32, write_u32};

#[derive(Debug, Clone)]
pub struct PeImage {
    pub base: u32,
    pub memory: Vec<u8>,
}

impl PeImage {
    pub(super) fn apply_relocations(
        &mut self,
        reloc_rva: u32,
        reloc_size: u32,
        delta: i64,
    ) -> Result<(), PeParseError> {
        let mut cursor = 0usize;
        let table_start = reloc_rva as usize;
        let table_size = reloc_size as usize;
        while cursor + 8 <= table_size {
            let block_off = table_start + cursor;
            let page_rva = read_u32(&self.memory, block_off)?;
            let block_size = read_u32(&self.memory, block_off + 4)? as usize;
            if block_size < 8 {
                break;
            }
            let entry_count = (block_size - 8) / 2;
            let entry_base = block_off + 8;
            for i in 0..entry_count {
                let entry = read_u16(&self.memory, entry_base + (i * 2))?;
                let reloc_type = (entry >> 12) & 0xF;
                let offset = (entry & 0x0FFF) as u32;
                match reloc_type {
                    0 => continue,
                    3 => {
                        let addr = (page_rva + offset) as usize;
                        let value = read_u32(&self.memory, addr)? as i64;
                        let patched = (value + delta) as u32;
                        write_u32(&mut self.memory, addr, patched)?;
                    }
                    _ => return Err(PeParseError::Unsupported("relocation type")),
                }
            }
            cursor += block_size;
        }
        Ok(())
    }
}
