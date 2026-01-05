//! PE file parsing routines.

use super::error::PeParseError;
use super::image::PeImage;
use super::io::{
    read_c_string, read_name, read_u16, read_u16_opt, read_u32, read_u32_opt, read_u8,
    read_utf16_string,
};
use super::types::*;

const DIRECTORY_COUNT: usize = 16;
const DIR_EXPORT: usize = 0;
const DIR_IMPORT: usize = 1;
const DIR_RESOURCE: usize = 2;
const DIR_EXCEPTION: usize = 3;
const DIR_SECURITY: usize = 4;
const DIR_RELOC: usize = 5;
const DIR_DEBUG: usize = 6;
const DIR_ARCHITECTURE: usize = 7;
const DIR_GLOBALPTR: usize = 8;
const DIR_TLS: usize = 9;
const DIR_LOAD_CONFIG: usize = 10;
const DIR_BOUND_IMPORT: usize = 11;
const DIR_IAT: usize = 12;
const DIR_DELAY_IMPORT: usize = 13;
const DIR_CLR: usize = 14;

#[derive(Debug, Clone)]
pub struct PeFile {
    pub dos_header: DosHeader,
    pub file_header: FileHeader,
    pub optional_header: OptionalHeader32,
    pub sections: Vec<SectionHeader>,
    pub data_directories: Vec<DataDirectory>,
    pub directories: PeDirectories,
    pub imports: Vec<ImportSymbol>,
    pub exports: Vec<ExportSymbol>,
}

impl PeFile {
    pub fn parse(image: &[u8]) -> Result<Self, PeParseError> {
        let dos_header = parse_dos_header(image)?;
        if dos_header.e_magic != 0x5A4D {
            return Err(PeParseError::InvalidSignature("missing MZ"));
        }

        let pe_off = dos_header.e_lfanew as usize;
        if pe_off + 4 + 20 > image.len() {
            return Err(PeParseError::UnexpectedEof("pe header"));
        }
        if &image[pe_off..pe_off + 4] != b"PE\0\0" {
            return Err(PeParseError::InvalidSignature("missing PE"));
        }

        let file_header_off = pe_off + 4;
        let file_header = parse_file_header(image, file_header_off)?;
        if file_header.machine != 0x014C {
            return Err(PeParseError::Unsupported("only x86 PE32 supported"));
        }

        let optional_off = file_header_off + 20;
        let optional_end = optional_off + file_header.size_of_optional_header as usize;
        if optional_end > image.len() {
            return Err(PeParseError::UnexpectedEof("optional header"));
        }
        let optional_header = parse_optional_header32(image, optional_off)?;
        if optional_header.magic != 0x10B {
            return Err(PeParseError::Unsupported("not PE32"));
        }

        let data_dir_off = optional_off + 0x60;
        let dir_count = (optional_header.number_of_rva_and_sizes as usize).min(DIRECTORY_COUNT);
        if data_dir_off + dir_count * 8 > optional_end {
            return Err(PeParseError::UnexpectedEof("data directory"));
        }
        let mut data_directories = Vec::with_capacity(dir_count);
        for i in 0..dir_count {
            let off = data_dir_off + i * 8;
            let rva = read_u32(image, off)?;
            let size = read_u32(image, off + 4)?;
            data_directories.push(DataDirectory { rva, size });
        }

        let section_off = optional_off + file_header.size_of_optional_header as usize;
        let section_bytes = file_header.number_of_sections as usize * 40;
        if section_off + section_bytes > image.len() {
            return Err(PeParseError::UnexpectedEof("section headers"));
        }
        let mut sections = Vec::with_capacity(file_header.number_of_sections as usize);
        for i in 0..file_header.number_of_sections as usize {
            let off = section_off + i * 40;
            sections.push(parse_section_header(image, off)?);
        }

        let mut pe = PeFile {
            dos_header,
            file_header,
            optional_header,
            sections,
            data_directories,
            directories: PeDirectories::default(),
            imports: Vec::new(),
            exports: Vec::new(),
        };

        let export_dir = directory(&pe.data_directories, DIR_EXPORT);
        let import_dir = directory(&pe.data_directories, DIR_IMPORT);
        let resource_dir = directory(&pe.data_directories, DIR_RESOURCE);
        let exception_dir = directory(&pe.data_directories, DIR_EXCEPTION);
        let security_dir = directory(&pe.data_directories, DIR_SECURITY);
        let reloc_dir = directory(&pe.data_directories, DIR_RELOC);
        let debug_dir = directory(&pe.data_directories, DIR_DEBUG);
        let architecture_dir = directory(&pe.data_directories, DIR_ARCHITECTURE);
        let global_ptr_dir = directory(&pe.data_directories, DIR_GLOBALPTR);
        let tls_dir = directory(&pe.data_directories, DIR_TLS);
        let load_config_dir = directory(&pe.data_directories, DIR_LOAD_CONFIG);
        let bound_import_dir = directory(&pe.data_directories, DIR_BOUND_IMPORT);
        let iat_dir = directory(&pe.data_directories, DIR_IAT);
        let delay_import_dir = directory(&pe.data_directories, DIR_DELAY_IMPORT);
        let clr_dir = directory(&pe.data_directories, DIR_CLR);

        pe.directories.export = parse_export_directory(image, &pe, export_dir)?;
        if let Some(dir) = &pe.directories.export {
            pe.exports = dir.symbols.clone();
        }

        let (import_directory, imports) = parse_import_directory(image, &pe, import_dir)?;
        pe.directories.import = import_directory;
        pe.imports = imports;

        pe.directories.resource = parse_resource_directory(image, &pe, resource_dir)?;
        pe.directories.exception = parse_raw_directory(image, &pe, exception_dir)?;
        pe.directories.security = parse_security_directory(image, security_dir)?;
        pe.directories.reloc = parse_relocation_directory(image, &pe, reloc_dir)?;
        pe.directories.debug = parse_debug_directory(image, &pe, debug_dir)?;
        pe.directories.architecture = parse_raw_directory(image, &pe, architecture_dir)?;
        pe.directories.global_ptr = if global_ptr_dir.rva != 0 {
            Some(global_ptr_dir.rva)
        } else {
            None
        };
        pe.directories.tls = parse_tls_directory(image, &pe, tls_dir)?;
        pe.directories.load_config = parse_load_config_directory(image, &pe, load_config_dir)?;
        pe.directories.bound_import = parse_bound_import_directory(image, &pe, bound_import_dir)?;
        pe.directories.iat = parse_iat_directory(image, &pe, iat_dir)?;
        pe.directories.delay_import = parse_delay_import_directory(image, &pe, delay_import_dir)?;
        pe.directories.clr = parse_clr_directory(image, &pe, clr_dir)?;

        Ok(pe)
    }

