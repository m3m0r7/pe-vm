//! Minimal OLE32 stubs for COM hosting.

use crate::vm::windows::{get_registry, registry::RegistryValue};
use crate::vm::{Vm, VmError};

use super::guid::{format_guid, parse_guid};

const S_OK: u32 = 0;
const E_INVALIDARG: u32 = 0x8007_0057;
const E_NOTIMPL: u32 = 0x8000_4001;
const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;
const CO_E_CLASSSTRING: u32 = 0x8004_010F;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("ole32.dll", "CLSIDFromString", crate::vm::stdcall_args(2), clsid_from_string);
    vm.register_import_stdcall("ole32.dll", "CLSIDFromProgID", crate::vm::stdcall_args(2), clsid_from_progid);
    vm.register_import_stdcall("ole32.dll", "StringFromGUID2", crate::vm::stdcall_args(3), string_from_guid2);
    vm.register_import_stdcall("ole32.dll", "CoCreateInstance", crate::vm::stdcall_args(5), co_create_instance);
    vm.register_import_stdcall("ole32.dll", "CoGetClassObject", crate::vm::stdcall_args(5), co_get_class_object);
    vm.register_import_stdcall("ole32.dll", "CoTaskMemAlloc", crate::vm::stdcall_args(1), co_task_mem_alloc);
    vm.register_import_stdcall("ole32.dll", "CoTaskMemRealloc", crate::vm::stdcall_args(2), co_task_mem_realloc);
    vm.register_import_stdcall("ole32.dll", "CoTaskMemFree", crate::vm::stdcall_args(1), co_task_mem_free);
    vm.register_import_stdcall(
        "ole32.dll",
        "CreateDataAdviseHolder",
        crate::vm::stdcall_args(1),
        create_data_advise_holder,
    );
    vm.register_import_stdcall("ole32.dll", "ReadClassStm", crate::vm::stdcall_args(2), read_class_stm);
    vm.register_import_stdcall("ole32.dll", "WriteClassStm", crate::vm::stdcall_args(2), write_class_stm);
    vm.register_import_stdcall("ole32.dll", "OleInitialize", crate::vm::stdcall_args(1), ole_initialize);
    vm.register_import_stdcall("ole32.dll", "OleUninitialize", crate::vm::stdcall_args(0), ole_uninitialize);
    vm.register_import_stdcall("ole32.dll", "OleSaveToStream", crate::vm::stdcall_args(2), ole_save_to_stream);
    vm.register_import_stdcall("ole32.dll", "OleLockRunning", crate::vm::stdcall_args(3), ole_lock_running);
    vm.register_import_stdcall(
        "ole32.dll",
        "CreateOleAdviseHolder",
        crate::vm::stdcall_args(1),
        create_ole_advise_holder,
    );
    vm.register_import_stdcall(
        "ole32.dll",
        "OleRegGetUserType",
        crate::vm::stdcall_args(3),
        ole_reg_get_user_type,
    );
    vm.register_import_stdcall(
        "ole32.dll",
        "OleRegGetMiscStatus",
        crate::vm::stdcall_args(3),
        ole_reg_get_misc_status,
    );
    vm.register_import_stdcall("ole32.dll", "OleRegEnumVerbs", crate::vm::stdcall_args(2), ole_reg_enum_verbs);
    vm.register_import_stdcall(
        "ole32.dll",
        "CreateStreamOnHGlobal",
        crate::vm::stdcall_args(3),
        create_stream_on_hglobal,
    );
}

// CLSIDFromString(lpsz, pclsid)
fn clsid_from_string(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let str_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if str_ptr == 0 || out_ptr == 0 {
        return E_INVALIDARG;
    }
    let text = read_utf16_z(vm, str_ptr).unwrap_or_default();
    let text = String::from_utf16_lossy(&text);
    let Some(bytes) = parse_guid(&text) else {
        return CO_E_CLASSSTRING;
    };
    if vm.write_bytes(out_ptr, &bytes).is_err() {
        return E_INVALIDARG;
    }
    S_OK
}

