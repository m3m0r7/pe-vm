use super::super::error::PeParseError;
use super::super::io::{read_c_string, read_u16, read_u32};
use super::super::types::{
    BoundForwarderRef, BoundImportDescriptor, BoundImportDirectory, DataDirectory,
    DelayImportDescriptor, DelayImportDirectory, DelayImportSymbol, ImportDescriptor,
    ImportDirectory, ImportSymbol,
};
use super::helpers::va_to_rva;
use super::PeFile;

pub(super) fn parse_import_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<(Option<ImportDirectory>, Vec<ImportSymbol>), PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok((None, Vec::new()));
    }
    let mut descriptors = Vec::new();
    let mut imports = Vec::new();
    let mut offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("import rva"))? as usize;

    loop {
        if offset + 20 > image.len() {
            return Err(PeParseError::UnexpectedEof("import descriptor"));
        }
        let original_first_thunk = read_u32(image, offset)?;
        let time_date_stamp = read_u32(image, offset + 4)?;
        let forwarder_chain = read_u32(image, offset + 8)?;
        let name_rva = read_u32(image, offset + 12)?;
        let first_thunk = read_u32(image, offset + 16)?;
        if original_first_thunk == 0 && name_rva == 0 && first_thunk == 0 {
            break;
        }

        let name_off = pe
            .rva_to_offset(name_rva)
            .ok_or(PeParseError::Invalid("import name rva"))? as usize;
        let module = read_c_string(image, name_off)?;

        let thunk_rva = if original_first_thunk != 0 {
            original_first_thunk
        } else {
            first_thunk
        };
        let mut thunk_off = pe
            .rva_to_offset(thunk_rva)
            .ok_or(PeParseError::Invalid("import thunk rva"))? as usize;
        let mut index = 0u32;
        let mut symbols = Vec::new();
        loop {
            let value = read_u32(image, thunk_off)?;
            if value == 0 {
                break;
            }
            let iat_rva = first_thunk + index * 4;
            let symbol = if (value & 0x8000_0000) != 0 {
                let ordinal = (value & 0xFFFF) as u16;
                ImportSymbol {
                    module: module.clone(),
                    name: None,
                    ordinal: Some(ordinal),
                    hint: None,
                    iat_rva,
                }
            } else {
                let name_off = pe
                    .rva_to_offset(value)
                    .ok_or(PeParseError::Invalid("import hint/name"))? as usize;
                if name_off + 2 > image.len() {
                    return Err(PeParseError::UnexpectedEof("import hint"));
                }
                let hint = read_u16(image, name_off)?;
                let name = read_c_string(image, name_off + 2)?;
                ImportSymbol {
                    module: module.clone(),
                    name: Some(name),
                    ordinal: None,
                    hint: Some(hint),
                    iat_rva,
                }
            };
            imports.push(symbol.clone());
            symbols.push(symbol);

            thunk_off += 4;
            index += 1;
        }

        descriptors.push(ImportDescriptor {
            module,
            original_first_thunk,
            time_date_stamp,
            forwarder_chain,
            first_thunk,
            symbols,
        });
        offset += 20;
    }

    Ok((Some(ImportDirectory { descriptors }), imports))
}

