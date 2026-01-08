//! Shared constants for OLEAUT32 stubs.

use crate::vm::Vm;

pub(super) const S_OK: u32 = 0;
pub(super) const E_FAIL: u32 = 0x8000_4005;
pub(super) const E_INVALIDARG: u32 = 0x8007_0057;
pub(super) const E_NOTIMPL: u32 = 0x8000_4001;
pub(super) const E_NOINTERFACE: u32 = 0x8000_4002;
pub(super) const DISP_E_MEMBERNOTFOUND: u32 = 0x8002_0003;
pub(super) const DISP_E_TYPEMISMATCH: u32 = 0x8002_0005;
pub(super) const DISP_E_BADPARAMCOUNT: u32 = 0x8002_000E;
pub(super) const TYPE_E_LIBNOTREGISTERED: u32 = 0x8002_801D;

pub(super) const VARIANT_SIZE: usize = 16;
pub(super) const VT_EMPTY: u16 = 0;
pub(super) const VT_NULL: u16 = 1;
pub(super) const VT_I4: u16 = 3;
pub(super) const VT_BSTR: u16 = 8;
pub(super) const VT_VARIANT: u16 = 12;
pub(super) const VT_I1: u16 = 16;
pub(super) const VT_UI1: u16 = 17;
pub(super) const VT_UI4: u16 = 19;
pub(super) const VT_INT: u16 = 22;
pub(super) const VT_UINT: u16 = 23;
pub(super) const VT_VOID: u16 = 24;
pub(super) const VT_HRESULT: u16 = 25;
pub(super) const VT_ARRAY: u16 = 0x2000;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(S_OK, 0);
        assert_eq!(E_INVALIDARG, 0x8007_0057);
        assert_eq!(E_NOTIMPL, 0x8000_4001);
        assert_eq!(E_NOINTERFACE, 0x8000_4002);
        assert_eq!(DISP_E_MEMBERNOTFOUND, 0x8002_0003);
        assert_eq!(DISP_E_TYPEMISMATCH, 0x8002_0005);
        assert_eq!(DISP_E_BADPARAMCOUNT, 0x8002_000E);
        assert_eq!(TYPE_E_LIBNOTREGISTERED, 0x8002_801D);
    }

    #[test]
    fn test_variant_types() {
        assert_eq!(VT_EMPTY, 0);
        assert_eq!(VT_NULL, 1);
        assert_eq!(VT_I4, 3);
        assert_eq!(VT_BSTR, 8);
        assert_eq!(VT_VARIANT, 12);
        assert_eq!(VT_I1, 16);
        assert_eq!(VT_UI1, 17);
        assert_eq!(VT_UI4, 19);
        assert_eq!(VT_INT, 22);
        assert_eq!(VT_UINT, 23);
        assert_eq!(VT_VOID, 24);
        assert_eq!(VT_HRESULT, 25);
        assert_eq!(VT_ARRAY, 0x2000);
        assert_eq!(VT_BYREF, 0x4000);
        assert_eq!(VT_USERDEFINED, 0x1D);
    }

    #[test]
    fn test_variant_size() {
        assert_eq!(VARIANT_SIZE, 16);
    }

    #[test]
    fn test_param_flags() {
        assert_eq!(PARAMFLAG_FIN, 0x1);
        assert_eq!(PARAMFLAG_FOUT, 0x2);
        assert_eq!(PARAMFLAG_FRETVAL, 0x8);
    }

    #[test]
    fn test_interface_ids() {
        assert_eq!(IID_IUNKNOWN, "{00000000-0000-0000-C000-000000000046}");
        assert_eq!(IID_ITYPELIB, "{00020402-0000-0000-C000-000000000046}");
        assert_eq!(IID_ITYPEINFO, "{00020401-0000-0000-C000-000000000046}");
        assert_eq!(IID_ITYPEINFO2, "{00020412-0000-0000-C000-000000000046}");
    }
}
