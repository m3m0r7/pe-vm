//! MSFT type library parsing and storage.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::pe::PeFile;
use crate::pe::{ResourceData, ResourceDirectory, ResourceId, ResourceNode};
use crate::vm::VmError;

const MSFT_SIGNATURE: u32 = 0x5446_534D; // "MSFT"
const SLTG_SIGNATURE: u32 = 0x4754_4C53; // "SLTG"
const HELPDLLFLAG: u32 = 0x0100;
const TYPELIB_RESOURCE_ID: u32 = 6;
const TYPEINFO_SIZE: usize = 0x64;
const TKIND_ALIAS: u32 = 6;
const VT_TYPEMASK: u32 = 0x0FFF;
const VT_PTR: u16 = 0x1A;
const VT_USERDEFINED: u16 = 0x1D;
const VT_BYREF: u16 = 0x4000;

#[derive(Debug, Clone)]
pub struct TypeLib {
    pub guid: [u8; 16],
    pub typeinfos: Vec<TypeInfoData>,
}

#[derive(Debug, Clone)]
pub struct TypeInfoData {
    pub guid: [u8; 16],
    pub typekind: u32,
    pub c_funcs: u16,
    pub c_vars: u16,
    pub c_impl_types: u16,
    pub cb_size_vft: u16,
    pub flags: u32,
    pub funcs: Vec<FuncDesc>,
}

#[derive(Debug, Clone)]
pub struct FuncDesc {
    pub memid: u32,
    pub invkind: u16,
    pub callconv: u16,
    pub vtable_offset: u16,
    pub ret_vt: u16,
    pub params: Vec<ParamDesc>,
}

#[derive(Debug, Clone)]
pub struct ParamDesc {
    pub vt: u16,
    pub flags: u32,
}

#[derive(Debug, Clone)]
struct TypeDescEntry {
    data: i32,
    vt: u16,
}

#[derive(Debug, Clone)]
struct TypeInfoHandle {
    typelib_id: u32,
    index: usize,
}

#[derive(Default)]
struct TypeLibStore {
    next_id: u32,
    typelibs: HashMap<u32, TypeLib>,
    typeinfos: HashMap<u32, TypeInfoHandle>,
}

fn store() -> &'static Mutex<TypeLibStore> {
    static STORE: OnceLock<Mutex<TypeLibStore>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(TypeLibStore { next_id: 1, ..TypeLibStore::default() }))
}

pub(crate) fn store_typelib(lib: TypeLib) -> u32 {
    let mut guard = store().lock().expect("typelib store");
    let id = guard.next_id;
    guard.next_id = guard.next_id.wrapping_add(1);
    guard.typelibs.insert(id, lib);
    id
}

pub(crate) fn store_typeinfo(typelib_id: u32, index: usize) -> Option<u32> {
    let mut guard = store().lock().expect("typelib store");
    guard.typelibs.get(&typelib_id)?;
    let id = guard.next_id;
    guard.next_id = guard.next_id.wrapping_add(1);
    guard.typeinfos.insert(id, TypeInfoHandle { typelib_id, index });
    Some(id)
}

pub(crate) fn get_typelib(id: u32) -> Option<TypeLib> {
    let guard = store().lock().expect("typelib store");
    guard.typelibs.get(&id).cloned()
}

pub(crate) fn get_typeinfo(id: u32) -> Option<TypeInfoData> {
    let guard = store().lock().expect("typelib store");
    let handle = guard.typeinfos.get(&id)?;
    guard
        .typelibs
        .get(&handle.typelib_id)
        .and_then(|lib| lib.typeinfos.get(handle.index).cloned())
}

pub(crate) fn load_from_bytes(bytes: &[u8]) -> Result<TypeLib, VmError> {
    if bytes.len() >= 4 {
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic == MSFT_SIGNATURE {
            return parse_msft(bytes);
        }
        if magic == SLTG_SIGNATURE {
            return Err(VmError::InvalidConfig("SLTG typelibs not supported"));
        }
    }
    let pe = PeFile::parse(bytes)?;
    let resource = pe
        .directories
        .resource
        .as_ref()
        .and_then(find_typelib_resource);
    let Some(resource) = resource else {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] TYPELIB resource not found in image");
        }
        return Err(VmError::InvalidConfig("TYPELIB resource not found"));
    };
    parse_msft(&resource.data)
}

fn find_typelib_resource(dir: &ResourceDirectory) -> Option<&ResourceData> {
    let node = dir.roots.iter().find(|node| matches_typelib_id(&node.id))?;
    find_first_resource(node)
}

