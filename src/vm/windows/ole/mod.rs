//! OLE.dll stub registration (Win32::OLE Perl module support).
//!
//! This module provides stub implementations for the Win32::OLE DLL
//! which is commonly used for Perl/COM automation.

pub const DLL_NAME: &str = "OLE.dll";

use crate::vm::windows::check_stub;
use crate::vm::Vm;

macro_rules! stub {
    ($name:ident) => {
        fn $name(vm: &mut Vm, _sp: u32) -> u32 {
            check_stub(vm, DLL_NAME, stringify!($name));
            0
        }
    };
}

// OLE.dll exports (Win32::OLE Perl module)
stub!(create_perl_object);
stub!(set_sv_from_variant);
stub!(set_sv_from_variant_ex);
stub!(set_variant_from_sv);
stub!(boot_win32_ole);

pub fn register(vm: &mut Vm) {
    // Win32::OLE Perl module functions
    vm.register_import(DLL_NAME, "CreatePerlObject", create_perl_object);
    vm.register_import(DLL_NAME, "SetSVFromVariant", set_sv_from_variant);
    vm.register_import(DLL_NAME, "SetSVFromVariantEx", set_sv_from_variant_ex);
    vm.register_import(DLL_NAME, "SetVariantFromSV", set_variant_from_sv);
    vm.register_import(DLL_NAME, "_boot_Win32__OLE", boot_win32_ole);
    vm.register_import(DLL_NAME, "boot_Win32__OLE", boot_win32_ole);
}
