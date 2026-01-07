use super::super::error::PeParseError;
use super::super::io::{read_u16, read_u32};
use super::super::types::{DataDirectory, RelocationBlock, RelocationDirectory, RelocationEntry};
use super::PeFile;

pub(super) fn parse_relocation_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<RelocationDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let mut blocks = Vec::new();
    let mut cursor = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("reloc rva"))? as usize;
    let end = cursor + dir.size as usize;
    while cursor + 8 <= end {
        let page_rva = read_u32(image, cursor)?;
        let block_size = read_u32(image, cursor + 4)? as usize;
        if block_size < 8 {
            break;
        }
        let entry_count = (block_size - 8) / 2;
        let mut entries = Vec::with_capacity(entry_count);
        let entry_base = cursor + 8;
        for i in 0..entry_count {
            let entry = read_u16(image, entry_base + i * 2)?;
            let reloc_type = ((entry >> 12) & 0xF) as u8;
            let offset = (entry & 0x0FFF) as u32;
            entries.push(RelocationEntry {
                rva: page_rva + offset,
                reloc_type,
            });
        }
        blocks.push(RelocationBlock { page_rva, entries });
        cursor += block_size;
    }

    Ok(Some(RelocationDirectory { blocks }))
}
