//! Minimal OLEAUT32 stubs for COM automation support.

#[doc(hidden)]
pub mod typelib;

mod bstr;
mod constants;
mod convert;
mod guid;
mod property;
mod safearray;
mod time;
mod typeinfo;
mod typelib_api;
mod variant;

use crate::vm::Vm;

use bstr::{
    sys_alloc_string, sys_alloc_string_byte_len, sys_alloc_string_len, sys_free_string,
    sys_string_byte_len, sys_string_len,
};
use convert::{var_bstr_cat, var_ui4_from_str};
use property::{ole_create_font_indirect, ole_create_property_frame};
use safearray::{safe_array_access_data, safe_array_create, safe_array_unaccess_data};
use time::{system_time_to_variant_time, variant_time_to_system_time};
use typelib_api::{
    load_reg_type_lib, load_type_lib, register_type_lib, unregister_type_lib,
};
use variant::{variant_change_type, variant_clear, variant_init};

pub(crate) use bstr::{alloc_bstr, read_bstr};

pub fn register(vm: &mut Vm) {
    // BSTR helpers.
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 2, 4, sys_alloc_string);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 4, 8, sys_alloc_string_len);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 6, 4, sys_free_string);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 7, 4, sys_string_len);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 149, 4, sys_string_byte_len);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 150, 8, sys_alloc_string_byte_len);

    vm.register_import_stdcall("OLEAUT32.dll", "SysAllocString", crate::vm::stdcall_args(1), sys_alloc_string);
    vm.register_import_stdcall("OLEAUT32.dll", "SysAllocStringLen", crate::vm::stdcall_args(2), sys_alloc_string_len);
    vm.register_import_stdcall("OLEAUT32.dll", "SysFreeString", crate::vm::stdcall_args(1), sys_free_string);
    vm.register_import_stdcall("OLEAUT32.dll", "SysStringLen", crate::vm::stdcall_args(1), sys_string_len);
    vm.register_import_stdcall("OLEAUT32.dll", "SysStringByteLen", crate::vm::stdcall_args(1), sys_string_byte_len);
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "SysAllocStringByteLen",
        crate::vm::stdcall_args(2),
        sys_alloc_string_byte_len,
    );

    // VARIANT helpers.
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 8, 4, variant_init);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 9, 4, variant_clear);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 12, 16, variant_change_type);

    vm.register_import_stdcall("OLEAUT32.dll", "VariantInit", crate::vm::stdcall_args(1), variant_init);
    vm.register_import_stdcall("OLEAUT32.dll", "VariantClear", crate::vm::stdcall_args(1), variant_clear);
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "VariantChangeType",
        crate::vm::stdcall_args(4),
        variant_change_type,
    );

    // SafeArray stubs.
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 15, 12, safe_array_create);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 23, 8, safe_array_access_data);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 24, 4, safe_array_unaccess_data);

    vm.register_import_stdcall("OLEAUT32.dll", "SafeArrayCreate", crate::vm::stdcall_args(3), safe_array_create);
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "SafeArrayAccessData",
        crate::vm::stdcall_args(2),
        safe_array_access_data,
    );
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "SafeArrayUnaccessData",
        crate::vm::stdcall_args(1),
        safe_array_unaccess_data,
    );

    // Type library stubs.
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 161, 8, load_type_lib);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 162, 20, load_reg_type_lib);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 163, 12, register_type_lib);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 186, 20, unregister_type_lib);

    vm.register_import_stdcall("OLEAUT32.dll", "LoadTypeLib", crate::vm::stdcall_args(2), load_type_lib);
    vm.register_import_stdcall("OLEAUT32.dll", "LoadRegTypeLib", crate::vm::stdcall_args(5), load_reg_type_lib);
    vm.register_import_stdcall("OLEAUT32.dll", "RegisterTypeLib", crate::vm::stdcall_args(3), register_type_lib);
    vm.register_import_stdcall("OLEAUT32.dll", "UnRegisterTypeLib", crate::vm::stdcall_args(5), unregister_type_lib);

    // Time and conversion helpers.
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 184, 8, system_time_to_variant_time);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 185, 12, variant_time_to_system_time);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 277, 16, var_ui4_from_str);
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 313, 12, var_bstr_cat);

    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "SystemTimeToVariantTime",
        crate::vm::stdcall_args(2),
        system_time_to_variant_time,
    );
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "VariantTimeToSystemTime",
        crate::vm::stdcall_args(3),
        variant_time_to_system_time,
    );
    vm.register_import_stdcall("OLEAUT32.dll", "VarUI4FromStr", crate::vm::stdcall_args(4), var_ui4_from_str);
    vm.register_import_stdcall("OLEAUT32.dll", "VarBstrCat", crate::vm::stdcall_args(3), var_bstr_cat);

    // UI helpers (stub).
    vm.register_import_ordinal_stdcall(
        "OLEAUT32.dll",
        417,
        44,
        ole_create_property_frame,
    );
    vm.register_import_ordinal_stdcall("OLEAUT32.dll", 420, 12, ole_create_font_indirect);

    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "OleCreatePropertyFrame",
        crate::vm::stdcall_args(11),
        ole_create_property_frame,
    );
    vm.register_import_stdcall(
        "OLEAUT32.dll",
        "OleCreateFontIndirect",
        crate::vm::stdcall_args(3),
        ole_create_font_indirect,
    );
}
