use super::super::error::PeParseError;
use super::super::types::DataDirectory;
use super::PeFile;

pub(super) fn parse_raw_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<Vec<u8>>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("directory rva"))? as usize;
    if offset + dir.size as usize > image.len() {
        return Err(PeParseError::UnexpectedEof("directory bytes"));
    }
    Ok(Some(image[offset..offset + dir.size as usize].to_vec()))
}