fn matches_typelib_id(id: &ResourceId) -> bool {
    match id {
        ResourceId::Id(value) => *value == TYPELIB_RESOURCE_ID,
        ResourceId::Name(name) => name.eq_ignore_ascii_case("TYPELIB"),
    }
}

fn find_first_resource(node: &ResourceNode) -> Option<&ResourceData> {
    if let Some(data) = node.data.as_ref() {
        return Some(data);
    }
    for child in &node.children {
        if let Some(data) = find_first_resource(child) {
            return Some(data);
        }
    }
    None
}

#[doc(hidden)]
pub fn parse_msft(data: &[u8]) -> Result<TypeLib, VmError> {
    let reader = Reader::new(data);
    let magic = reader.read_u32(0)?;
    if magic != MSFT_SIGNATURE {
        return Err(VmError::InvalidConfig("invalid MSFT signature"));
    }
    let varflags = reader.read_u32(0x14)?;
    let nrtypeinfos = reader.read_u32(0x20)? as usize;
    let posguid = reader.read_i32(0x08)?;
    let segdir_offset = 0x54usize
        + nrtypeinfos
            .checked_mul(4)
            .ok_or(VmError::InvalidConfig("typelib count overflow"))?
        + if (varflags & HELPDLLFLAG) != 0 { 4 } else { 0 };

    let segdir = SegDir::read(&reader, segdir_offset)?;
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
            let vt = resolve_vartype(param_type, type_descs, aliases)?;
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
        return Ok((data_type as u32 & VT_TYPEMASK) as u16);
    }
    let idx = (data_type as usize)
        .checked_div(8)
        .ok_or(VmError::InvalidConfig("typelib type index"))?;
    let Some(entry) = type_descs.get(idx) else {
        return Ok(VT_USERDEFINED);
    };
    match entry.vt {
        VT_PTR => {
            let target = resolve_vartype(entry.data, type_descs, aliases)?;
            Ok(target | VT_BYREF)
        }
        VT_USERDEFINED => {
            if (entry.data & 3) == 0 {
                let index = (entry.data as usize) / TYPEINFO_SIZE;
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    let alias = aliases.get(index).and_then(|value| *value);
                    eprintln!(
                        "[pe_vm] typelib userdefined hreftype=0x{:08X} index={index} alias={alias:?}",
                        entry.data
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
        let data = reader.read_i32(base)?;
        let vt_raw = reader.read_u32(base + 4)?;
        let vt = (vt_raw & 0xFFFF) as u16;
        out.push(TypeDescEntry { data, vt });
    }
    Ok(out)
}

fn read_guid(reader: &Reader, segdir: &SegDir, offset: i32) -> Option<[u8; 16]> {
    if offset < 0 {
        return None;
    }
    let base = segdir.guid_tab.offset as usize + offset as usize;
    let mut guid = [0u8; 16];
    for (index, slot) in guid.iter_mut().enumerate() {
        *slot = reader.read_u8(base + index).ok()?;
    }
    Some(guid)
}

#[derive(Debug, Clone)]
struct SegEntry {
    offset: u32,
    length: u32,
}

#[derive(Debug, Clone)]
struct SegDir {
    typeinfo_tab: SegEntry,
    guid_tab: SegEntry,
    typdesc_tab: SegEntry,
}

impl SegDir {
    fn read(reader: &Reader, offset: usize) -> Result<Self, VmError> {
        let typeinfo_tab = SegEntry {
            offset: reader.read_u32(offset)?,
            length: reader.read_u32(offset + 4)?,
        };
        let guid_tab = SegEntry {
            offset: reader.read_u32(offset + 5 * 16)?,
            length: reader.read_u32(offset + 5 * 16 + 4)?,
        };
        let typdesc_tab = SegEntry {
            offset: reader.read_u32(offset + 10 * 16)?,
            length: reader.read_u32(offset + 10 * 16 + 4)?,
        };
        Ok(Self {
            typeinfo_tab,
            guid_tab,
            typdesc_tab,
        })
    }
}

#[derive(Clone)]
struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn read_u8(&self, offset: usize) -> Result<u8, VmError> {
        self.data.get(offset).copied().ok_or(VmError::MemoryOutOfRange)
    }

    fn read_u16(&self, offset: usize) -> Result<u16, VmError> {
        if offset + 2 > self.data.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u16::from_le_bytes([self.data[offset], self.data[offset + 1]]))
    }

    fn read_u32(&self, offset: usize) -> Result<u32, VmError> {
        if offset + 4 > self.data.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ]))
    }

    fn read_i32(&self, offset: usize) -> Result<i32, VmError> {
        Ok(self.read_u32(offset)? as i32)
    }
}