// CLSIDFromProgID(lpszProgID, pclsid)
fn clsid_from_progid(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let progid_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if progid_ptr == 0 || out_ptr == 0 {
        return E_INVALIDARG;
    }
    let progid = read_utf16_z(vm, progid_ptr).unwrap_or_default();
    let progid = String::from_utf16_lossy(&progid);
    let Some(registry) = get_registry(vm) else {
        return CLASS_E_CLASSNOTAVAILABLE;
    };
    let key = format!(r"HKCR\{progid}\CLSID");
    let clsid = match registry.get(&key) {
        Ok(Some(RegistryValue::String(value))) => value,
        _ => return CLASS_E_CLASSNOTAVAILABLE,
    };
    let Some(bytes) = parse_guid(clsid) else {
        return CLASS_E_CLASSNOTAVAILABLE;
    };
    if vm.write_bytes(out_ptr, &bytes).is_err() {
        return E_INVALIDARG;
    }
    S_OK
}

// StringFromGUID2(rguid, lpsz, cchMax)
fn string_from_guid2(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let guid_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let max_len = vm.read_u32(stack_ptr + 12).unwrap_or(0) as usize;
    if guid_ptr == 0 || out_ptr == 0 || max_len == 0 {
        return 0;
    }
    let guid = match read_guid(vm, guid_ptr) {
        Ok(value) => value,
        Err(_) => return 0,
    };
    let text = format_guid(&guid);
    let utf16: Vec<u16> = text.encode_utf16().collect();
    if utf16.len() + 1 > max_len {
        return 0;
    }
    for (i, unit) in utf16.iter().enumerate() {
        let _ = vm.write_u16(out_ptr + (i as u32) * 2, *unit);
    }
    let _ = vm.write_u16(out_ptr + (utf16.len() as u32) * 2, 0);
    (utf16.len() + 1) as u32
}

// CoCreateInstance(...)
fn co_create_instance(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    CLASS_E_CLASSNOTAVAILABLE
}

// CoGetClassObject(...)
fn co_get_class_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    CLASS_E_CLASSNOTAVAILABLE
}

// CoTaskMemAlloc(size)
fn co_task_mem_alloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let size = vm.read_u32(stack_ptr + 4).unwrap_or(0) as usize;
    let buf = vec![0u8; size];
    vm.alloc_bytes(&buf, 8).unwrap_or(0)
}

// CoTaskMemRealloc(ptr, size)
fn co_task_mem_realloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let size = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    let buf = vec![0u8; size];
    vm.alloc_bytes(&buf, 8).unwrap_or(0)
}

// CoTaskMemFree(ptr)
fn co_task_mem_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// CreateDataAdviseHolder(...)
fn create_data_advise_holder(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// ReadClassStm(...)
fn read_class_stm(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// WriteClassStm(...)
fn write_class_stm(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// OleInitialize(...)
fn ole_initialize(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// OleUninitialize()
fn ole_uninitialize(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// OleSaveToStream(...)
fn ole_save_to_stream(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// OleLockRunning(...)
fn ole_lock_running(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// CreateOleAdviseHolder(...)
fn create_ole_advise_holder(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// OleRegGetUserType(...)
fn ole_reg_get_user_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

// OleRegGetMiscStatus(...)
fn ole_reg_get_misc_status(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

// OleRegEnumVerbs(...)
fn ole_reg_enum_verbs(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

// CreateStreamOnHGlobal(...)
fn create_stream_on_hglobal(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// Read a null-terminated UTF-16 string.
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

// Read GUID bytes from VM memory.
fn read_guid(vm: &Vm, ptr: u32) -> Result<[u8; 16], VmError> {
    let mut out = [0u8; 16];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = vm.read_u8(ptr + i as u32)?;
    }
    Ok(out)
}
