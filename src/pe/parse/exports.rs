use super::super::error::PeParseError;
use super::super::io::{read_c_string, read_u16, read_u32};
use super::super::types::{DataDirectory, ExportDirectory, ExportSymbol};
use super::PeFile;

pub(super) fn parse_export_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<ExportDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let export_off = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("export rva"))? as usize;
    if export_off + 40 > image.len() {
        return Err(PeParseError::UnexpectedEof("export directory"));
    }

    let name_rva = read_u32(image, export_off + 12)?;
    let ordinal_base = read_u32(image, export_off + 16)?;
    let number_of_functions = read_u32(image, export_off + 20)? as usize;
    let number_of_names = read_u32(image, export_off + 24)? as usize;
    let address_of_functions = read_u32(image, export_off + 28)?;
    let address_of_names = read_u32(image, export_off + 32)?;
    let address_of_name_ordinals = read_u32(image, export_off + 36)?;

    let name = if name_rva != 0 {
        let name_off = pe
            .rva_to_offset(name_rva)
            .ok_or(PeParseError::Invalid("export name"))? as usize;
        Some(read_c_string(image, name_off)?)
    } else {
        None
    };

    let mut function_rvas = Vec::with_capacity(number_of_functions);
    for i in 0..number_of_functions {
        let func_rva_off = pe
            .rva_to_offset(address_of_functions + (i as u32) * 4)
            .ok_or(PeParseError::Invalid("export functions"))? as usize;
        function_rvas.push(read_u32(image, func_rva_off)?);
    }

    let mut name_map = vec![None; number_of_functions];
    for i in 0..number_of_names {
        let name_rva_ptr = address_of_names + (i as u32) * 4;
        let name_rva = read_u32(
            image,
            pe.rva_to_offset(name_rva_ptr)
                .ok_or(PeParseError::Invalid("export names"))? as usize,
        )?;
        let name_off = pe
            .rva_to_offset(name_rva)
            .ok_or(PeParseError::Invalid("export name rva"))? as usize;
        let entry_name = read_c_string(image, name_off)?;

        let ordinal_index = read_u16(
            image,
            pe.rva_to_offset(address_of_name_ordinals + (i as u32) * 2)
                .ok_or(PeParseError::Invalid("export ordinals"))? as usize,
        )? as usize;
        if ordinal_index >= number_of_functions {
            return Err(PeParseError::Invalid("export ordinal out of range"));
        }
        name_map[ordinal_index] = Some(entry_name);
    }

    let mut symbols = Vec::with_capacity(number_of_functions);
    for (index, func_rva) in function_rvas.into_iter().enumerate() {
        let forwarder = if func_rva >= dir.rva && func_rva < dir.rva + dir.size && func_rva != 0 {
            let fwd_off = pe
                .rva_to_offset(func_rva)
                .ok_or(PeParseError::Invalid("export forwarder"))? as usize;
            Some(read_c_string(image, fwd_off)?)
        } else {
            None
        };
        symbols.push(ExportSymbol {
            name: name_map[index].clone(),
            ordinal: (ordinal_base as usize + index) as u16,
            rva: func_rva,
            forwarder,
        });
    }

    Ok(Some(ExportDirectory {
        name,
        ordinal_base,
        symbols,
    }))
}
