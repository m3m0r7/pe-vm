use crate::vm::VmError;

use super::{FuncDesc, ParamDesc, TypeInfoData, TypeLib};
use super::super::constants::{
    PARAMFLAG_FOUT, PARAMFLAG_FRETVAL, VT_BSTR, VT_I1, VT_I4, VT_INT, VT_UI1, VT_UI4, VT_UINT,
};
use super::reader::{Reader, SegDir, SegEntry};
use super::MSFT_SIGNATURE;

const HELPDLLFLAG: u32 = 0x0100;
const TYPEINFO_SIZE: usize = 0x64;
const TKIND_ALIAS: u32 = 6;
const VT_TYPEMASK: u32 = 0x0FFF;
const VT_PTR: u16 = 0x1A;
const VT_USERDEFINED: u16 = 0x1D;
const VT_BYREF: u16 = 0x4000;

#[derive(Debug, Clone)]
struct TypeDescEntry {
    data: i32,
    vt: u16,
}

pub(super) fn parse_msft(data: &[u8]) -> Result<TypeLib, VmError> {
    let reader = Reader::new(data);
    let magic = reader.read_u32(0)?;
    if magic != MSFT_SIGNATURE {
        return Err(VmError::InvalidConfig("invalid MSFT signature"));
    }
    let varflags = reader.read_u32(0x14)?;
    let nrtypeinfos = reader.read_u32(0x20)? as usize;
    let posguid = reader.read_i32(0x08)?;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] typelib header varflags=0x{varflags:08X} nrtypeinfos={nrtypeinfos} posguid=0x{posguid:08X}"
        );
        let target = [
            0x40, 0x77, 0xB1, 0x2A, 0x41, 0x0C, 0xD7, 0x11, 0x91, 0x6F, 0x00, 0x03, 0x47,
            0x9B, 0xEB, 0x3F,
        ];
        if data.len() >= target.len() {
            for idx in 0..=data.len().saturating_sub(target.len()) {
                if data[idx..idx + target.len()] == target {
                    eprintln!(
                        "[pe_vm] typelib guid bytes found at 0x{idx:08X}"
                    );
                    break;
                }
            }
        }
    }
    let segdir_offset = 0x54usize
        + nrtypeinfos
            .checked_mul(4)
            .ok_or(VmError::InvalidConfig("typelib count overflow"))?
        + if (varflags & HELPDLLFLAG) != 0 { 4 } else { 0 };

    let mut segdir = SegDir::read(&reader, segdir_offset)?;
    if !guid_table_looks_valid(&reader, &segdir, posguid) {
        if let Some(guess) = guess_guid_tab(&reader, segdir_offset, posguid) {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!(
                    "[pe_vm] typelib guid table adjusted offset=0x{:X} len=0x{:X}",
                    guess.offset, guess.length
                );
            }
            segdir.guid_tab = guess;
        }
    }
    if segdir.guid_tab.offset == segdir.typdesc_tab.offset
        && segdir.guid_tab.length == segdir.typdesc_tab.length
    {
        if let Some(guess) = guess_typdesc_tab(&reader, segdir_offset, &segdir.guid_tab) {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!(
                    "[pe_vm] typelib typdesc table adjusted offset=0x{:X} len=0x{:X}",
                    guess.offset, guess.length
                );
            }
            segdir.typdesc_tab = guess;
        }
    }
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] typelib segdir typeinfo offset=0x{:X} len=0x{:X} guid offset=0x{:X} len=0x{:X} typdesc offset=0x{:X} len=0x{:X}",
            segdir.typeinfo_tab.offset,
            segdir.typeinfo_tab.length,
            segdir.guid_tab.offset,
            segdir.guid_tab.length,
            segdir.typdesc_tab.offset,
            segdir.typdesc_tab.length
        );
    }
    let type_descs = read_typdesc_table(&reader, &segdir)?;

    let mut aliases = vec![None; nrtypeinfos];
    for (index, slot) in aliases.iter_mut().enumerate() {
        let entry_offset = segdir.typeinfo_tab.offset as usize
            + index
                .checked_mul(TYPEINFO_SIZE)
                .ok_or(VmError::InvalidConfig("typeinfo offset overflow"))?;
        let typekind = reader.read_u32(entry_offset)? & 0xF;
        if typekind != TKIND_ALIAS {
            continue;
        }
        let datatype1 = reader.read_i32(entry_offset + 0x54)?;
        if let Ok(vt) = resolve_vartype_basic(datatype1, &type_descs) {
            *slot = Some(vt);
        }
    }

    let mut typeinfos = Vec::with_capacity(nrtypeinfos);
    for index in 0..nrtypeinfos {
        let entry_offset = segdir.typeinfo_tab.offset as usize
            + index
                .checked_mul(TYPEINFO_SIZE)
                .ok_or(VmError::InvalidConfig("typeinfo offset overflow"))?;
        let typeinfo = parse_typeinfo(&reader, &segdir, &type_descs, &aliases, entry_offset)?;
        typeinfos.push(typeinfo);
    }
    let guid = read_guid(&reader, &segdir, posguid).unwrap_or([0u8; 16]);
    Ok(TypeLib { guid, typeinfos })
}

