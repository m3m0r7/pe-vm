use super::super::error::PeParseError;
use super::super::io::{read_u16, read_u32, read_utf16_string};
use super::super::types::{DataDirectory, ResourceData, ResourceDirectory, ResourceId, ResourceNode};
use super::PeFile;

pub(super) fn parse_resource_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<ResourceDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let base = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("resource rva"))? as usize;
    let roots = parse_resource_node(image, pe, base, base, 0)?;
    Ok(Some(ResourceDirectory { roots }))
}

fn parse_resource_node(
    image: &[u8],
    pe: &PeFile,
    base: usize,
    offset: usize,
    depth: usize,
) -> Result<Vec<ResourceNode>, PeParseError> {
    if depth > 8 {
        return Err(PeParseError::Invalid("resource depth"));
    }
    if offset + 16 > image.len() {
        return Err(PeParseError::UnexpectedEof("resource directory"));
    }
    let number_of_named_entries = read_u16(image, offset + 12)? as usize;
    let number_of_id_entries = read_u16(image, offset + 14)? as usize;
    let total_entries = number_of_named_entries + number_of_id_entries;
    let mut nodes = Vec::with_capacity(total_entries);
    let entries_off = offset + 16;
    for i in 0..total_entries {
        let entry_off = entries_off + i * 8;
        if entry_off + 8 > image.len() {
            return Err(PeParseError::UnexpectedEof("resource entry"));
        }
        let name = read_u32(image, entry_off)?;
        let data = read_u32(image, entry_off + 4)?;
        let id = if (name & 0x8000_0000) != 0 {
            let name_offset = (name & 0x7FFF_FFFF) as usize;
            let name_off = base + name_offset;
            ResourceId::Name(read_utf16_string(image, name_off)?)
        } else {
            ResourceId::Id(name)
        };
        if (data & 0x8000_0000) != 0 {
            let dir_offset = (data & 0x7FFF_FFFF) as usize;
            let children = parse_resource_node(image, pe, base, base + dir_offset, depth + 1)?;
            nodes.push(ResourceNode {
                id,
                children,
                data: None,
            });
        } else {
            let data_offset = (data & 0x7FFF_FFFF) as usize;
            let data_off = base + data_offset;
            if data_off + 16 > image.len() {
                return Err(PeParseError::UnexpectedEof("resource data"));
            }
            let data_rva = read_u32(image, data_off)?;
            let size = read_u32(image, data_off + 4)?;
            let codepage = read_u32(image, data_off + 8)?;
            let data_bytes = if let Some(off) = pe.rva_to_offset(data_rva) {
                let off = off as usize;
                if off + size as usize > image.len() {
                    return Err(PeParseError::UnexpectedEof("resource data bytes"));
                }
                image[off..off + size as usize].to_vec()
            } else {
                Vec::new()
            };
            nodes.push(ResourceNode {
                id,
                children: Vec::new(),
                data: Some(ResourceData {
                    rva: data_rva,
                    size,
                    codepage,
                    data: data_bytes,
                }),
            });
        }
    }
    Ok(nodes)
}
