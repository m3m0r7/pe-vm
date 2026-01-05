//! Minimal OLEAUT32 stubs for COM automation support.

#[doc(hidden)]
pub mod typelib;

use crate::vm::{ComOutParam, Value, Vm, VmError};
use crate::vm::windows::guid::{format_guid, parse_guid};
use crate::vm::windows::{get_registry, registry::RegistryValue};

const S_OK: u32 = 0;
const E_INVALIDARG: u32 = 0x8007_0057;
const E_NOTIMPL: u32 = 0x8000_4001;
const E_NOINTERFACE: u32 = 0x8000_4002;
const DISP_E_MEMBERNOTFOUND: u32 = 0x8002_0003;
const DISP_E_TYPEMISMATCH: u32 = 0x8002_0005;
const DISP_E_BADPARAMCOUNT: u32 = 0x8002_000E;
const TYPE_E_LIBNOTREGISTERED: u32 = 0x8002_801D;

const VARIANT_SIZE: usize = 16;
const VT_EMPTY: u16 = 0;
const VT_I4: u16 = 3;
const VT_VARIANT: u16 = 12;
const VT_BSTR: u16 = 8;
const VT_UI4: u16 = 19;
const VT_INT: u16 = 22;
const VT_UINT: u16 = 23;
const VT_HRESULT: u16 = 25;
const VT_VOID: u16 = 24;
const VT_BYREF: u16 = 0x4000;
const VT_USERDEFINED: u16 = 0x1D;

type OleMethod = (&'static str, u32, fn(&mut Vm, u32) -> u32);

const PARAMFLAG_FIN: u32 = 0x1;
const PARAMFLAG_FOUT: u32 = 0x2;
const PARAMFLAG_FRETVAL: u32 = 0x8;

const IID_IUNKNOWN: &str = "{00000000-0000-0000-C000-000000000046}";
const IID_ITYPELIB: &str = "{00020402-0000-0000-C000-000000000046}";
const IID_ITYPEINFO: &str = "{00020401-0000-0000-C000-000000000046}";
const IID_ITYPEINFO2: &str = "{00020412-0000-0000-C000-000000000046}";

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

// Allocate a BSTR from UTF-16 units.
pub(crate) fn alloc_bstr_from_utf16(vm: &mut Vm, utf16: &[u16]) -> Result<u32, VmError> {
    let byte_len = (utf16.len() * 2) as u32;
    let mut buf = Vec::with_capacity(4 + utf16.len() * 2 + 2);
    buf.extend_from_slice(&byte_len.to_le_bytes());
    for unit in utf16 {
        buf.extend_from_slice(&unit.to_le_bytes());
    }
    buf.extend_from_slice(&0u16.to_le_bytes());
    let base = vm.alloc_bytes(&buf, 4)?;
    Ok(base + 4)
}

// Allocate a BSTR from a UTF-8 string.
pub(crate) fn alloc_bstr(vm: &mut Vm, text: &str) -> Result<u32, VmError> {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    alloc_bstr_from_utf16(vm, &utf16)
}

// Read a BSTR into a UTF-8 string.
pub(crate) fn read_bstr(vm: &Vm, ptr: u32) -> Result<String, VmError> {
    if ptr == 0 {
        return Ok(String::new());
    }
    if ptr < 4 {
        return Err(VmError::MemoryOutOfRange);
    }
    let byte_len = vm.read_u32(ptr - 4)? as usize;
    let char_len = byte_len / 2;
    let mut utf16 = Vec::with_capacity(char_len);
    for i in 0..char_len {
        utf16.push(vm.read_u16(ptr + (i as u32) * 2)?);
    }
    Ok(String::from_utf16_lossy(&utf16))
}

// Read a null-terminated UTF-16 string from VM memory.
fn read_utf16_z(vm: &Vm, ptr: u32) -> Result<Vec<u16>, VmError> {
    let mut out = Vec::new();
    let mut cursor = ptr;
    for _ in 0..0x10000 {
        let value = vm.read_u16(cursor)?;
        if value == 0 {
            break;
        }
        out.push(value);
        cursor = cursor.wrapping_add(2);
    }
    Ok(out)
}

// Read a UTF-16 string with an explicit length.
fn read_utf16_len(vm: &Vm, ptr: u32, len: usize) -> Result<Vec<u16>, VmError> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(vm.read_u16(ptr + (i as u32) * 2)?);
    }
    Ok(out)
}

// SysAllocString(wstr)
fn sys_alloc_string(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let src = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if src == 0 {
        return 0;
    }
    let utf16 = match read_utf16_z(vm, src) {
        Ok(value) => value,
        Err(_) => return 0,
    };
    alloc_bstr_from_utf16(vm, &utf16).unwrap_or(0)
}

// SysAllocStringLen(wstr, len)
fn sys_alloc_string_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let src = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let len = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let utf16 = if src == 0 {
        vec![0u16; len]
    } else {
        match read_utf16_len(vm, src, len) {
            Ok(value) => value,
            Err(_) => return 0,
        }
    };
    alloc_bstr_from_utf16(vm, &utf16).unwrap_or(0)
}

