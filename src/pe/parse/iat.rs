use super::super::error::PeParseError;
use super::super::io::read_u32;
use super::super::types::{DataDirectory, IatDirectory};
use super::PeFile;

pub(super) fn parse_iat_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<IatDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("iat rva"))? as usize;
    let count = (dir.size as usize) / 4;
    if offset + count * 4 > image.len() {
        return Err(PeParseError::UnexpectedEof("iat"));
    }
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        entries.push(read_u32(image, offset + i * 4)?);
    }
    Ok(Some(IatDirectory {
        rva: dir.rva,
        size: dir.size,
        entries,
    }))
}
