use super::super::error::PeParseError;
use super::super::io::read_u32;
use super::super::types::{DataDirectory, TlsDirectory};
use super::helpers::va_to_rva;
use super::PeFile;

pub(super) fn parse_tls_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<TlsDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("tls rva"))? as usize;
    if offset + 24 > image.len() {
        return Err(PeParseError::UnexpectedEof("tls directory"));
    }
    let start_raw_data = read_u32(image, offset)?;
    let end_raw_data = read_u32(image, offset + 4)?;
    let address_of_index = read_u32(image, offset + 8)?;
    let address_of_callbacks = read_u32(image, offset + 12)?;
    let size_of_zero_fill = read_u32(image, offset + 16)?;
    let characteristics = read_u32(image, offset + 20)?;

    let mut callbacks = Vec::new();
    if address_of_callbacks != 0 {
        if let Some(callbacks_rva) = va_to_rva(pe, address_of_callbacks) {
            if let Some(off) = pe.rva_to_offset(callbacks_rva) {
                let mut cursor = off as usize;
                for _ in 0..256 {
                    if cursor + 4 > image.len() {
                        break;
                    }
                    let value = read_u32(image, cursor)?;
                    if value == 0 {
                        break;
                    }
                    callbacks.push(value);
                    cursor += 4;
                }
            }
        }
    }

    Ok(Some(TlsDirectory {
        start_raw_data,
        end_raw_data,
        address_of_index,
        address_of_callbacks,
        size_of_zero_fill,
        characteristics,
        callbacks,
    }))
}