// SysAllocStringByteLen(ptr, len)
fn sys_alloc_string_byte_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let src = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let len = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    if src == 0 {
        return alloc_bstr_from_utf16(vm, &[]).unwrap_or(0);
    }
    let mut bytes = Vec::with_capacity(len);
    for i in 0..len {
        bytes.push(vm.read_u8(src + i as u32).unwrap_or(0));
    }
    let utf16: Vec<u16> = bytes.iter().map(|value| *value as u16).collect();
    alloc_bstr_from_utf16(vm, &utf16).unwrap_or(0)
}

// SysFreeString(wstr)
fn sys_free_string(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// SysStringLen(wstr)
fn sys_string_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 || ptr < 4 {
        return 0;
    }
    vm.read_u32(ptr - 4).unwrap_or(0) / 2
}

// SysStringByteLen(wstr)
fn sys_string_byte_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 || ptr < 4 {
        return 0;
    }
    vm.read_u32(ptr - 4).unwrap_or(0)
}

// VariantInit(ptr)
fn variant_init(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return E_INVALIDARG;
    }
    let _ = vm.write_bytes(ptr, &[0u8; VARIANT_SIZE]);
    S_OK
}

// VariantClear(ptr)
fn variant_clear(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return E_INVALIDARG;
    }
    let _ = vm.write_bytes(ptr, &[0u8; VARIANT_SIZE]);
    S_OK
}

// VariantChangeType(dest, src, flags, vt)
fn variant_change_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let vt = vm.read_u32(stack_ptr + 16).unwrap_or(0) as u16;
    if dest == 0 || src == 0 {
        return E_INVALIDARG;
    }
    let src_vt = vm.read_u16(src).unwrap_or(VT_EMPTY);
    let result = match (src_vt, vt) {
        (VT_I4, VT_I4) | (VT_UI4, VT_UI4) => {
            let value = vm.read_u32(src + 8).unwrap_or(0);
            write_variant_u32(vm, dest, vt, value)
        }
        (VT_BSTR, VT_I4) | (VT_BSTR, VT_UI4) => {
            let ptr = vm.read_u32(src + 8).unwrap_or(0);
            let text = read_bstr(vm, ptr).unwrap_or_default();
            let parsed = text.trim().parse::<u32>().unwrap_or(0);
            write_variant_u32(vm, dest, vt, parsed)
        }
        _ => Err(VmError::InvalidConfig("variant change type unsupported")),
    };
    match result {
        Ok(()) => S_OK,
        Err(_) => DISP_E_TYPEMISMATCH,
    }
}

// SafeArrayCreate(...)
fn safe_array_create(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// SafeArrayAccessData(...)
fn safe_array_access_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// SafeArrayUnaccessData(...)
fn safe_array_unaccess_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// RegisterTypeLib(...)
fn register_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    S_OK
}

// LoadTypeLib(...)
fn load_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let path_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out == 0 {
        return E_INVALIDARG;
    }
    let text = match read_utf16_z(vm, path_ptr) {
        Ok(units) => String::from_utf16_lossy(&units),
        Err(_) => return E_INVALIDARG,
    };
    let path = text.trim().trim_matches('"');
    if path.is_empty() {
        return E_INVALIDARG;
    }
    match load_typelib_from_path(vm, path, None) {
        Ok(ptr) => {
            let _ = vm.write_u32(out, ptr);
            S_OK
        }
        Err(_) => E_NOTIMPL,
    }
}

// LoadRegTypeLib(...)
fn load_reg_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let guid_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let major = vm.read_u32(stack_ptr + 8).unwrap_or(0) as u16;
    let minor = vm.read_u32(stack_ptr + 12).unwrap_or(0) as u16;
    let out = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if out == 0 {
        return E_INVALIDARG;
    }
    let guid_bytes = match read_guid_bytes(vm, guid_ptr) {
        Some(value) => value,
        None => return E_INVALIDARG,
    };
    let guid = format_guid(&guid_bytes);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] LoadRegTypeLib guid={guid} version={major}.{minor}");
    }
    let path = match resolve_typelib_path(vm, &guid, major, minor) {
        Ok(Some(value)) => value,
        Ok(None) => match vm.image_path() {
            Some(path) => path.to_string(),
            None => return TYPE_E_LIBNOTREGISTERED,
        },
        Err(_) => return E_NOTIMPL,
    };
    match load_typelib_from_path(vm, &path, Some(guid_bytes)) {
        Ok(ptr) => {
            let _ = vm.write_u32(out, ptr);
            S_OK
        }
        Err(_) => E_NOTIMPL,
    }
}

