//! Shared constants for OLEAUT32 stubs.

use crate::vm::Vm;

pub(super) const S_OK: u32 = 0;
pub(super) const E_INVALIDARG: u32 = 0x8007_0057;
pub(super) const E_NOTIMPL: u32 = 0x8000_4001;
pub(super) const E_NOINTERFACE: u32 = 0x8000_4002;
pub(super) const DISP_E_MEMBERNOTFOUND: u32 = 0x8002_0003;
pub(super) const DISP_E_TYPEMISMATCH: u32 = 0x8002_0005;
pub(super) const DISP_E_BADPARAMCOUNT: u32 = 0x8002_000E;
pub(super) const TYPE_E_LIBNOTREGISTERED: u32 = 0x8002_801D;

pub(super) const VARIANT_SIZE: usize = 16;
pub(super) const VT_EMPTY: u16 = 0;
pub(super) const VT_I4: u16 = 3;
pub(super) const VT_BSTR: u16 = 8;
pub(super) const VT_VARIANT: u16 = 12;
pub(super) const VT_UI4: u16 = 19;
pub(super) const VT_INT: u16 = 22;
pub(super) const VT_UINT: u16 = 23;
pub(super) const VT_VOID: u16 = 24;
pub(super) const VT_HRESULT: u16 = 25;
pub(super) const VT_BYREF: u16 = 0x4000;
pub(super) const VT_USERDEFINED: u16 = 0x1D;

pub(super) type OleMethod = (&'static str, u32, fn(&mut Vm, u32) -> u32);

pub(super) const PARAMFLAG_FIN: u32 = 0x1;
pub(super) const PARAMFLAG_FOUT: u32 = 0x2;
pub(super) const PARAMFLAG_FRETVAL: u32 = 0x8;

pub(super) const IID_IUNKNOWN: &str = "{00000000-0000-0000-C000-000000000046}";
pub(super) const IID_ITYPELIB: &str = "{00020402-0000-0000-C000-000000000046}";
pub(super) const IID_ITYPEINFO: &str = "{00020401-0000-0000-C000-000000000046}";
pub(super) const IID_ITYPEINFO2: &str = "{00020412-0000-0000-C000-000000000046}";
