// Shared PE fixtures for integration tests.

const IMAGE_BASE: u32 = 0x0040_0000;
const FILE_ALIGNMENT: u32 = 0x200;
const SECTION_ALIGNMENT: u32 = 0x1000;
const TEXT_RVA: u32 = 0x1000;
const RDATA_RVA: u32 = 0x2000;
const TEXT_RAW: u32 = 0x200;
const RDATA_RAW: u32 = 0x400;
const TEXT_RAW_SIZE: u32 = 0x200;
const RDATA_RAW_SIZE: u32 = 0x200;
const SIZE_OF_HEADERS: u32 = 0x200;
const SIZE_OF_IMAGE: u32 = 0x3000;

pub fn build_test_dll() -> Vec<u8> {
    let total_size = (RDATA_RAW + RDATA_RAW_SIZE) as usize;
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
    write_u16(&mut image, file_off + 2, 2); // NumberOfSections
    write_u16(&mut image, file_off + 16, 0xE0); // SizeOfOptionalHeader
    write_u16(&mut image, file_off + 18, 0x210E); // Characteristics (DLL)

    // Optional header (PE32).
    let opt_off = file_off + 20;
    write_u16(&mut image, opt_off + 0x00, 0x10B);
    write_u32(&mut image, opt_off + 0x04, TEXT_RAW_SIZE); // SizeOfCode
    write_u32(&mut image, opt_off + 0x08, RDATA_RAW_SIZE); // SizeOfInitializedData
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

    // Data directories.
    let data_dir_off = opt_off + 0x60;
    write_u32(&mut image, data_dir_off + 0x00, RDATA_RVA + 0x00); // Export table
    write_u32(&mut image, data_dir_off + 0x04, 0x40);
    write_u32(&mut image, data_dir_off + 0x08, RDATA_RVA + 0x48); // Import table
    write_u32(&mut image, data_dir_off + 0x0C, 0x28);

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
        0x200,
        RDATA_RVA,
        RDATA_RAW_SIZE,
        RDATA_RAW,
        0x4000_0040,
    );

    // .text code: prologue + printf + epilogue.
    let hello_str_rva = RDATA_RVA + 0xA0;
    let iat_rva = RDATA_RVA + 0x78;
    let hello_str_va = IMAGE_BASE + hello_str_rva;
    let iat_va = IMAGE_BASE + iat_rva;

    let mut code = Vec::new();
    code.extend_from_slice(&[0x55, 0x89, 0xE5, 0x83, 0xEC, 0x08]); // push ebp; mov ebp, esp; sub esp,8
    code.push(0xB8);
    code.extend_from_slice(&hello_str_va.to_le_bytes()); // mov eax, imm32
    code.push(0x50); // push eax
    code.push(0xFF);
    code.push(0x15);
    code.extend_from_slice(&iat_va.to_le_bytes()); // call [iat]
    code.extend_from_slice(&[0x83, 0xC4, 0x04]); // add esp, 4
    code.extend_from_slice(&[0x31, 0xC0]); // xor eax, eax
    code.extend_from_slice(&[0xC9, 0xC3]); // leave; ret

    let text_off = TEXT_RAW as usize;
    image[text_off..text_off + code.len()].copy_from_slice(&code);

    // .rdata layout.
    let rdata_off = RDATA_RAW as usize;

    // Export directory.
    write_u32(&mut image, rdata_off + 0x0C, RDATA_RVA + 0x38); // Name
    write_u32(&mut image, rdata_off + 0x10, 1); // Base
    write_u32(&mut image, rdata_off + 0x14, 1); // NumberOfFunctions
    write_u32(&mut image, rdata_off + 0x18, 1); // NumberOfNames
    write_u32(&mut image, rdata_off + 0x1C, RDATA_RVA + 0x28); // AddressOfFunctions
    write_u32(&mut image, rdata_off + 0x20, RDATA_RVA + 0x2C); // AddressOfNames
    write_u32(&mut image, rdata_off + 0x24, RDATA_RVA + 0x30); // AddressOfNameOrdinals

    // Export tables.
    write_u32(&mut image, rdata_off + 0x28, TEXT_RVA); // EAT[0]
    write_u32(&mut image, rdata_off + 0x2C, RDATA_RVA + 0x32); // ENT[0]
    write_u16(&mut image, rdata_off + 0x30, 0); // EOT[0]
    write_bytes(&mut image, rdata_off + 0x32, b"hello\0");
    write_bytes(&mut image, rdata_off + 0x38, b"test.dll\0");

    // Import descriptor.
    write_u32(&mut image, rdata_off + 0x48, RDATA_RVA + 0x70); // OriginalFirstThunk
    write_u32(&mut image, rdata_off + 0x54, RDATA_RVA + 0x90); // Name
    write_u32(&mut image, rdata_off + 0x58, RDATA_RVA + 0x78); // FirstThunk

    // Import tables.
    write_u32(&mut image, rdata_off + 0x70, RDATA_RVA + 0x80); // ILT[0]
    write_u32(&mut image, rdata_off + 0x74, 0); // ILT terminator
    write_u32(&mut image, rdata_off + 0x78, 0); // IAT[0] resolved at runtime
    write_u32(&mut image, rdata_off + 0x7C, 0); // IAT terminator

    // Hint/name and DLL name.
    write_u16(&mut image, rdata_off + 0x80, 0);
    write_bytes(&mut image, rdata_off + 0x82, b"printf\0");
    write_bytes(&mut image, rdata_off + 0x90, b"msvcrt.dll\0");

    // Hello string.
    write_bytes(&mut image, rdata_off + 0xA0, b"Hello, world!\n\0");

    image
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
