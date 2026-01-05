//! Minimal OLEAUT32 stubs for COM automation support.

use crate::vm::{Vm, VmError};

const S_OK: u32 = 0;
const E_INVALIDARG: u32 = 0x8007_0057;
const E_NOTIMPL: u32 = 0x8000_4001;
const DISP_E_TYPEMISMATCH: u32 = 0x8002_0005;

const VARIANT_SIZE: usize = 16;
const VT_EMPTY: u16 = 0;
const VT_I4: u16 = 3;
const VT_BSTR: u16 = 8;
const VT_UI4: u16 = 19;

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
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    S_OK
}

// LoadRegTypeLib(...)
fn load_reg_type_lib(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    if out != 0 {
        let _ = vm.write_u32(out, 0);
    }
    S_OK
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
