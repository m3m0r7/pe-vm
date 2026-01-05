// Tests parsing of PE data directories from a synthetic image.
use pe_vm::{PeFile, ResourceId};

const IMAGE_BASE: u32 = 0x0040_0000;
const FILE_ALIGNMENT: u32 = 0x200;
const SECTION_ALIGNMENT: u32 = 0x1000;
const TEXT_RVA: u32 = 0x1000;
const RDATA_RVA: u32 = 0x2000;
const RSRC_RVA: u32 = 0x3000;
const RELOC_RVA: u32 = 0x4000;
const TEXT_RAW: u32 = 0x200;
const RDATA_RAW: u32 = 0x400;
const RSRC_RAW: u32 = 0xA00;
const RELOC_RAW: u32 = 0xC00;
const TEXT_RAW_SIZE: u32 = 0x200;
const RDATA_RAW_SIZE: u32 = 0x600;
const RSRC_RAW_SIZE: u32 = 0x200;
const RELOC_RAW_SIZE: u32 = 0x200;
const SIZE_OF_HEADERS: u32 = 0x200;
const SIZE_OF_IMAGE: u32 = 0x5000;

#[derive(Clone)]
struct SectionLayout {
    base_rva: u32,
    base_raw: u32,
    cursor: usize,
}

impl SectionLayout {
    fn new(base_rva: u32, base_raw: u32) -> Self {
        Self {
            base_rva,
            base_raw,
            cursor: 0,
        }
    }

    fn alloc(&mut self, size: usize, align: usize) -> usize {
        let mask = align.saturating_sub(1);
        let aligned = (self.cursor + mask) & !mask;
        self.cursor = aligned + size;
        aligned
    }

    fn rva(&self, offset: usize) -> u32 {
        self.base_rva + offset as u32
    }

    fn raw(&self, offset: usize) -> usize {
        self.base_raw as usize + offset
    }
}