fn load_typelib_from_path(
    vm: &mut Vm,
    path: &str,
    expected_guid: Option<[u8; 16]>,
) -> Result<u32, VmError> {
    let host_path = vm.map_path(path);
    let bytes = std::fs::read(host_path)?;
    let lib = match typelib::load_from_bytes(&bytes) {
        Ok(lib) => lib,
        Err(err) => {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!("[pe_vm] TypeLib load failed: {err}");
            }
            return Err(err);
        }
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] TypeLib loaded guid={}", format_guid(&lib.guid));
    }
    if let Some(expected) = expected_guid {
        if lib.guid != expected {
            return Err(VmError::InvalidConfig("typelib guid mismatch"));
        }
    }
    alloc_typelib(vm, lib)
}

fn resolve_typelib_path(
    vm: &Vm,
    guid: &str,
    major: u16,
    minor: u16,
) -> Result<Option<String>, VmError> {
    let registry =
        get_registry(vm).ok_or(VmError::InvalidConfig("windows registry unavailable"))?;
    let version = format!("{major}.{minor}");
    let candidates = [
        format!(r"HKCR\TypeLib\{guid}\{version}\0\win32"),
        format!(r"HKLM\Software\Classes\TypeLib\{guid}\{version}\0\win32"),
        format!(r"HKCU\Software\Classes\TypeLib\{guid}\{version}\0\win32"),
        format!(
            r"HKLM\Software\Classes\WOW6432Node\TypeLib\{guid}\{version}\0\win32"
        ),
        format!(
            r"HKLM\Software\WOW6432Node\Classes\TypeLib\{guid}\{version}\0\win32"
        ),
    ];
    for key in candidates {
        let value = registry
            .get(&key)
            .map_err(|_| VmError::InvalidConfig("registry key"))?;
        let Some(RegistryValue::String(path)) = value else {
            continue;
        };
        return Ok(Some(path.to_string()));
    }
    Ok(None)
}

const TYPELIB_METHODS: &[OleMethod] = &[
    ("pe_vm.typelib.QueryInterface", 3, typelib_query_interface),
    ("pe_vm.typelib.AddRef", 1, typelib_add_ref),
    ("pe_vm.typelib.Release", 1, typelib_release),
    ("pe_vm.typelib.GetTypeInfoCount", 1, typelib_get_typeinfo_count),
    ("pe_vm.typelib.GetTypeInfo", 3, typelib_get_typeinfo),
    ("pe_vm.typelib.GetTypeInfoType", 3, typelib_get_typeinfo_type),
    ("pe_vm.typelib.GetTypeInfoOfGuid", 3, typelib_get_typeinfo_of_guid),
    ("pe_vm.typelib.GetLibAttr", 2, typelib_not_impl),
    ("pe_vm.typelib.GetTypeComp", 2, typelib_not_impl),
    ("pe_vm.typelib.GetDocumentation", 6, typelib_not_impl),
    ("pe_vm.typelib.IsName", 4, typelib_not_impl),
    ("pe_vm.typelib.FindName", 6, typelib_not_impl),
    ("pe_vm.typelib.ReleaseTLibAttr", 2, typelib_not_impl),
];

