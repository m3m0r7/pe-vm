// Validate MSFT typelib parsing for a minimal dispatch interface.
use pe_vm::windows::oleaut32::typelib::parse_msft;

const MSFT_SIGNATURE: u32 = 0x5446_534D;

fn write_u16(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn build_minimal_msft() -> Vec<u8> {
    const HEADER_SIZE: usize = 0x54;
    const SEGDIR_SIZE: usize = 15 * 16;
    const TYPEINFO_SIZE: usize = 0x64;
    const GUID_ENTRY_SIZE: usize = 0x18;
    const NR_TYPEINFOS: usize = 1;

    let segdir_offset = HEADER_SIZE + NR_TYPEINFOS * 4;
    let typeinfo_offset = segdir_offset + SEGDIR_SIZE;
    let guid_offset = typeinfo_offset + TYPEINFO_SIZE;
    let mem_offset = guid_offset + GUID_ENTRY_SIZE;
    let total_size = mem_offset + 0x38;

    let mut data = vec![0u8; total_size];

    write_u32(&mut data, 0, MSFT_SIGNATURE);
    write_u32(&mut data, 4, 0x0001_0002);
    write_u32(&mut data, 0x08, 0); // posguid
    write_u32(&mut data, 0x14, 0); // varflags
    write_u32(&mut data, 0x20, NR_TYPEINFOS as u32);

    // Segment directory entries (typeinfo tab, guid tab, typdesc tab).
    write_u32(&mut data, segdir_offset, typeinfo_offset as u32);
    write_u32(&mut data, segdir_offset + 4, TYPEINFO_SIZE as u32);
    write_u32(&mut data, segdir_offset + 12, 0x0F);

    let guid_seg = segdir_offset + 5 * 16;
    write_u32(&mut data, guid_seg, guid_offset as u32);
    write_u32(&mut data, guid_seg + 4, GUID_ENTRY_SIZE as u32);
    write_u32(&mut data, guid_seg + 12, 0x0F);

    let typdesc_seg = segdir_offset + 10 * 16;
    write_u32(&mut data, typdesc_seg, 0);
    write_u32(&mut data, typdesc_seg + 4, 0);

    // TypeInfo base (dispatch, one function, guid at index 0).
    write_u32(&mut data, typeinfo_offset, 4);
    write_u32(&mut data, typeinfo_offset + 4, mem_offset as u32);
    write_u32(&mut data, typeinfo_offset + 0x18, 1);
    write_u32(&mut data, typeinfo_offset + 0x2C, 0);
    write_u32(&mut data, typeinfo_offset + 0x30, 0);

    // Guid entry bytes (16 bytes) followed by hreftype/next.
    for idx in 0..16 {
        data[guid_offset + idx] = (idx + 1) as u8;
    }

    // Member info layout for one function.
    let info_len = 0x24u32;
    write_u32(&mut data, mem_offset, info_len);

    let record_offset = mem_offset + 4;
    write_u32(&mut data, record_offset, info_len);
    write_u32(&mut data, record_offset + 4, 0x8000_0003); // VT_I4
    write_u32(&mut data, record_offset + 8, 0);
    write_u16(&mut data, record_offset + 12, 0x10);
    write_u16(&mut data, record_offset + 14, 0);
    write_u32(&mut data, record_offset + 16, 1 | (1 << 3) | (4 << 8));
    write_u16(&mut data, record_offset + 20, 1);
    write_u16(&mut data, record_offset + 22, 0);

    // Param info at end of record (BSTR).
    let param_offset = record_offset + 0x18;
    write_u32(&mut data, param_offset, 0x8000_0008); // VT_BSTR
    write_u32(&mut data, param_offset + 4, 0);
    write_u32(&mut data, param_offset + 8, 0);

    // Member ID and name arrays (member id at index 1).
    let memid_offset = mem_offset + info_len as usize;
    write_u32(&mut data, memid_offset, 0);
    write_u32(&mut data, memid_offset + 4, 0x1234);
    write_u32(&mut data, memid_offset + 8, 0);
    write_u32(&mut data, memid_offset + 12, 0);

    data
}

#[test]
fn parse_msft_minimal_dispatch() {
    let data = build_minimal_msft();
    let lib = parse_msft(&data).expect("parse msft");

    assert_eq!(lib.typeinfos.len(), 1);
    let info = &lib.typeinfos[0];
    assert_eq!(info.c_funcs, 1);
    assert_eq!(info.funcs.len(), 1);

    let func = &info.funcs[0];
    assert_eq!(func.memid, 0x1234);
    assert_eq!(func.ret_vt, 3);
    assert_eq!(func.params.len(), 1);
    assert_eq!(func.params[0].vt, 8);
}