fn build_directory_dll() -> Vec<u8> {
    let security_size = 0x10usize;
    let security_off = (RELOC_RAW + RELOC_RAW_SIZE) as usize;
    let total_size = security_off + security_size;
    let mut image = vec![0u8; total_size];

    // DOS header.
    image[0] = b'M';
    image[1] = b'Z';
    write_u32(&mut image, 0x3C, 0x80);

    // PE signature.
    let pe_off = 0x80;
    image[pe_off..pe_off + 4].copy_from_slice(b"PE\0\0");

    // File header.
    let file_off = pe_off + 4;
    write_u16(&mut image, file_off + 0, 0x14C); // Machine x86
    write_u16(&mut image, file_off + 2, 4); // NumberOfSections
    write_u16(&mut image, file_off + 16, 0xE0); // SizeOfOptionalHeader
    write_u16(&mut image, file_off + 18, 0x210E); // Characteristics (DLL)

    // Optional header (PE32).
    let opt_off = file_off + 20;
    write_u16(&mut image, opt_off + 0x00, 0x10B);
    write_u32(&mut image, opt_off + 0x04, TEXT_RAW_SIZE); // SizeOfCode
    write_u32(&mut image, opt_off + 0x08, RDATA_RAW_SIZE + RSRC_RAW_SIZE + RELOC_RAW_SIZE);
    write_u32(&mut image, opt_off + 0x10, TEXT_RVA); // EntryPoint
    write_u32(&mut image, opt_off + 0x14, TEXT_RVA); // BaseOfCode
    write_u32(&mut image, opt_off + 0x18, RDATA_RVA); // BaseOfData
    write_u32(&mut image, opt_off + 0x1C, IMAGE_BASE);
    write_u32(&mut image, opt_off + 0x20, SECTION_ALIGNMENT);
    write_u32(&mut image, opt_off + 0x24, FILE_ALIGNMENT);
    write_u16(&mut image, opt_off + 0x28, 4); // Major OS version
    write_u16(&mut image, opt_off + 0x30, 4); // Major subsystem version
    write_u32(&mut image, opt_off + 0x38, SIZE_OF_IMAGE);
    write_u32(&mut image, opt_off + 0x3C, SIZE_OF_HEADERS);
    write_u16(&mut image, opt_off + 0x44, 3); // Subsystem (CUI)
    write_u32(&mut image, opt_off + 0x48, 0x0010_0000); // SizeOfStackReserve
    write_u32(&mut image, opt_off + 0x4C, 0x0000_1000); // SizeOfStackCommit
    write_u32(&mut image, opt_off + 0x50, 0x0010_0000); // SizeOfHeapReserve
    write_u32(&mut image, opt_off + 0x54, 0x0000_1000); // SizeOfHeapCommit
    write_u32(&mut image, opt_off + 0x5C, 16); // NumberOfRvaAndSizes

    // Section headers.
    let sect_off = opt_off + 0xE0;
    write_section(
        &mut image,
        sect_off,
        b".text\0\0\0",
        0x100,
        TEXT_RVA,
        TEXT_RAW_SIZE,
        TEXT_RAW,
        0x6000_0020,
    );
    write_section(
        &mut image,
        sect_off + 40,
        b".rdata\0\0",
        RDATA_RAW_SIZE,
        RDATA_RVA,
        RDATA_RAW_SIZE,
        RDATA_RAW,
        0x4000_0040,
    );
    write_section(
        &mut image,
        sect_off + 80,
        b".rsrc\0\0\0",
        RSRC_RAW_SIZE,
        RSRC_RVA,
        RSRC_RAW_SIZE,
        RSRC_RAW,
        0x4000_0040,
    );
    write_section(
        &mut image,
        sect_off + 120,
        b".reloc\0\0",
        RELOC_RAW_SIZE,
        RELOC_RVA,
        RELOC_RAW_SIZE,
        RELOC_RAW,
        0x4200_0040,
    );

    let mut rdata = SectionLayout::new(RDATA_RVA, RDATA_RAW);
    let exc_off = rdata.alloc(0x10, 4);
    let debug_off = rdata.alloc(0x1C, 4);
    let debug_data_off = rdata.alloc(0x10, 4);
    let tls_off = rdata.alloc(0x18, 4);
    let tls_callbacks_off = rdata.alloc(0x08, 4);
    let tls_index_off = rdata.alloc(0x04, 4);
    let tls_raw_off = rdata.alloc(0x08, 4);
    let load_config_off = rdata.alloc(0x40, 4);

    let bound_off = rdata.alloc(0x18, 4);
    let bound_str_off = rdata.alloc(0x20, 1);
    let bound_end = rdata.cursor;

    let iat_off = rdata.alloc(0x08, 4);

    let delay_off = rdata.alloc(0x40, 4);
    let delay_name_table_off = rdata.alloc(0x08, 4);
    let delay_iat_off = rdata.alloc(0x08, 4);
    let delay_hint_name_off = rdata.alloc(0x10, 2);
    let delay_dll_name_off = rdata.alloc(0x0C, 1);
    let delay_end = rdata.cursor;

    let clr_off = rdata.alloc(0x48, 4);
    let metadata_off = rdata.alloc(0x10, 4);
    let arch_off = rdata.alloc(0x08, 4);
    let global_ptr_off = rdata.alloc(0x04, 4);

    assert!(rdata.cursor <= RDATA_RAW_SIZE as usize);

    // Exception directory bytes.
    write_bytes(&mut image, rdata.raw(exc_off), b"EXCEPTION-TEST");

    // Debug directory entry.
    write_u32(&mut image, rdata.raw(debug_off) + 0, 0);
    write_u32(&mut image, rdata.raw(debug_off) + 4, 0x11223344);
    write_u16(&mut image, rdata.raw(debug_off) + 8, 1);
    write_u16(&mut image, rdata.raw(debug_off) + 10, 0);
    write_u32(&mut image, rdata.raw(debug_off) + 12, 2); // CodeView
    write_u32(&mut image, rdata.raw(debug_off) + 16, 0x10);
    write_u32(&mut image, rdata.raw(debug_off) + 20, rdata.rva(debug_data_off));
    write_u32(&mut image, rdata.raw(debug_off) + 24, rdata.raw(debug_data_off) as u32);
    write_bytes(&mut image, rdata.raw(debug_data_off), b"DEBUGDATA-TEST");

    // TLS directory.
    let tls_raw_va = IMAGE_BASE + rdata.rva(tls_raw_off);
    let tls_index_va = IMAGE_BASE + rdata.rva(tls_index_off);
    let tls_callbacks_va = IMAGE_BASE + rdata.rva(tls_callbacks_off);
    write_u32(&mut image, rdata.raw(tls_off) + 0, tls_raw_va);
    write_u32(&mut image, rdata.raw(tls_off) + 4, tls_raw_va + 8);
    write_u32(&mut image, rdata.raw(tls_off) + 8, tls_index_va);
    write_u32(&mut image, rdata.raw(tls_off) + 12, tls_callbacks_va);
    write_u32(&mut image, rdata.raw(tls_off) + 16, 0);
    write_u32(&mut image, rdata.raw(tls_off) + 20, 0);
    write_u32(&mut image, rdata.raw(tls_callbacks_off), IMAGE_BASE + TEXT_RVA + 0x10);
    write_u32(&mut image, rdata.raw(tls_callbacks_off) + 4, 0);
    write_bytes(&mut image, rdata.raw(tls_raw_off), b"TLS-DATA");

    // Load config directory.
    write_u32(&mut image, rdata.raw(load_config_off) + 0, 0x40);
    write_u32(&mut image, rdata.raw(load_config_off) + 4, 0xAABBCCDD);
    write_u16(&mut image, rdata.raw(load_config_off) + 8, 2);
    write_u16(&mut image, rdata.raw(load_config_off) + 10, 1);
    write_u32(&mut image, rdata.raw(load_config_off) + 12, 0x10);
    write_u32(&mut image, rdata.raw(load_config_off) + 16, 0x20);
    write_u32(&mut image, rdata.raw(load_config_off) + 60, IMAGE_BASE + TEXT_RVA);

    // Bound import directory.
    let bound_module_name = b"bound.dll\0";
    let bound_forward_name = b"fwd.dll\0";
    let bound_module_off = bound_str_off - bound_off;
    let bound_forward_off = bound_module_off + bound_module_name.len();
    write_u32(&mut image, rdata.raw(bound_off) + 0, 0x01020304);
    write_u16(&mut image, rdata.raw(bound_off) + 4, bound_module_off as u16);
    write_u16(&mut image, rdata.raw(bound_off) + 6, 1);
    write_u32(&mut image, rdata.raw(bound_off) + 8, 0x05060708);
    write_u16(&mut image, rdata.raw(bound_off) + 12, bound_forward_off as u16);
    write_u16(&mut image, rdata.raw(bound_off) + 14, 0);
    write_bytes(&mut image, rdata.raw(bound_str_off), bound_module_name);
    write_bytes(
        &mut image,
        rdata.raw(bound_str_off) + bound_module_name.len(),
        bound_forward_name,
    );

    // IAT directory.
    write_u32(&mut image, rdata.raw(iat_off), 0x1122_3344);
    write_u32(&mut image, rdata.raw(iat_off) + 4, 0);

    // Delay import directory.
    write_u32(&mut image, rdata.raw(delay_off) + 0, 1);
    write_u32(&mut image, rdata.raw(delay_off) + 4, rdata.rva(delay_dll_name_off));
    write_u32(&mut image, rdata.raw(delay_off) + 8, 0);
    write_u32(&mut image, rdata.raw(delay_off) + 12, rdata.rva(delay_iat_off));
    write_u32(&mut image, rdata.raw(delay_off) + 16, rdata.rva(delay_name_table_off));
    write_u32(&mut image, rdata.raw(delay_off) + 20, 0);
    write_u32(&mut image, rdata.raw(delay_off) + 24, 0);
    write_u32(&mut image, rdata.raw(delay_off) + 28, 0);
    write_u32(&mut image, rdata.raw(delay_name_table_off), rdata.rva(delay_hint_name_off));
    write_u32(&mut image, rdata.raw(delay_name_table_off) + 4, 0);
    write_u32(&mut image, rdata.raw(delay_iat_off), 0);
    write_u32(&mut image, rdata.raw(delay_iat_off) + 4, 0);
    write_u16(&mut image, rdata.raw(delay_hint_name_off), 0);
    write_bytes(
        &mut image,
        rdata.raw(delay_hint_name_off) + 2,
        b"delay_func\0",
    );
    write_bytes(&mut image, rdata.raw(delay_dll_name_off), b"delay.dll\0");

    // CLR directory.
    write_u32(&mut image, rdata.raw(clr_off) + 0, 0x48);
    write_u16(&mut image, rdata.raw(clr_off) + 4, 2);
    write_u16(&mut image, rdata.raw(clr_off) + 6, 5);
    write_u32(&mut image, rdata.raw(clr_off) + 8, rdata.rva(metadata_off));
    write_u32(&mut image, rdata.raw(clr_off) + 12, 0x10);
    write_u32(&mut image, rdata.raw(clr_off) + 16, 1);
    write_u32(&mut image, rdata.raw(clr_off) + 20, 0x0600_0001);
    write_bytes(&mut image, rdata.raw(metadata_off), b"METADATA-TEST");

    // Architecture directory data.
    write_bytes(&mut image, rdata.raw(arch_off), b"ARCHTEST");

    // Global pointer.
    write_u32(&mut image, rdata.raw(global_ptr_off), IMAGE_BASE + TEXT_RVA + 0x20);

    // Resource section.
    let rsrc_raw = RSRC_RAW as usize;
    write_u16(&mut image, rsrc_raw + 12, 0);
    write_u16(&mut image, rsrc_raw + 14, 1);
    write_u32(&mut image, rsrc_raw + 16, 10);
    write_u32(&mut image, rsrc_raw + 20, 0x8000_0018);
    write_u16(&mut image, rsrc_raw + 0x18 + 12, 0);
    write_u16(&mut image, rsrc_raw + 0x18 + 14, 1);
    write_u32(&mut image, rsrc_raw + 0x28, 1);
    write_u32(&mut image, rsrc_raw + 0x2C, 0x8000_0030);
    write_u16(&mut image, rsrc_raw + 0x30 + 12, 0);
    write_u16(&mut image, rsrc_raw + 0x30 + 14, 1);
    write_u32(&mut image, rsrc_raw + 0x40, 0x0409);
    write_u32(&mut image, rsrc_raw + 0x44, 0x0000_0048);
    write_u32(&mut image, rsrc_raw + 0x48, RSRC_RVA + 0x58);
    write_u32(&mut image, rsrc_raw + 0x4C, 3);
    write_u32(&mut image, rsrc_raw + 0x50, 0);
    write_u32(&mut image, rsrc_raw + 0x54, 0);
    write_bytes(&mut image, rsrc_raw + 0x58, b"RSC");

    // Relocation section.
    let reloc_raw = RELOC_RAW as usize;
    write_u32(&mut image, reloc_raw + 0, TEXT_RVA);
    write_u32(&mut image, reloc_raw + 4, 12);
    write_u16(&mut image, reloc_raw + 8, (3u16 << 12) | 0x10);
    write_u16(&mut image, reloc_raw + 10, 0);

    // Security directory bytes.
    write_bytes(&mut image, security_off, b"SECURITYTESTDATA");

    // Data directories.
    let data_dir_off = opt_off + 0x60;
    write_u32(&mut image, data_dir_off + 0x10, RSRC_RVA); // Resource
    write_u32(&mut image, data_dir_off + 0x14, 0x80);
    write_u32(&mut image, data_dir_off + 0x18, rdata.rva(exc_off)); // Exception
    write_u32(&mut image, data_dir_off + 0x1C, 0x10);
    write_u32(&mut image, data_dir_off + 0x20, security_off as u32); // Security (file offset)
    write_u32(&mut image, data_dir_off + 0x24, security_size as u32);
    write_u32(&mut image, data_dir_off + 0x28, RELOC_RVA); // Reloc
    write_u32(&mut image, data_dir_off + 0x2C, 12);
    write_u32(&mut image, data_dir_off + 0x30, rdata.rva(debug_off)); // Debug
    write_u32(&mut image, data_dir_off + 0x34, 28);
    write_u32(&mut image, data_dir_off + 0x38, rdata.rva(arch_off)); // Architecture
    write_u32(&mut image, data_dir_off + 0x3C, 8);
    write_u32(&mut image, data_dir_off + 0x40, rdata.rva(global_ptr_off)); // GlobalPtr
    write_u32(&mut image, data_dir_off + 0x44, 0);
    write_u32(&mut image, data_dir_off + 0x48, rdata.rva(tls_off)); // TLS
    write_u32(&mut image, data_dir_off + 0x4C, 24);
    write_u32(&mut image, data_dir_off + 0x50, rdata.rva(load_config_off)); // Load Config
    write_u32(&mut image, data_dir_off + 0x54, 0x40);
    write_u32(&mut image, data_dir_off + 0x58, rdata.rva(bound_off)); // Bound Import
    write_u32(&mut image, data_dir_off + 0x5C, (bound_end - bound_off) as u32);
    write_u32(&mut image, data_dir_off + 0x60, rdata.rva(iat_off)); // IAT
    write_u32(&mut image, data_dir_off + 0x64, 8);
    write_u32(&mut image, data_dir_off + 0x68, rdata.rva(delay_off)); // Delay Import
    write_u32(&mut image, data_dir_off + 0x6C, (delay_end - delay_off) as u32);
    write_u32(&mut image, data_dir_off + 0x70, rdata.rva(clr_off)); // CLR
    write_u32(&mut image, data_dir_off + 0x74, 0x48);

    image
}