const TYPEINFO_METHODS: &[OleMethod] = &[
    ("pe_vm.typeinfo.QueryInterface", 3, typeinfo_query_interface),
    ("pe_vm.typeinfo.AddRef", 1, typeinfo_add_ref),
    ("pe_vm.typeinfo.Release", 1, typeinfo_release),
    ("pe_vm.typeinfo.GetTypeAttr", 2, typeinfo_get_type_attr),
    ("pe_vm.typeinfo.GetTypeComp", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncDesc", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarDesc", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetNames", 5, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetRefTypeOfImplType", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetImplTypeFlags", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetIDsOfNames", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.Invoke", 8, typeinfo_invoke),
    ("pe_vm.typeinfo.GetDocumentation", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetDllEntry", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetRefTypeInfo", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.AddressOfMember", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.CreateInstance", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetMops", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetContainingTypeLib", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.ReleaseTypeAttr", 2, typeinfo_release_type_attr),
    ("pe_vm.typeinfo.ReleaseFuncDesc", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.ReleaseVarDesc", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetTypeKind", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetTypeFlags", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncIndexOfMemId", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarIndexOfMemId", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetParamCustData", 5, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetImplTypeCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetDocumentation2", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllCustData", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllFuncCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllParamCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllVarCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllImplTypeCustData", 3, typeinfo_not_impl),
];

fn alloc_typelib(vm: &mut Vm, lib: typelib::TypeLib) -> Result<u32, VmError> {
    register_typelib_thunks(vm);
    register_typeinfo_thunks(vm);
    let lib_id = typelib::store_typelib(lib);
    let vtable = build_vtable(vm, TYPELIB_METHODS)?;
    build_object(vm, vtable, &[lib_id])
}

fn build_typeinfo_object(vm: &mut Vm, typeinfo_id: u32) -> Result<u32, VmError> {
    let vtable = build_vtable(vm, TYPEINFO_METHODS)?;
    build_object(vm, vtable, &[typeinfo_id])
}

fn build_vtable(vm: &mut Vm, methods: &[OleMethod]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(methods.len() * 4);
    for &(name, _, _) in methods {
        let entry = vm
            .resolve_dynamic_import(name)
            .ok_or(VmError::InvalidConfig("missing import"))?;
        bytes.extend_from_slice(&entry.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn build_object(vm: &mut Vm, vtable_ptr: u32, extras: &[u32]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity((1 + extras.len()) * 4);
    bytes.extend_from_slice(&vtable_ptr.to_le_bytes());
    for extra in extras {
        bytes.extend_from_slice(&extra.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn register_typelib_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPELIB_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn register_typeinfo_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPEINFO_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn typelib_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let iid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let ok = guid_matches(vm, iid_ptr, IID_IUNKNOWN) || guid_matches(vm, iid_ptr, IID_ITYPELIB);
    if ok {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

fn typelib_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typelib_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typelib_get_typeinfo_count(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        return 0;
    };
    lib.typeinfos.len() as u32
}

fn typelib_get_typeinfo(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let index = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if index >= lib.typeinfos.len() {
        let _ = vm.write_u32(out_ptr, 0);
        return E_INVALIDARG;
    }
    let Some(typeinfo_id) = typelib::store_typeinfo(lib_id, index) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        if let Some(info) = lib.typeinfos.get(index) {
            eprintln!(
                "[pe_vm] ITypeLib::GetTypeInfo index={index} guid={}",
                format_guid(&info.guid)
            );
        }
    }
    let typeinfo = build_typeinfo_object(vm, typeinfo_id).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, typeinfo);
    if typeinfo == 0 {
        return E_NOTIMPL;
    }
    S_OK
}

fn typelib_get_typeinfo_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let index = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    let kind = lib
        .typeinfos
        .get(index)
        .map(|info| info.typekind)
        .unwrap_or(0);
    let _ = vm.write_u32(out_ptr, kind);
    S_OK
}

fn typelib_get_typeinfo_of_guid(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let guid_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let guid = match read_guid_bytes(vm, guid_ptr) {
        Some(value) => value,
        None => {
            let _ = vm.write_u32(out_ptr, 0);
            return E_INVALIDARG;
        }
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeLib::GetTypeInfoOfGuid guid={}",
            format_guid(&guid)
        );
    }
    let lib_id = vm.read_u32(this.wrapping_add(4)).unwrap_or(0);
    let Some(lib) = typelib::get_typelib(lib_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    let index = match lib.typeinfos.iter().position(|info| info.guid == guid) {
        Some(value) => value,
        None => {
            let _ = vm.write_u32(out_ptr, 0);
            return E_NOTIMPL;
        }
    };
    let Some(typeinfo_id) = typelib::store_typeinfo(lib_id, index) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        if let Some(info) = lib.typeinfos.get(index) {
            eprintln!(
                "[pe_vm] ITypeLib::GetTypeInfoOfGuid match index={index} guid={}",
                format_guid(&info.guid)
            );
        }
    }
    let typeinfo = build_typeinfo_object(vm, typeinfo_id).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, typeinfo);
    if typeinfo == 0 {
        return E_NOTIMPL;
    }
    S_OK
}

fn typelib_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn typeinfo_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (this, thiscall) = resolve_typeinfo_this(vm, stack_ptr).unwrap_or((0, false));
    let iid_ptr = vm.read_u32(stack_ptr + if thiscall { 4 } else { 8 }).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + if thiscall { 8 } else { 12 }).unwrap_or(0);
    if out_ptr == 0 {
        return E_NOINTERFACE;
    }
    let ok = guid_matches(vm, iid_ptr, IID_IUNKNOWN)
        || guid_matches(vm, iid_ptr, IID_ITYPEINFO)
        || guid_matches(vm, iid_ptr, IID_ITYPEINFO2);
    if ok {
        let _ = vm.write_u32(out_ptr, this);
        return S_OK;
    }
    let _ = vm.write_u32(out_ptr, 0);
    E_NOINTERFACE
}

fn typeinfo_add_ref(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typeinfo_release(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn typeinfo_get_type_attr(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return E_NOTIMPL;
    };
    let out_ptr = vm.read_u32(stack_ptr + if thiscall { 4 } else { 8 }).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let Some(info) = typelib::get_typeinfo(info_id) else {
        let _ = vm.write_u32(out_ptr, 0);
        return E_NOTIMPL;
    };

    let mut bytes = vec![0u8; 0x4C];
    bytes[0..16].copy_from_slice(&info.guid);
    bytes[24..28].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    bytes[28..32].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    bytes[40..44].copy_from_slice(&info.typekind.to_le_bytes());
    bytes[44..46].copy_from_slice(&info.c_funcs.to_le_bytes());
    bytes[46..48].copy_from_slice(&info.c_vars.to_le_bytes());
    bytes[48..50].copy_from_slice(&info.c_impl_types.to_le_bytes());
    bytes[50..52].copy_from_slice(&info.cb_size_vft.to_le_bytes());
    bytes[54..56].copy_from_slice(&(info.flags as u16).to_le_bytes());
    let attr_ptr = vm.alloc_bytes(&bytes, 4).unwrap_or(0);
    if attr_ptr == 0 {
        return E_NOTIMPL;
    }
    let _ = vm.write_u32(out_ptr, attr_ptr);
    S_OK
}

fn typeinfo_invoke(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    let Some(info) = typelib::get_typeinfo(info_id) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    vm.set_last_com_out_params(Vec::new());
    let mut slots = [0u32; 9];
    for (idx, slot) in slots.iter_mut().enumerate() {
        *slot = vm.read_u32(stack_ptr.wrapping_add((idx * 4) as u32)).unwrap_or(0);
    }
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        let mut line = format!("[pe_vm] ITypeInfo::Invoke stack thiscall={thiscall}");
        for (idx, value) in slots.iter().enumerate() {
            line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
        }
        let ecx = vm.reg32(crate::vm::REG_ECX);
        line.push_str(&format!(" ecx=0x{ecx:08X}"));
        eprintln!("{line}");
    }

    #[derive(Clone, Copy)]
    struct InvokeArgs {
        instance: u32,
        memid: u32,
        flags: u16,
        disp_params: u32,
        result_ptr: u32,
        arg_err: u32,
        layout: &'static str,
    }

    let base = if thiscall { 1 } else { 2 };
    let normal = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "normal",
    };
    let no_flags = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "no_flags",
    };
    let swapped_no_flags = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "swapped_no_flags",
    };
    let swapped_normal = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "swapped_normal",
    };

    let mut selected = normal;
    for candidate in [normal, no_flags, swapped_no_flags, swapped_normal] {
        if info.funcs.iter().any(|func| func.memid == candidate.memid) {
            selected = candidate;
            break;
        }
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke layout={} memid=0x{:08X} flags=0x{:04X} disp=0x{:08X}",
            selected.layout,
            selected.memid,
            selected.flags,
            selected.disp_params
        );
    }

    let instance = selected.instance;
    let memid = selected.memid;
    let flags = selected.flags;
    let disp_params = selected.disp_params;
    let result_ptr = selected.result_ptr;
    let arg_err = selected.arg_err;
    let Some(func) = info.funcs.iter().find(|func| func.memid == memid) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke func memid=0x{:08X} params={} vtable=0x{:04X} ret_vt=0x{:04X}",
            func.memid,
            func.params.len(),
            func.vtable_offset,
            func.ret_vt
        );
        eprintln!("[pe_vm] ITypeInfo::Invoke callconv=0x{:X}", func.callconv);
        let vt_list = func
            .params
            .iter()
            .map(|param| format!("0x{:04X}", param.vt))
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!("[pe_vm] ITypeInfo::Invoke param_vt=[{vt_list}]");
    }
    if flags != 0 && func.invkind != 0 && (flags & func.invkind) == 0 {
        return DISP_E_MEMBERNOTFOUND;
    }

    let mut instance = instance;
    let mut disp_params = disp_params;
    if !valid_vtable(vm, instance, func.vtable_offset)
        && valid_vtable(vm, disp_params, func.vtable_offset)
    {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] ITypeInfo::Invoke swapped instance/disp_params");
        }
        std::mem::swap(&mut instance, &mut disp_params);
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() && disp_params != 0 {
        let rgvarg = vm.read_u32(disp_params).unwrap_or(0);
        let cargs = vm.read_u32(disp_params + 8).unwrap_or(0);
        let named = vm.read_u32(disp_params + 12).unwrap_or(0);
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke disp rgvarg=0x{rgvarg:08X} cargs={cargs} named={named}"
        );
    }

    let args_ptr = vm.read_u32(disp_params).unwrap_or(0);
    let arg_count = vm.read_u32(disp_params + 8).unwrap_or(0) as usize;
    let mut input_positions = vec![None; func.params.len()];
    let mut input_count = 0usize;
    for (index, param) in func.params.iter().enumerate() {
        let flags = param.flags;
        if (flags & PARAMFLAG_FRETVAL) != 0 || is_out_only(flags) {
            input_positions[index] = None;
        } else {
            input_positions[index] = Some(input_count);
            input_count += 1;
        }
    }
    let mut positional_fallback = false;
    if arg_count > input_count {
        if arg_count > func.params.len() {
            if arg_err != 0 {
                let _ = vm.write_u32(arg_err, arg_count as u32);
            }
            return DISP_E_BADPARAMCOUNT;
        }
        positional_fallback = true;
    }

    let mut values = Vec::with_capacity(func.params.len() + 1);
    let mut out_params = Vec::new();
    let provided = arg_count.min(input_count);
    let mut retval_param: Option<(usize, u16)> = None;
    for (index, param) in func.params.iter().enumerate() {
        let flags = param.flags;
        let is_retval = (flags & PARAMFLAG_FRETVAL) != 0;
        let out_only = is_out_only(flags);
        let record_out = is_retval || out_only || ((flags & PARAMFLAG_FOUT) != 0 && (param.vt & VT_BYREF) != 0);
        if positional_fallback {
            if index < arg_count {
                let arg_index = arg_count.saturating_sub(1).saturating_sub(index);
                let var_ptr = args_ptr.wrapping_add((arg_index * VARIANT_SIZE) as u32);
                let value = match read_variant_arg(vm, var_ptr, param.vt) {
                    Ok(value) => value,
                    Err(_) => {
                        if arg_err != 0 {
                            let _ = vm.write_u32(arg_err, arg_index as u32);
                        }
                        return DISP_E_TYPEMISMATCH;
                    }
                };
                values.push(Value::U32(value));
                if record_out {
                    out_params.push(ComOutParam {
                        index,
                        vt: param.vt,
                        flags,
                        ptr: value,
                    });
                }
                continue;
            }
            let value = match alloc_out_arg(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return DISP_E_TYPEMISMATCH,
            };
            if is_retval && retval_param.is_none() {
                retval_param = Some((index, param.vt));
            }
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        }
        let Some(position) = input_positions[index] else {
            let value = match alloc_out_arg(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return DISP_E_TYPEMISMATCH,
            };
            if is_retval && retval_param.is_none() {
                retval_param = Some((index, param.vt));
            }
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        };
        if position >= provided {
            let value = match default_arg_for_vt(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return DISP_E_TYPEMISMATCH,
            };
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        }
        let arg_index = provided.saturating_sub(1).saturating_sub(position);
        let var_ptr = args_ptr.wrapping_add((arg_index * VARIANT_SIZE) as u32);
        let value = match read_variant_arg(vm, var_ptr, param.vt) {
            Ok(value) => value,
            Err(_) => {
                if arg_err != 0 {
                    let _ = vm.write_u32(arg_err, arg_index as u32);
                }
                return DISP_E_TYPEMISMATCH;
            }
        };
        values.push(Value::U32(value));
        if record_out {
            out_params.push(ComOutParam {
                index,
                vt: param.vt,
                flags,
                ptr: value,
            });
        }
    }
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        let rendered = values
            .iter()
            .map(|value| match value {
                Value::U32(v) => format!("0x{v:08X}"),
                Value::U64(v) => format!("0x{v:016X}"),
                Value::String(text) => format!("{text:?}"),
                Value::Env(_) => "<env>".to_string(),
            })
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!("[pe_vm] ITypeInfo::Invoke args=[{rendered}]");
        for (index, param) in func.params.iter().enumerate() {
            if (param.vt & !VT_BYREF) != VT_BSTR {
                continue;
            }
            let Some(Value::U32(ptr)) = values.get(index) else {
                continue;
            };
            let mut preview = String::new();
            for i in 0..8u32 {
                let addr = ptr.wrapping_add(i * 2);
                let value = vm.read_u16(addr).unwrap_or(0);
                if i != 0 {
                    preview.push(' ');
                }
                preview.push_str(&format!("{value:04X}"));
            }
            eprintln!(
                "[pe_vm] ITypeInfo::Invoke bstr[{index}] ptr=0x{ptr:08X} utf16={preview}"
            );
        }
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke call instance=0x{instance:08X} vtable_off=0x{:04X}",
            func.vtable_offset
        );
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke instance_in_vm={}",
            vm.contains_addr(instance)
        );
        let instance_vtable = vm.read_u32(instance).unwrap_or(0);
        let dispatch = vm.dispatch_instance().unwrap_or(0);
        let dispatch_vtable = if dispatch == 0 {
            0
        } else {
            vm.read_u32(dispatch).unwrap_or(0)
        };
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable instance=0x{instance_vtable:08X} dispatch=0x{dispatch_vtable:08X}"
        );
    }
    let mut instance_ptr = instance;
    if let Some(dispatch) = vm.dispatch_instance() {
        if valid_vtable(vm, dispatch, func.vtable_offset) {
            instance_ptr = dispatch;
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!(
                    "[pe_vm] ITypeInfo::Invoke dispatch_instance=0x{instance_ptr:08X}"
                );
            }
        }
    }
    if !valid_vtable(vm, instance_ptr, func.vtable_offset) {
        return E_NOTIMPL;
    }
    let entry = match vtable_entry(vm, instance_ptr, func.vtable_offset) {
        Ok(value) => value,
        Err(_) => {
            let fallback = vm.reg32(crate::vm::REG_ECX);
            if fallback != 0 && fallback != instance_ptr {
                instance_ptr = fallback;
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    eprintln!(
                        "[pe_vm] ITypeInfo::Invoke fallback instance=0x{instance_ptr:08X}"
                    );
                }
                vtable_entry(vm, instance_ptr, func.vtable_offset).unwrap_or(0)
            } else {
                0
            }
        }
    };
    if entry == 0 {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] ITypeInfo::Invoke vtable lookup failed");
        }
        return E_NOTIMPL;
    }
    let thiscall_entry = detect_thiscall(vm, entry);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] ITypeInfo::Invoke entry thiscall={thiscall_entry}");
    }
    let result = if thiscall_entry {
        vm.execute_at_with_stack_with_ecx(entry, instance_ptr, &values)
    } else {
        let mut call_args = Vec::with_capacity(values.len() + 1);
        call_args.push(Value::U32(instance_ptr));
        call_args.extend(values.iter().cloned());
        vm.execute_at_with_stack(entry, &call_args)
    };
    let result = match result {
        Ok(value) => value,
        Err(err) => {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!("[pe_vm] ITypeInfo::Invoke call failed: {err}");
            }
            return E_NOTIMPL;
        }
    };
    vm.set_last_com_out_params(out_params);
    let retval_value = retval_param.and_then(|(index, vt)| {
        let Some(Value::U32(ptr)) = values.get(index) else {
            return None;
        };
        read_retval_value(vm, *ptr, vt).ok()
    });
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke returned eax=0x{result:08X} ret_vt=0x{:04X}",
            func.ret_vt
        );
    }
    if result_ptr != 0 && func.ret_vt != VT_VOID && func.ret_vt != VT_EMPTY {
        let result_write = if func.ret_vt == VT_HRESULT {
            if let Some((vt, value)) = retval_value {
                write_variant_value(vm, result_ptr, vt, value)
            } else {
                write_variant_value(vm, result_ptr, VT_I4, result)
            }
        } else {
            write_variant_value(vm, result_ptr, func.ret_vt, result)
        };
        if result_write.is_err() {
            return DISP_E_TYPEMISMATCH;
        }
    }
    if arg_err != 0 {
        let _ = vm.write_u32(arg_err, 0);
    }
    if func.ret_vt == VT_HRESULT {
        return result;
    }
    S_OK
}

