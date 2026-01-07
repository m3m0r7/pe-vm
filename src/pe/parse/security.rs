use super::super::error::PeParseError;
use super::super::types::{DataDirectory, SecurityDirectory};

pub(super) fn parse_security_directory(
    image: &[u8],
    dir: DataDirectory,
) -> Result<Option<SecurityDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = dir.rva as usize;
    if offset + dir.size as usize > image.len() {
        return Err(PeParseError::UnexpectedEof("security directory"));
    }
    Ok(Some(SecurityDirectory {
        file_offset: dir.rva,
        size: dir.size,
        data: image[offset..offset + dir.size as usize].to_vec(),
    }))
}