// Ensure all data directories parse and resource IDs are readable.
#[test]
fn parse_all_directories() {
    let image = build_directory_dll();
    let pe = PeFile::parse(&image).expect("parse");
    let dirs = pe.directories;

    assert!(dirs.resource.is_some());
    assert!(dirs.exception.is_some());
    assert!(dirs.security.is_some());
    assert!(dirs.reloc.is_some());
    assert!(dirs.debug.is_some());
    assert!(dirs.architecture.is_some());
    assert!(dirs.tls.is_some());
    assert!(dirs.load_config.is_some());
    assert!(dirs.bound_import.is_some());
    assert!(dirs.iat.is_some());
    assert!(dirs.delay_import.is_some());
    assert!(dirs.clr.is_some());

    let resource = dirs.resource.unwrap();
    assert_eq!(resource.roots.len(), 1);
    if let ResourceId::Id(value) = resource.roots[0].id {
        assert_eq!(value, 10);
    } else {
        panic!("unexpected resource id");
    }

    let tls = dirs.tls.unwrap();
    assert_eq!(tls.callbacks.len(), 1);
    assert_eq!(tls.callbacks[0], IMAGE_BASE + TEXT_RVA + 0x10);

    let bound = dirs.bound_import.unwrap();
    assert_eq!(bound.descriptors.len(), 1);
    assert_eq!(bound.descriptors[0].module, "bound.dll");

    let delay = dirs.delay_import.unwrap();
    assert_eq!(delay.descriptors.len(), 1);
    assert_eq!(delay.descriptors[0].module, "delay.dll");
    assert_eq!(delay.descriptors[0].symbols.len(), 1);
    assert_eq!(delay.descriptors[0].symbols[0].name.as_deref(), Some("delay_func"));

    let reloc = dirs.reloc.unwrap();
    assert_eq!(reloc.blocks.len(), 1);
    assert_eq!(reloc.blocks[0].entries.len(), 2);

    let security = dirs.security.unwrap();
    assert!(security.data.starts_with(b"SECURITY"));
}

fn write_section(
    image: &mut [u8],
    offset: usize,
    name: &[u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    raw_size: u32,
    raw_ptr: u32,
    characteristics: u32,
) {
    image[offset..offset + 8].copy_from_slice(name);
    write_u32(image, offset + 8, virtual_size);
    write_u32(image, offset + 12, virtual_address);
    write_u32(image, offset + 16, raw_size);
    write_u32(image, offset + 20, raw_ptr);
    write_u32(image, offset + 36, characteristics);
}

fn write_u16(image: &mut [u8], offset: usize, value: u16) {
    image[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(image: &mut [u8], offset: usize, value: u32) {
    image[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_bytes(image: &mut [u8], offset: usize, bytes: &[u8]) {
    image[offset..offset + bytes.len()].copy_from_slice(bytes);
}