fn typeinfo_release_type_attr(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn typeinfo_not_impl(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn is_typeinfo_object(vm: &mut Vm, ptr: u32) -> bool {
    if ptr == 0 {
        return false;
    }
    let vtable_ptr = vm.read_u32(ptr).unwrap_or(0);
    if !vm.contains_addr(vtable_ptr) {
        return false;
    }
    let entry = vm.read_u32(vtable_ptr).unwrap_or(0);
    let Some(expected) = vm.resolve_dynamic_import("pe_vm.typeinfo.QueryInterface") else {
        return false;
    };
    entry == expected
}

fn resolve_typeinfo_this(vm: &mut Vm, stack_ptr: u32) -> Option<(u32, bool)> {
    let stack_this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if stack_this != 0 && is_typeinfo_object(vm, stack_this) {
        return Some((stack_this, false));
    }
    let ecx = vm.reg32(crate::vm::REG_ECX);
    if ecx != 0 && is_typeinfo_object(vm, ecx) {
        return Some((ecx, true));
    }
    None
}

fn resolve_typeinfo_info(vm: &mut Vm, stack_ptr: u32) -> Option<(u32, u32, bool)> {
    let (this, thiscall) = resolve_typeinfo_this(vm, stack_ptr)?;
    let info_id = vm.read_u32(this.wrapping_add(4)).ok()?;
    if typelib::get_typeinfo(info_id).is_some() {
        return Some((this, info_id, thiscall));
    }
    None
}

fn read_guid_bytes(vm: &Vm, ptr: u32) -> Option<[u8; 16]> {
    let mut bytes = [0u8; 16];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr.wrapping_add(idx as u32)).ok()?;
    }
    Some(bytes)
}

