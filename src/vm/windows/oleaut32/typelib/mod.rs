//! MSFT type library parsing and storage.

mod parse;
mod reader;
mod resources;
mod store;

use crate::pe::PeFile;
use crate::vm::VmError;

const MSFT_SIGNATURE: u32 = 0x5446_534D; // "MSFT"
const SLTG_SIGNATURE: u32 = 0x4754_4C53; // "SLTG"

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

pub(crate) fn store_typelib(lib: TypeLib) -> u32 {
    store::store_typelib(lib)
}

pub(crate) fn store_typeinfo(typelib_id: u32, index: usize) -> Option<u32> {
    store::store_typeinfo(typelib_id, index)
}

pub(crate) fn get_typelib(id: u32) -> Option<TypeLib> {
    store::get_typelib(id)
}

pub(crate) fn get_typeinfo(id: u32) -> Option<TypeInfoData> {
    store::get_typeinfo(id)
}

pub(crate) fn load_from_bytes(bytes: &[u8]) -> Result<TypeLib, VmError> {
    if bytes.len() >= 4 {
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic == MSFT_SIGNATURE {
            return parse::parse_msft(bytes);
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
        .and_then(resources::find_typelib_resource);
    let Some(resource) = resource else {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] TYPELIB resource not found in image");
        }
        return Err(VmError::InvalidConfig("TYPELIB resource not found"));
    };
    parse::parse_msft(&resource.data)
}

#[doc(hidden)]
pub fn parse_msft(data: &[u8]) -> Result<TypeLib, VmError> {
    parse::parse_msft(data)
}