fn parse_typeinfo(
    reader: &Reader,
    segdir: &SegDir,
    type_descs: &[TypeDescEntry],
    aliases: &[Option<u16>],
    offset: usize,
) -> Result<TypeInfoData, VmError> {
    let typekind = reader.read_u32(offset)?;
    let memoffset = reader.read_i32(offset + 4)?;
    let c_element = reader.read_u32(offset + 0x18)?;
    let posguid = reader.read_i32(offset + 0x2C)?;
    let flags = reader.read_u32(offset + 0x30)?;
    let c_funcs = (c_element & 0xFFFF) as u16;
    let c_vars = (c_element >> 16) as u16;
    let (c_impl_types, cb_size_vft) = (
        reader.read_u16(offset + 0x4C)?,
        reader.read_u16(offset + 0x4E)?,
    );
    let guid = read_guid(reader, segdir, posguid).unwrap_or([0u8; 16]);
    let funcs = if c_funcs > 0 && memoffset > 0 {
        parse_funcs(
            reader,
            memoffset as usize,
            c_funcs,
            c_vars,
            type_descs,
            aliases,
        )?
    } else {
        Vec::new()
    };
    Ok(TypeInfoData {
        guid,
        typekind: typekind & 0xF,
        c_funcs,
        c_vars,
        c_impl_types,
        cb_size_vft,
        flags,
        funcs,
    })
}

fn parse_funcs(
    reader: &Reader,
    offset: usize,
    c_funcs: u16,
    _c_vars: u16,
    type_descs: &[TypeDescEntry],
    aliases: &[Option<u16>],
) -> Result<Vec<FuncDesc>, VmError> {
    let infolen = reader.read_u32(offset)? as usize;
    let mut recoffset = offset + 4;
    let mut funcs = Vec::with_capacity(c_funcs as usize);
    for i in 0..c_funcs as usize {
        let info = reader.read_u32(recoffset)?;
        let reclength = (info & 0xFFFF) as usize;
        let data_type = reader.read_i32(recoffset + 4)?;
        let _flags = reader.read_u32(recoffset + 8)?;
        let vtable_offset = reader.read_u16(recoffset + 12)?;
        let fkccic = reader.read_u32(recoffset + 16)?;
        let nrargs = reader.read_u16(recoffset + 20)? as usize;
        let _nroargs = reader.read_u16(recoffset + 22)? as usize;

        let memid_offset = offset
            .checked_add(infolen)
            .and_then(|value| value.checked_add((i + 1) * 4))
            .ok_or(VmError::InvalidConfig("typelib memid overflow"))?;
        let memid = reader.read_u32(memid_offset)?;

        let invkind = ((fkccic >> 3) & 0xF) as u16;
        let callconv = ((fkccic >> 8) & 0xF) as u16;
        let ret_vt = resolve_vartype(data_type, type_descs, aliases)?;
        if std::env::var("PE_VM_TRACE_COM").is_ok() && matches!(memid, 0x3 | 0x4 | 0x7 | 0xD) {
            eprintln!(
                "[pe_vm] typelib raw func memid=0x{memid:08X} ret_type=0x{data_type:08X} invkind=0x{invkind:X}"
            );
        }

        let mut params = Vec::with_capacity(nrargs);
        let params_offset = recoffset
            .checked_add(reclength)
            .and_then(|value| value.checked_sub(nrargs * 12))
            .ok_or(VmError::InvalidConfig("typelib param overflow"))?;
        for index in 0..nrargs {
            let base = params_offset + index * 12;
            let param_type = reader.read_i32(base)?;
            let _name = reader.read_i32(base + 4)?;
            let flags = reader.read_u32(base + 8)?;
            if std::env::var("PE_VM_TRACE_COM").is_ok() && matches!(memid, 0x3 | 0x4 | 0x7 | 0xD) {
                eprintln!(
                    "[pe_vm] typelib raw param memid=0x{memid:08X} index={index} type=0x{param_type:08X} flags=0x{flags:08X}"
                );
            }
            let mut vt = resolve_vartype(param_type, type_descs, aliases)?;
            if param_type >= 0 {
                let raw = param_type as u32;
                if raw == 0 && (flags & (PARAMFLAG_FOUT | PARAMFLAG_FRETVAL)) != 0 {
                    vt = VT_I4;
                } else if matches!(raw as u16, VT_BSTR | VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_UI1 | VT_I1) {
                    vt = raw as u16;
                }
            }
            if (flags & (PARAMFLAG_FOUT | PARAMFLAG_FRETVAL)) != 0 && (vt & VT_BYREF) == 0 {
                vt |= VT_BYREF;
            }
            if std::env::var("PE_VM_TRACE_COM").is_ok() && matches!(memid, 0x3 | 0x4 | 0x7 | 0xD) {
                eprintln!(
                    "[pe_vm] typelib resolved param memid=0x{memid:08X} index={index} vt=0x{vt:04X}"
                );
            }
            params.push(ParamDesc { vt, flags });
        }

        funcs.push(FuncDesc {
            memid,
            invkind,
            callconv,
            vtable_offset: vtable_offset & !1,
            ret_vt,
            params,
        });
        recoffset = recoffset
            .checked_add(reclength)
            .ok_or(VmError::InvalidConfig("typelib record overflow"))?;
    }
    Ok(funcs)
}

