use super::super::error::PeParseError;
use super::super::io::{read_u16, read_u32};
use super::super::types::{DataDirectory, DebugDirectory, DebugDirectoryEntry};
use super::PeFile;

pub(super) fn parse_debug_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<DebugDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("debug rva"))? as usize;
    let count = (dir.size as usize) / 28;
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let entry_off = offset + i * 28;
        if entry_off + 28 > image.len() {
            return Err(PeParseError::UnexpectedEof("debug entry"));
        }
        let characteristics = read_u32(image, entry_off)?;
        let time_date_stamp = read_u32(image, entry_off + 4)?;
        let major_version = read_u16(image, entry_off + 8)?;
        let minor_version = read_u16(image, entry_off + 10)?;
        let debug_type = read_u32(image, entry_off + 12)?;
        let size_of_data = read_u32(image, entry_off + 16)?;
        let address_of_raw_data = read_u32(image, entry_off + 20)?;
        let pointer_to_raw_data = read_u32(image, entry_off + 24)?;
        let mut data = Vec::new();
        if size_of_data > 0 {
            if pointer_to_raw_data != 0 {
                let off = pointer_to_raw_data as usize;
                if off + size_of_data as usize <= image.len() {
                    data = image[off..off + size_of_data as usize].to_vec();
                }
            } else if address_of_raw_data != 0 {
                if let Some(off) = pe.rva_to_offset(address_of_raw_data) {
                    let off = off as usize;
                    if off + size_of_data as usize <= image.len() {
                        data = image[off..off + size_of_data as usize].to_vec();
                    }
                }
            }
        }
        entries.push(DebugDirectoryEntry {
            characteristics,
            time_date_stamp,
            major_version,
            minor_version,
            debug_type,
            size_of_data,
            address_of_raw_data,
            pointer_to_raw_data,
            data,
        });
    }

    Ok(Some(DebugDirectory { entries }))
}