fn vtable_entry(vm: &Vm, instance: u32, offset: u16) -> Result<u32, VmError> {
    let vtable_ptr = vm.read_u32(instance)?;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable_ptr=0x{vtable_ptr:08X} in_vm={}",
            vm.contains_addr(vtable_ptr)
        );
    }
    if !vm.contains_addr(vtable_ptr) {
        return Err(VmError::MemoryOutOfRange);
    }
    let entry = vtable_ptr.wrapping_add(offset as u32);
    let value = vm.read_u32(entry)?;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable_entry=0x{entry:08X} fn=0x{value:08X} in_vm={}",
            vm.contains_addr(value)
        );
    }
    Ok(value)
}

fn valid_vtable(vm: &Vm, instance: u32, offset: u16) -> bool {
    let vtable_ptr = vm.read_u32(instance).unwrap_or(0);
    if vtable_ptr == 0 || !vm.contains_addr(vtable_ptr) {
        return false;
    }
    let entry = vm.read_u32(vtable_ptr.wrapping_add(offset as u32)).unwrap_or(0);
    entry != 0 && vm.contains_addr(entry)
}

fn detect_thiscall(vm: &Vm, entry: u32) -> bool {
    let mut bytes = [0u8; 96];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(entry.wrapping_add(idx as u32)).unwrap_or(0);
    }

    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x44 && bytes[idx + 2] == 0x24 && bytes[idx + 3] == 0x04 {
            return false;
        }
    }
    for idx in 0..bytes.len().saturating_sub(2) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x45 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x75 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x4D && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x55 && bytes[idx + 2] == 0x08 {
            return false;
        }
    }

    for idx in 0..bytes.len().saturating_sub(1) {
        let opcode = bytes[idx];
        if !matches!(opcode, 0x8B | 0x89 | 0x8A | 0x8D) {
            continue;
        }
        let modrm = bytes[idx + 1];
        let mod_bits = modrm & 0xC0;
        let rm = modrm & 0x07;
        if mod_bits != 0xC0 && rm == 0x01 {
            return true;
        }
    }
    false
}

