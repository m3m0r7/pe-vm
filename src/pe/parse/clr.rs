use super::super::error::PeParseError;
use super::super::io::{read_u16, read_u32};
use super::super::types::{ClrDirectory, DataDirectory};
use super::PeFile;

pub(super) fn parse_clr_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<ClrDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("clr rva"))? as usize;
    if offset + 0x48 > image.len() {
        return Err(PeParseError::UnexpectedEof("clr header"));
    }
    let cb = read_u32(image, offset)?;
    let major_runtime_version = read_u16(image, offset + 4)?;
    let minor_runtime_version = read_u16(image, offset + 6)?;
    let metadata = DataDirectory {
        rva: read_u32(image, offset + 8)?,
        size: read_u32(image, offset + 12)?,
    };
    let flags = read_u32(image, offset + 16)?;
    let entry_point_token = read_u32(image, offset + 20)?;
    let resources = DataDirectory {
        rva: read_u32(image, offset + 24)?,
        size: read_u32(image, offset + 28)?,
    };
    let strong_name_signature = DataDirectory {
        rva: read_u32(image, offset + 32)?,
        size: read_u32(image, offset + 36)?,
    };
    let code_manager_table = DataDirectory {
        rva: read_u32(image, offset + 40)?,
        size: read_u32(image, offset + 44)?,
    };
    let vtable_fixups = DataDirectory {
        rva: read_u32(image, offset + 48)?,
        size: read_u32(image, offset + 52)?,
    };
    let export_address_table_jumps = DataDirectory {
        rva: read_u32(image, offset + 56)?,
        size: read_u32(image, offset + 60)?,
    };
    let managed_native_header = DataDirectory {
        rva: read_u32(image, offset + 64)?,
        size: read_u32(image, offset + 68)?,
    };

    Ok(Some(ClrDirectory {
        cb,
        major_runtime_version,
        minor_runtime_version,
        metadata,
        flags,
        entry_point_token,
        resources,
        strong_name_signature,
        code_manager_table,
        vtable_fixups,
        export_address_table_jumps,
        managed_native_header,
    }))
}
