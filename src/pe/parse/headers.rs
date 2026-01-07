use super::super::error::PeParseError;
use super::super::io::{read_name, read_u16, read_u32, read_u8};
use super::super::types::{DosHeader, FileHeader, OptionalHeader32, SectionHeader};

pub(super) fn parse_dos_header(image: &[u8]) -> Result<DosHeader, PeParseError> {
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

pub(super) fn parse_file_header(image: &[u8], offset: usize) -> Result<FileHeader, PeParseError> {
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

pub(super) fn parse_optional_header32(
    image: &[u8],
    offset: usize,
) -> Result<OptionalHeader32, PeParseError> {
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

pub(super) fn parse_section_header(
    image: &[u8],
    offset: usize,
) -> Result<SectionHeader, PeParseError> {
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