    pub fn image_base(&self) -> u32 {
        self.optional_header.image_base
    }

    pub fn rva_to_offset(&self, rva: u32) -> Option<u32> {
        if rva == 0 {
            return None;
        }
        if rva < self.optional_header.size_of_headers {
            return Some(rva);
        }
        for section in &self.sections {
            let start = section.virtual_address;
            let size = section.virtual_size.max(section.raw_size);
            let end = start.saturating_add(size);
            if rva >= start && rva < end {
                return Some(section.raw_ptr + (rva - start));
            }
        }
        None
    }

    pub fn export_rva(&self, name: &str) -> Option<u32> {
        self.exports
            .iter()
            .find(|symbol| symbol.name.as_deref() == Some(name))
            .map(|symbol| symbol.rva)
    }

    pub fn load_image(&self, image: &[u8], load_base: Option<u32>) -> Result<PeImage, PeParseError> {
        let mut base = load_base.unwrap_or(self.optional_header.image_base);
        base &= !0xFFF;
        if base == 0 {
            base = 0x0040_0000;
        }

        let size = self.optional_header.size_of_image as usize;
        if size == 0 {
            return Err(PeParseError::Invalid("size_of_image is zero"));
        }
        let mut memory = vec![0u8; size];

        let headers_size = self.optional_header.size_of_headers as usize;
        if headers_size > image.len() || headers_size > memory.len() {
            return Err(PeParseError::Invalid("size_of_headers out of range"));
        }
        memory[0..headers_size].copy_from_slice(&image[0..headers_size]);

        for section in &self.sections {
            let src_start = section.raw_ptr as usize;
            let src_end = src_start.saturating_add(section.raw_size as usize);
            if src_start >= image.len() {
                continue;
            }
            let src_end = src_end.min(image.len());
            let dst_start = section.virtual_address as usize;
            if dst_start >= memory.len() {
                continue;
            }
            let dst_end = (dst_start + (src_end - src_start)).min(memory.len());
            memory[dst_start..dst_end]
                .copy_from_slice(&image[src_start..src_start + (dst_end - dst_start)]);

            if section.virtual_size > section.raw_size {
                let zero_start = dst_start + section.raw_size as usize;
                let zero_end = (dst_start + section.virtual_size as usize).min(memory.len());
                for byte in &mut memory[zero_start..zero_end] {
                    *byte = 0;
                }
            }
        }

        let mut image = PeImage { base, memory };
        let reloc_dir = directory(&self.data_directories, DIR_RELOC);
        if base != self.optional_header.image_base && reloc_dir.rva != 0 && reloc_dir.size != 0 {
            image.apply_relocations(
                reloc_dir.rva,
                reloc_dir.size,
                base as i64 - self.optional_header.image_base as i64,
            )?;
        }

        Ok(image)
    }
}

