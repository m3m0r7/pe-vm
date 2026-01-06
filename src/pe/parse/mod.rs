//! PE file parsing routines.

mod clr;
mod debug;
mod exports;
mod headers;
mod helpers;
mod imports;
mod iat;
mod load_config;
mod raw;
mod reloc;
mod resource;
mod security;
mod tls;

use super::error::PeParseError;
use super::image::PeImage;
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
        let dos_header = headers::parse_dos_header(image)?;
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
        let file_header = headers::parse_file_header(image, file_header_off)?;
        if file_header.machine != 0x014C {
            return Err(PeParseError::Unsupported("only x86 PE32 supported"));
        }

        let optional_off = file_header_off + 20;
        let optional_end = optional_off + file_header.size_of_optional_header as usize;
        if optional_end > image.len() {
            return Err(PeParseError::UnexpectedEof("optional header"));
        }
        let optional_header = headers::parse_optional_header32(image, optional_off)?;
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
            let rva = super::io::read_u32(image, off)?;
            let size = super::io::read_u32(image, off + 4)?;
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
            sections.push(headers::parse_section_header(image, off)?);
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

        let export_dir = helpers::directory(&pe.data_directories, DIR_EXPORT);
        let import_dir = helpers::directory(&pe.data_directories, DIR_IMPORT);
        let resource_dir = helpers::directory(&pe.data_directories, DIR_RESOURCE);
        let exception_dir = helpers::directory(&pe.data_directories, DIR_EXCEPTION);
        let security_dir = helpers::directory(&pe.data_directories, DIR_SECURITY);
        let reloc_dir = helpers::directory(&pe.data_directories, DIR_RELOC);
        let debug_dir = helpers::directory(&pe.data_directories, DIR_DEBUG);
        let architecture_dir = helpers::directory(&pe.data_directories, DIR_ARCHITECTURE);
        let global_ptr_dir = helpers::directory(&pe.data_directories, DIR_GLOBALPTR);
        let tls_dir = helpers::directory(&pe.data_directories, DIR_TLS);
        let load_config_dir = helpers::directory(&pe.data_directories, DIR_LOAD_CONFIG);
        let bound_import_dir = helpers::directory(&pe.data_directories, DIR_BOUND_IMPORT);
        let iat_dir = helpers::directory(&pe.data_directories, DIR_IAT);
        let delay_import_dir = helpers::directory(&pe.data_directories, DIR_DELAY_IMPORT);
        let clr_dir = helpers::directory(&pe.data_directories, DIR_CLR);

        pe.directories.export = exports::parse_export_directory(image, &pe, export_dir)?;
        if let Some(dir) = &pe.directories.export {
            pe.exports = dir.symbols.clone();
        }

        let (import_directory, imports) = imports::parse_import_directory(image, &pe, import_dir)?;
        pe.directories.import = import_directory;
        pe.imports = imports;

        pe.directories.resource = resource::parse_resource_directory(image, &pe, resource_dir)?;
        pe.directories.exception = raw::parse_raw_directory(image, &pe, exception_dir)?;
        pe.directories.security = security::parse_security_directory(image, security_dir)?;
        pe.directories.reloc = reloc::parse_relocation_directory(image, &pe, reloc_dir)?;
        pe.directories.debug = debug::parse_debug_directory(image, &pe, debug_dir)?;
        pe.directories.architecture = raw::parse_raw_directory(image, &pe, architecture_dir)?;
        pe.directories.global_ptr = if global_ptr_dir.rva != 0 {
            Some(global_ptr_dir.rva)
        } else {
            None
        };
        pe.directories.tls = tls::parse_tls_directory(image, &pe, tls_dir)?;
        pe.directories.load_config = load_config::parse_load_config_directory(image, &pe, load_config_dir)?;
        pe.directories.bound_import = imports::parse_bound_import_directory(image, &pe, bound_import_dir)?;
        pe.directories.iat = iat::parse_iat_directory(image, &pe, iat_dir)?;
        pe.directories.delay_import = imports::parse_delay_import_directory(image, &pe, delay_import_dir)?;
        pe.directories.clr = clr::parse_clr_directory(image, &pe, clr_dir)?;

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
            let src_offset = section.raw_ptr as usize;
            let src_end = src_offset.saturating_add(section.raw_size as usize);
            if src_offset >= image.len() || src_end > image.len() {
                continue;
            }
            let dst_offset = section.virtual_address as usize;
            let dst_end = dst_offset.saturating_add(section.raw_size as usize);
            if dst_offset >= memory.len() || dst_end > memory.len() {
                continue;
            }
            memory[dst_offset..dst_end].copy_from_slice(&image[src_offset..src_end]);
        }

        let mut image = PeImage { base, memory };
        let reloc_dir = helpers::directory(&self.data_directories, DIR_RELOC);
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