fn resolve_vartype_basic(data_type: i32, type_descs: &[TypeDescEntry]) -> Result<u16, VmError> {
    resolve_vartype(data_type, type_descs, &[])
}

fn resolve_vartype(
    data_type: i32,
    type_descs: &[TypeDescEntry],
    aliases: &[Option<u16>],
) -> Result<u16, VmError> {
    if data_type < 0 {
        let vt = (data_type as u32 & VT_TYPEMASK) as u16;
        if vt == VT_USERDEFINED && std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!(
                "[pe_vm] typelib resolve_vartype immediate userdefined data_type=0x{data_type:08X}"
            );
        }
        return Ok(vt);
    }
    let raw = data_type as u32;
    if type_descs.is_empty() && raw <= VT_TYPEMASK {
        return Ok(raw as u16);
    }
    let idx = (data_type as usize)
        .checked_div(8)
        .ok_or(VmError::InvalidConfig("typelib type index"))?;
    let Some(entry) = type_descs.get(idx) else {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!(
                "[pe_vm] typelib resolve_vartype missing typedesc index={idx} data_type=0x{data_type:08X}"
            );
        }
        return Ok(VT_USERDEFINED);
    };
    match entry.vt {
        VT_PTR => {
            let target = resolve_vartype(entry.data, type_descs, aliases)?;
            Ok(target | VT_BYREF)
        }
        VT_USERDEFINED => {
            let hreftype = entry.data;
            let cleaned = hreftype & !3;
            if cleaned >= 0 {
                let index = (cleaned as usize) / TYPEINFO_SIZE;
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    let alias = aliases.get(index).and_then(|value| *value);
                    eprintln!(
                        "[pe_vm] typelib userdefined hreftype=0x{:08X} cleaned=0x{:08X} index={index} alias={alias:?}",
                        hreftype, cleaned
                    );
                }
                if let Some(Some(vt)) = aliases.get(index) {
                    return Ok(*vt);
                }
            }
            Ok(VT_USERDEFINED)
        }
        _ => Ok(entry.vt),
    }
}

fn read_typdesc_table(reader: &Reader, segdir: &SegDir) -> Result<Vec<TypeDescEntry>, VmError> {
    let mut out = Vec::new();
    let offset = segdir.typdesc_tab.offset as usize;
    let length = segdir.typdesc_tab.length as usize;
    if offset == 0 || length == 0 {
        return Ok(out);
    }
    let count = length / 8;
    for idx in 0..count {
        let base = offset + idx * 8;
        let data_raw = reader.read_u32(base)?;
        let data = data_raw as i32;
        let vt_raw = reader.read_u32(base + 4)?;
        let vt = (vt_raw & 0xFFFF) as u16;
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!(
                "[pe_vm] typelib typdesc[{idx}] data=0x{data_raw:08X} vt_raw=0x{vt_raw:08X} vt=0x{vt:04X}"
            );
        }
        out.push(TypeDescEntry { data, vt });
    }
    Ok(out)
}