fn parse_dos_header(image: &[u8]) -> Result<DosHeader, PeParseError> {
    if image.len() < 0x40 {
        return Err(PeParseError::UnexpectedEof("dos header"));
    }
    let e_magic = read_u16(image, 0x00)?;
    let e_cblp = read_u16(image, 0x02)?;
    let e_cp = read_u16(image, 0x04)?;
    let e_crlc = read_u16(image, 0x06)?;
    let e_cparhdr = read_u16(image, 0x08)?;
    let e_minalloc = read_u16(image, 0x0A)?;
    let e_maxalloc = read_u16(image, 0x0C)?;
    let e_ss = read_u16(image, 0x0E)?;
    let e_sp = read_u16(image, 0x10)?;
    let e_csum = read_u16(image, 0x12)?;
    let e_ip = read_u16(image, 0x14)?;
    let e_cs = read_u16(image, 0x16)?;
    let e_lfarlc = read_u16(image, 0x18)?;
    let e_ovno = read_u16(image, 0x1A)?;
    let mut e_res = [0u16; 4];
    for (i, slot) in e_res.iter_mut().enumerate() {
        *slot = read_u16(image, 0x1C + i * 2)?;
    }
    let e_oemid = read_u16(image, 0x24)?;
    let e_oeminfo = read_u16(image, 0x26)?;
    let mut e_res2 = [0u16; 10];
    for (i, slot) in e_res2.iter_mut().enumerate() {
        *slot = read_u16(image, 0x28 + i * 2)?;
    }
    let e_lfanew = read_u32(image, 0x3C)?;

    Ok(DosHeader {
        e_magic,
        e_cblp,
        e_cp,
        e_crlc,
        e_cparhdr,
        e_minalloc,
        e_maxalloc,
        e_ss,
        e_sp,
        e_csum,
        e_ip,
        e_cs,
        e_lfarlc,
        e_ovno,
        e_res,
        e_oemid,
        e_oeminfo,
        e_res2,
        e_lfanew,
    })
}

fn parse_file_header(image: &[u8], offset: usize) -> Result<FileHeader, PeParseError> {
    Ok(FileHeader {
        machine: read_u16(image, offset)?,
        number_of_sections: read_u16(image, offset + 2)?,
        time_date_stamp: read_u32(image, offset + 4)?,
        pointer_to_symbol_table: read_u32(image, offset + 8)?,
        number_of_symbols: read_u32(image, offset + 12)?,
        size_of_optional_header: read_u16(image, offset + 16)?,
        characteristics: read_u16(image, offset + 18)?,
    })
}

fn parse_optional_header32(image: &[u8], offset: usize) -> Result<OptionalHeader32, PeParseError> {
    Ok(OptionalHeader32 {
        magic: read_u16(image, offset)?,
        major_linker_version: read_u8(image, offset + 2)?,
        minor_linker_version: read_u8(image, offset + 3)?,
        size_of_code: read_u32(image, offset + 4)?,
        size_of_initialized_data: read_u32(image, offset + 8)?,
        size_of_uninitialized_data: read_u32(image, offset + 12)?,
        address_of_entry_point: read_u32(image, offset + 16)?,
        base_of_code: read_u32(image, offset + 20)?,
        base_of_data: read_u32(image, offset + 24)?,
        image_base: read_u32(image, offset + 28)?,
        section_alignment: read_u32(image, offset + 32)?,
        file_alignment: read_u32(image, offset + 36)?,
        major_operating_system_version: read_u16(image, offset + 40)?,
        minor_operating_system_version: read_u16(image, offset + 42)?,
        major_image_version: read_u16(image, offset + 44)?,
        minor_image_version: read_u16(image, offset + 46)?,
        major_subsystem_version: read_u16(image, offset + 48)?,
        minor_subsystem_version: read_u16(image, offset + 50)?,
        win32_version_value: read_u32(image, offset + 52)?,
        size_of_image: read_u32(image, offset + 56)?,
        size_of_headers: read_u32(image, offset + 60)?,
        checksum: read_u32(image, offset + 64)?,
        subsystem: read_u16(image, offset + 68)?,
        dll_characteristics: read_u16(image, offset + 70)?,
        size_of_stack_reserve: read_u32(image, offset + 72)?,
        size_of_stack_commit: read_u32(image, offset + 76)?,
        size_of_heap_reserve: read_u32(image, offset + 80)?,
        size_of_heap_commit: read_u32(image, offset + 84)?,
        loader_flags: read_u32(image, offset + 88)?,
        number_of_rva_and_sizes: read_u32(image, offset + 92)?,
    })
}

