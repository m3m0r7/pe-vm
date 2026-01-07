//! OLE.dll stub registration (Win32::OLE Perl module support).
//!
//! This module provides stub implementations for the Win32::OLE DLL
//! which is commonly used for Perl/COM automation.

pub const DLL_NAME: &str = "OLE.dll";

use crate::vm::windows::check_stub;
use crate::vm::Vm;



// OLE.dll exports (Win32::OLE Perl module)
define_stub_fn!(DLL, create_perl_object, 0);
define_stub_fn!(DLL, set_sv_from_variant, 0);
define_stub_fn!(DLL, set_sv_from_variant_ex, 0);
define_stub_fn!(DLL, set_variant_from_sv, 0);
define_stub_fn!(DLL, boot_win32_ole, 0);

pub fn register(vm: &mut Vm) {
    // Win32::OLE Perl module functions
    vm.register_import(DLL_NAME, "CreatePerlObject", create_perl_object);
    vm.register_import(DLL_NAME, "SetSVFromVariant", set_sv_from_variant);
    vm.register_import(DLL_NAME, "SetSVFromVariantEx", set_sv_from_variant_ex);
    vm.register_import(DLL_NAME, "SetVariantFromSV", set_variant_from_sv);
    vm.register_import(DLL_NAME, "_boot_Win32__OLE", boot_win32_ole);
    vm.register_import(DLL_NAME, "boot_Win32__OLE", boot_win32_ole);
}
