use std::fmt;

use crate::pe::PeParseError;

#[derive(Debug)]
pub enum VmError {
    Io(std::io::Error),
    Pe(PeParseError),
    MemoryOutOfRange,
    OutOfMemory,
    DivideError,
    UnsupportedInstruction(u8),
    ExecutionLimit,
    MissingExport(String),
    MissingImports(Vec<String>),
    NoImage,
    InvalidConfig(&'static str),
    Com(u32),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Io(err) => write!(f, "io error: {err}"),
            VmError::Pe(err) => write!(f, "pe error: {err}"),
            VmError::MemoryOutOfRange => write!(f, "memory out of range"),
            VmError::OutOfMemory => write!(f, "out of memory"),
            VmError::DivideError => write!(f, "divide error"),
            VmError::UnsupportedInstruction(op) => write!(f, "unsupported instruction 0x{op:02X}"),
            VmError::ExecutionLimit => write!(f, "execution limit reached"),
            VmError::MissingExport(name) => write!(f, "missing export: {name}"),
            VmError::MissingImports(list) => write!(f, "missing imports: {}", list.join(", ")),
            VmError::NoImage => write!(f, "no image loaded"),
            VmError::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            VmError::Com(code) => {
                if let Some(name) = hresult_name(*code) {
                    write!(f, "com error: 0x{code:08X} ({name})")
                } else {
                    write!(f, "com error: 0x{code:08X}")
                }
            }
        }
    }
}

impl std::error::Error for VmError {}

impl From<std::io::Error> for VmError {
    fn from(err: std::io::Error) -> Self {
        VmError::Io(err)
    }
}

impl From<PeParseError> for VmError {
    fn from(err: PeParseError) -> Self {
        VmError::Pe(err)
    }
}

// Common HRESULT name mapping for clearer COM error output.
fn hresult_name(code: u32) -> Option<&'static str> {
    match code {
        0x80004001 => Some("E_NOTIMPL"),
        0x80004002 => Some("E_NOINTERFACE"),
        0x80004003 => Some("E_POINTER"),
        0x80004004 => Some("E_ABORT"),
        0x80004005 => Some("E_FAIL"),
        0x80070005 => Some("E_ACCESSDENIED"),
        0x8007000E => Some("E_OUTOFMEMORY"),
        0x80070057 => Some("E_INVALIDARG"),
        0x80020003 => Some("DISP_E_MEMBERNOTFOUND"),
        0x80020004 => Some("DISP_E_PARAMNOTFOUND"),
        0x80020005 => Some("DISP_E_TYPEMISMATCH"),
        0x80020006 => Some("DISP_E_UNKNOWNNAME"),
        0x80020007 => Some("DISP_E_NONAMEDARGS"),
        0x80020008 => Some("DISP_E_BADVARTYPE"),
        0x80020009 => Some("DISP_E_EXCEPTION"),
        0x8002000A => Some("DISP_E_OVERFLOW"),
        0x8002000B => Some("DISP_E_BADINDEX"),
        0x8002000C => Some("DISP_E_UNKNOWNLCID"),
        0x8002000D => Some("DISP_E_ARRAYISLOCKED"),
        0x8002000E => Some("DISP_E_BADPARAMCOUNT"),
        0x8002801D => Some("TYPE_E_LIBNOTREGISTERED"),
        0x80040154 => Some("REGDB_E_CLASSNOTREG"),
        0x800401F0 => Some("CO_E_NOTINITIALIZED"),
        0x800401F3 => Some("CO_E_CLASSSTRING"),
        0x80040111 => Some("CLASS_E_CLASSNOTAVAILABLE"),
        _ => None,
    }
}