fn parse_section_header(image: &[u8], offset: usize) -> Result<SectionHeader, PeParseError> {
    let name = read_name(image, offset)?;
    Ok(SectionHeader {
        name,
        virtual_size: read_u32(image, offset + 8)?,
        virtual_address: read_u32(image, offset + 12)?,
        raw_size: read_u32(image, offset + 16)?,
        raw_ptr: read_u32(image, offset + 20)?,
        pointer_to_relocations: read_u32(image, offset + 24)?,
        pointer_to_linenumbers: read_u32(image, offset + 28)?,
        number_of_relocations: read_u16(image, offset + 32)?,
        number_of_linenumbers: read_u16(image, offset + 34)?,
        characteristics: read_u32(image, offset + 36)?,
    })
}

fn parse_import_directory(
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

fn parse_delay_import_directory(
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

fn parse_bound_import_directory(
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

fn parse_export_directory(
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

fn parse_relocation_directory(
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

fn parse_resource_directory(
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

fn parse_debug_directory(
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

fn parse_tls_directory(
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

fn parse_load_config_directory(
    image: &[u8],
    pe: &PeFile,
    dir: DataDirectory,
) -> Result<Option<LoadConfigDirectory32>, PeParseError> {
    if dir.rva == 0 || dir.size == 0 {
        return Ok(None);
    }
    let offset = pe
        .rva_to_offset(dir.rva)
        .ok_or(PeParseError::Invalid("load config rva"))? as usize;
    if offset + 4 > image.len() {
        return Err(PeParseError::UnexpectedEof("load config"));
    }
    let max = (dir.size as usize).min(image.len().saturating_sub(offset));
    let limit = offset + max;
    let cfg = LoadConfigDirectory32 {
        size: read_u32_opt(image, offset, limit),
        time_date_stamp: read_u32_opt(image, offset + 4, limit),
        major_version: read_u16_opt(image, offset + 8, limit),
        minor_version: read_u16_opt(image, offset + 10, limit),
        global_flags_clear: read_u32_opt(image, offset + 12, limit),
        global_flags_set: read_u32_opt(image, offset + 16, limit),
        critical_section_default_timeout: read_u32_opt(image, offset + 20, limit),
        decommit_free_block_threshold: read_u32_opt(image, offset + 24, limit),
        decommit_total_free_threshold: read_u32_opt(image, offset + 28, limit),
        lock_prefix_table: read_u32_opt(image, offset + 32, limit),
        maximum_allocation_size: read_u32_opt(image, offset + 36, limit),
        virtual_memory_threshold: read_u32_opt(image, offset + 40, limit),
        process_affinity_mask: read_u32_opt(image, offset + 44, limit),
        process_heap_flags: read_u32_opt(image, offset + 48, limit),
        csd_version: read_u16_opt(image, offset + 52, limit),
        dependent_load_flags: read_u16_opt(image, offset + 54, limit),
        edit_list: read_u32_opt(image, offset + 56, limit),
        security_cookie: read_u32_opt(image, offset + 60, limit),
        se_handler_table: read_u32_opt(image, offset + 64, limit),
        se_handler_count: read_u32_opt(image, offset + 68, limit),
        guard_cf_check_function_pointer: read_u32_opt(image, offset + 72, limit),
        guard_cf_dispatch_function_pointer: read_u32_opt(image, offset + 76, limit),
        guard_cf_function_table: read_u32_opt(image, offset + 80, limit),
        guard_cf_function_count: read_u32_opt(image, offset + 84, limit),
        guard_flags: read_u32_opt(image, offset + 88, limit),
    };

    Ok(Some(cfg))
}

fn parse_iat_directory(
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

fn parse_security_directory(
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

fn parse_raw_directory(
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

fn parse_clr_directory(
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

fn directory(dirs: &[DataDirectory], index: usize) -> DataDirectory {
    dirs.get(index).copied().unwrap_or(DataDirectory { rva: 0, size: 0 })
}

fn va_to_rva(pe: &PeFile, va: u32) -> Option<u32> {
    if va == 0 {
        return None;
    }
    if va < pe.optional_header.image_base {
        return None;
    }
    Some(va - pe.optional_header.image_base)
}