fn read_guid(reader: &Reader, segdir: &SegDir, offset: i32) -> Option<[u8; 16]> {
    if offset < 0 {
        return None;
    }
    let base = segdir.guid_tab.offset as usize + offset as usize;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        let mut bytes = [0u8; 16];
        for (index, slot) in bytes.iter_mut().enumerate() {
            *slot = reader.read_u8(base + index).ok().unwrap_or(0);
        }
        eprintln!(
            "[pe_vm] typelib guid raw @0x{base:08X}: {:02X?}",
            bytes
        );
    }
    let mut guid = [0u8; 16];
    for (index, slot) in guid.iter_mut().enumerate() {
        *slot = reader.read_u8(base + index).ok()?;
    }
    Some(guid)
}

fn guid_table_looks_valid(reader: &Reader, segdir: &SegDir, posguid: i32) -> bool {
    if segdir.guid_tab.offset == 0xFFFF_FFFF || segdir.guid_tab.length < 16 || posguid < 0 {
        return false;
    }
    let base = segdir.guid_tab.offset as usize + posguid as usize;
    let mut bytes = [0u8; 16];
    for (index, slot) in bytes.iter_mut().enumerate() {
        *slot = reader.read_u8(base + index).unwrap_or(0);
    }
    let all_zero = bytes.iter().all(|b| *b == 0);
    let all_ff = bytes.iter().all(|b| *b == 0xFF);
    let zero_ff = bytes.iter().filter(|b| **b == 0 || **b == 0xFF).count();
    !(all_zero || all_ff || zero_ff >= 12)
}

fn guess_guid_tab(reader: &Reader, segdir_offset: usize, posguid: i32) -> Option<SegEntry> {
    if posguid < 0 {
        return None;
    }
    let entry_size = 8usize;
    let mut best: Option<(SegEntry, usize)> = None;
    for idx in 0..12usize {
        let base = segdir_offset + idx * entry_size;
        let seg_offset = reader.read_u32(base).ok()?;
        let seg_length = reader.read_u32(base + 4).ok()?;
        if seg_offset == 0xFFFF_FFFF || seg_length < 16 {
            continue;
        }
        let posguid = posguid as u32;
        if posguid as u64 + 16 > seg_length as u64 {
            continue;
        }
        let base = seg_offset as usize + posguid as usize;
        let mut bytes = [0u8; 16];
        for (index, slot) in bytes.iter_mut().enumerate() {
            *slot = reader.read_u8(base + index).unwrap_or(0);
        }
        let all_zero = bytes.iter().all(|b| *b == 0);
        let all_ff = bytes.iter().all(|b| *b == 0xFF);
        let zero_ff = bytes.iter().filter(|b| **b == 0 || **b == 0xFF).count();
        if all_zero || all_ff {
            continue;
        }
        let entry = SegEntry {
            offset: seg_offset,
            length: seg_length,
        };
        if best
            .as_ref()
            .map(|(_, score)| zero_ff < *score)
            .unwrap_or(true)
        {
            best = Some((entry, zero_ff));
        }
    }
    best.map(|(entry, _)| entry)
}

fn guess_typdesc_tab(
    reader: &Reader,
    segdir_offset: usize,
    guid_tab: &SegEntry,
) -> Option<SegEntry> {
    let entry_size = 8usize;
    let mut best: Option<(SegEntry, f32)> = None;
    for idx in 0..12usize {
        let base = segdir_offset + idx * entry_size;
        let seg_offset = reader.read_u32(base).ok()?;
        let seg_length = reader.read_u32(base + 4).ok()?;
        if seg_offset == 0xFFFF_FFFF || seg_length == 0 || seg_length % 8 != 0 {
            continue;
        }
        if seg_offset == guid_tab.offset && seg_length == guid_tab.length {
            continue;
        }
        let entry_count = (seg_length / 8) as usize;
        if entry_count == 0 {
            continue;
        }
        let mut plausible = 0usize;
        let mut vt_preview = Vec::new();
        for i in 0..entry_count {
            let vt_raw = reader
                .read_u32(seg_offset as usize + i * 8 + 4)
                .unwrap_or(0);
            let vt = (vt_raw & 0xFFFF) as u16;
            if vt_preview.len() < 4 {
                vt_preview.push(vt);
            }
            if vt <= 0x30 || vt == 0x1A || vt == 0x1D {
                plausible += 1;
            }
        }
        let ratio = plausible as f32 / entry_count as f32;
        let entry = SegEntry {
            offset: seg_offset,
            length: seg_length,
        };
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!(
                "[pe_vm] typelib typdesc candidate offset=0x{seg_offset:08X} len=0x{seg_length:08X} ratio={ratio:.2} vt0={:?}",
                vt_preview
            );
        }
        if best
            .as_ref()
            .map(|(_, score)| ratio > *score)
            .unwrap_or(true)
        {
            best = Some((entry, ratio));
        }
    }
    best.map(|(entry, _)| entry)
}