pub(super) fn parse_delay_import_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<DelayImportDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let mut descriptors = Vec::new();
    let mut offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("delay import rva"))? as usize;

    loop {
        if offset + 32 > image.len() {
            return Err(PeParseError::UnexpectedEof("delay import descriptor"));
        }
        let attributes = read_u32(image, offset)?;
        let name_rva = read_u32(image, offset + 4)?;
        let module_handle_rva = read_u32(image, offset + 8)?;
        let delay_import_address_table = read_u32(image, offset + 12)?;
        let delay_import_name_table = read_u32(image, offset + 16)?;
        let bound_delay_import_table = read_u32(image, offset + 20)?;
        let unload_delay_import_table = read_u32(image, offset + 24)?;
        let time_date_stamp = read_u32(image, offset + 28)?;

        if attributes == 0
            && name_rva == 0
            && module_handle_rva == 0
            && delay_import_address_table == 0
            && delay_import_name_table == 0
            && bound_delay_import_table == 0
            && unload_delay_import_table == 0
            && time_date_stamp == 0
        {
            break;
        }

        let use_rva = (attributes & 1) != 0;
        let module_name_rva = if use_rva {
            name_rva
        } else {
            va_to_rva(pe, name_rva).ok_or(PeParseError::Invalid("delay import name VA"))?
        };
        let module_name_off = pe
            .rva_to_offset(module_name_rva)
            .ok_or(PeParseError::Invalid("delay import name"))? as usize;
        let module = read_c_string(image, module_name_off)?;

        let name_table_rva = if use_rva {
            delay_import_name_table
        } else {
            va_to_rva(pe, delay_import_name_table)
                .ok_or(PeParseError::Invalid("delay import name table"))?
        };
        let iat_rva = if use_rva {
            delay_import_address_table
        } else {
            va_to_rva(pe, delay_import_address_table)
                .ok_or(PeParseError::Invalid("delay import iat"))?
        };

        let mut symbols = Vec::new();
        if name_table_rva != 0 {
            let mut thunk_off = pe
                .rva_to_offset(name_table_rva)
                .ok_or(PeParseError::Invalid("delay import name table"))? as usize;
            let mut index = 0u32;
            loop {
                let value = read_u32(image, thunk_off)?;
                if value == 0 {
                    break;
                }
                let iat_entry_rva = iat_rva + index * 4;
                let symbol = if (value & 0x8000_0000) != 0 {
                    DelayImportSymbol {
                        module: module.clone(),
                        name: None,
                        ordinal: Some((value & 0xFFFF) as u16),
                        hint: None,
                        iat_rva: iat_entry_rva,
                    }
                } else {
                    let hint_name_rva = if use_rva {
                        value
                    } else {
                        va_to_rva(pe, value).ok_or(PeParseError::Invalid("delay import hint"))?
                    };
                    let hint_off = pe
                        .rva_to_offset(hint_name_rva)
                        .ok_or(PeParseError::Invalid("delay import hint"))? as usize;
                    if hint_off + 2 > image.len() {
                        return Err(PeParseError::UnexpectedEof("delay import hint"));
                    }
                    let hint = read_u16(image, hint_off)?;
                    let name = read_c_string(image, hint_off + 2)?;
                    DelayImportSymbol {
                        module: module.clone(),
                        name: Some(name),
                        ordinal: None,
                        hint: Some(hint),
                        iat_rva: iat_entry_rva,
                    }
                };
                symbols.push(symbol);
                thunk_off += 4;
                index += 1;
            }
        }

        descriptors.push(DelayImportDescriptor {
            module,
            attributes,
            name_rva: module_name_rva,
            module_handle_rva,
            delay_import_address_table: iat_rva,
            delay_import_name_table: name_table_rva,
            bound_delay_import_table,
            unload_delay_import_table,
            time_date_stamp,
            symbols,
        });

        offset += 32;
    }

    Ok(Some(DelayImportDirectory { descriptors }))
}

pub(super) fn parse_bound_import_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<BoundImportDirectory>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let base = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("bound import rva"))? as usize;
    let mut offset = base;
    let mut descriptors = Vec::new();
    loop {
        if offset + 8 > image.len() {
            return Err(PeParseError::UnexpectedEof("bound import descriptor"));
        }
        let time_date_stamp = read_u32(image, offset)?;
        let offset_module_name = read_u16(image, offset + 4)? as usize;
        let number_of_forwarder_refs = read_u16(image, offset + 6)? as usize;
        if time_date_stamp == 0 && offset_module_name == 0 && number_of_forwarder_refs == 0 {
            break;
        }
        let module_off = base + offset_module_name;
        let module = read_c_string(image, module_off)?;

        let mut forwarder_refs = Vec::new();
        let mut forward_off = offset + 8;
        for _ in 0..number_of_forwarder_refs {
            if forward_off + 8 > image.len() {
                return Err(PeParseError::UnexpectedEof("bound import forwarder"));
            }
            let f_time = read_u32(image, forward_off)?;
            let f_name_off = read_u16(image, forward_off + 4)? as usize;
            let f_module = read_c_string(image, base + f_name_off)?;
            forwarder_refs.push(BoundForwarderRef {
                module: f_module,
                time_date_stamp: f_time,
            });
            forward_off += 8;
        }

        descriptors.push(BoundImportDescriptor {
            module,
            time_date_stamp,
            forwarder_refs,
        });

        offset = offset + 8 + number_of_forwarder_refs * 8;
    }

    Ok(Some(BoundImportDirectory { descriptors }))
}