fn read_variant_arg(vm: &Vm, var_ptr: u32, expected_vt: u16) -> Result<u32, VmError> {
    if var_ptr == 0 {
        return Err(VmError::InvalidConfig("variant pointer is null"));
    }
    let actual_vt = vm.read_u16(var_ptr)?;
    let value = vm.read_u32(var_ptr + 8)?;
    let expected_base = expected_vt & !VT_BYREF;
    let actual_base = actual_vt & !VT_BYREF;

    if (expected_vt & VT_BYREF) != 0 {
        if (actual_vt & VT_BYREF) == 0 {
            return Err(VmError::InvalidConfig("expected byref variant"));
        }
        return Ok(value);
    }

    if (actual_vt & VT_BYREF) != 0 {
        if value == 0 {
            return Err(VmError::InvalidConfig("null byref pointer"));
        }
        return match actual_base {
            VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED => vm.read_u32(value),
            VT_BSTR => vm.read_u32(value),
            _ => Err(VmError::InvalidConfig("unsupported byref variant")),
        };
    }

    let expected_int = matches!(
        expected_base,
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED
    );
    let actual_int = matches!(
        actual_base,
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED
    );
    if expected_int && actual_int {
        return Ok(value);
    }
    if expected_base == VT_BSTR && actual_base == VT_BSTR {
        return Ok(value);
    }
    Err(VmError::InvalidConfig("variant type mismatch"))
}

fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}

fn alloc_out_arg(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    vm.alloc_bytes(&vec![0u8; size], 4)
}

fn default_arg_for_vt(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    if (vt & VT_BYREF) == 0 && base != VT_USERDEFINED && base != VT_VARIANT {
        return Ok(0);
    }
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    let buffer = vec![0u8; size];
    vm.alloc_bytes(&buffer, 4)
}

fn normalize_retval_vt(vt: u16) -> u16 {
    let base = vt & !VT_BYREF;
    if base == VT_USERDEFINED {
        return VT_I4;
    }
    base
}

fn read_retval_value(vm: &Vm, ptr: u32, vt: u16) -> Result<(u16, u32), VmError> {
    if ptr == 0 {
        return Err(VmError::InvalidConfig("retval pointer is null"));
    }
    let value = vm.read_u32(ptr)?;
    Ok((normalize_retval_vt(vt), value))
}

fn write_variant_value(vm: &mut Vm, dest: u32, vt: u16, value: u32) -> Result<(), VmError> {
    let base_vt = vt & !VT_BYREF;
    match base_vt {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => write_variant_u32(vm, dest, base_vt, value),
        _ => Err(VmError::InvalidConfig("unsupported variant type")),
    }
}

fn guid_matches(vm: &Vm, ptr: u32, guid: &str) -> bool {
    let Some(expected) = parse_guid(guid) else {
        return false;
    };
    let mut actual = [0u8; 16];
    for (idx, slot) in actual.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr.wrapping_add(idx as u32)).unwrap_or(0);
    }
    actual == expected
}

// UnRegisterTypeLib(...)
fn unregister_type_lib(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// SystemTimeToVariantTime(...)
fn system_time_to_variant_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out == 0 {
        return 0;
    }
    let bytes = 0f64.to_le_bytes();
    let _ = vm.write_bytes(out, &bytes);
    1
}

// VariantTimeToSystemTime(...)
fn variant_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out == 0 {
        return 0;
    }
    let _ = vm.write_bytes(out, &[0u8; 16]);
    1
}

// VarUI4FromStr(...)
fn var_ui4_from_str(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let str_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let text = read_bstr(vm, str_ptr).unwrap_or_default();
    match text.trim().parse::<u32>() {
        Ok(value) => {
            let _ = vm.write_u32(out_ptr, value);
            S_OK
        }
        Err(_) => DISP_E_TYPEMISMATCH,
    }
}

// VarBstrCat(...)
fn var_bstr_cat(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let right_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let left = read_bstr(vm, left_ptr).unwrap_or_default();
    let right = read_bstr(vm, right_ptr).unwrap_or_default();
    let combined = format!("{left}{right}");
    let bstr = alloc_bstr(vm, &combined).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, bstr);
    S_OK
}

// OleCreatePropertyFrame(...)
fn ole_create_property_frame(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// OleCreateFontIndirect(...)
fn ole_create_font_indirect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// Write a VARIANT with a 32-bit value.
fn write_variant_u32(vm: &mut Vm, dest: u32, vt: u16, value: u32) -> Result<(), VmError> {
    vm.write_u16(dest, vt)?;
    vm.write_u16(dest + 2, 0)?;
    vm.write_u16(dest + 4, 0)?;
    vm.write_u16(dest + 6, 0)?;
    vm.write_u32(dest + 8, value)?;
    vm.write_u32(dest + 12, 0)?;
    Ok(())
}
