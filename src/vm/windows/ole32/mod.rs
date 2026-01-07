//! Minimal OLE32 stubs for COM hosting.

pub const DLL_NAME: &str = "ole32.dll";

use crate::vm::windows::{get_registry, registry::RegistryValue};
use crate::vm::{Vm, VmError};
use crate::vm_args;

use super::guid::{format_guid, parse_guid};

const S_OK: u32 = 0;
const E_INVALIDARG: u32 = 0x8007_0057;
const E_NOTIMPL: u32 = 0x8000_4001;
const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;
const CO_E_CLASSSTRING: u32 = 0x8004_010F;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "CLSIDFromString", crate::vm::stdcall_args(2), clsid_from_string);
    vm.register_import_stdcall(DLL_NAME, "CLSIDFromProgID", crate::vm::stdcall_args(2), clsid_from_progid);
    vm.register_import_stdcall(DLL_NAME, "StringFromGUID2", crate::vm::stdcall_args(3), string_from_guid2);
    vm.register_import_stdcall(DLL_NAME, "CoCreateInstance", crate::vm::stdcall_args(5), co_create_instance);
    vm.register_import_stdcall(DLL_NAME, "CoGetClassObject", crate::vm::stdcall_args(5), co_get_class_object);
    vm.register_import_stdcall(DLL_NAME, "CoTaskMemAlloc", crate::vm::stdcall_args(1), co_task_mem_alloc);
    vm.register_import_stdcall(DLL_NAME, "CoTaskMemRealloc", crate::vm::stdcall_args(2), co_task_mem_realloc);
    vm.register_import_stdcall(DLL_NAME, "CoTaskMemFree", crate::vm::stdcall_args(1), co_task_mem_free);
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateDataAdviseHolder",
        crate::vm::stdcall_args(1),
        create_data_advise_holder,
    );
    vm.register_import_stdcall(DLL_NAME, "ReadClassStm", crate::vm::stdcall_args(2), read_class_stm);
    vm.register_import_stdcall(DLL_NAME, "WriteClassStm", crate::vm::stdcall_args(2), write_class_stm);
    vm.register_import_stdcall(DLL_NAME, "OleInitialize", crate::vm::stdcall_args(1), ole_initialize);
    vm.register_import_stdcall(DLL_NAME, "OleUninitialize", crate::vm::stdcall_args(0), ole_uninitialize);
    vm.register_import_stdcall(DLL_NAME, "OleSaveToStream", crate::vm::stdcall_args(2), ole_save_to_stream);
    vm.register_import_stdcall(DLL_NAME, "OleLockRunning", crate::vm::stdcall_args(3), ole_lock_running);
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateOleAdviseHolder",
        crate::vm::stdcall_args(1),
        create_ole_advise_holder,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "OleRegGetUserType",
        crate::vm::stdcall_args(3),
        ole_reg_get_user_type,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "OleRegGetMiscStatus",
        crate::vm::stdcall_args(3),
        ole_reg_get_misc_status,
    );
    vm.register_import_stdcall(DLL_NAME, "OleRegEnumVerbs", crate::vm::stdcall_args(2), ole_reg_enum_verbs);
    vm.register_import_stdcall(
        DLL_NAME,
        "CreateStreamOnHGlobal",
        crate::vm::stdcall_args(3),
        create_stream_on_hglobal,
    );
}

// CLSIDFromString(lpsz, pclsid)
fn clsid_from_string(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (str_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
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
    let (progid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
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
    let (guid_ptr, out_ptr, max_len) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let max_len = max_len as usize;
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
    let [size] = vm_args!(vm, stack_ptr; u32);
    let buf = vec![0u8; size as usize];
    vm.alloc_bytes(&buf, 8).unwrap_or(0)
}

// CoTaskMemRealloc(ptr, size)
fn co_task_mem_realloc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, size) = vm_args!(vm, stack_ptr; u32, u32);
    let buf = vec![0u8; size as usize];
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
    let (_, _, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

// OleRegGetMiscStatus(...)
fn ole_reg_get_misc_status(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, _, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr != 0 {
        let _ = vm.write_u32(out_ptr, 0);
    }
    S_OK
}

// OleRegEnumVerbs(...)
fn ole_reg_enum_verbs(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_co_create_instance_returns_class_not_available() {
        let mut vm = create_test_vm();
        let result = co_create_instance(&mut vm, 0);
        assert_eq!(result, CLASS_E_CLASSNOTAVAILABLE);
    }

    #[test]
    fn test_co_get_class_object_returns_class_not_available() {
        let mut vm = create_test_vm();
        let result = co_get_class_object(&mut vm, 0);
        assert_eq!(result, CLASS_E_CLASSNOTAVAILABLE);
    }

    #[test]
    fn test_co_task_mem_alloc_returns_pointer() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 100).unwrap(); // size = 100
        let result = co_task_mem_alloc(&mut vm, stack);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_co_task_mem_free_returns_zero() {
        let mut vm = create_test_vm();
        let result = co_task_mem_free(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ole_initialize_returns_s_ok() {
        let mut vm = create_test_vm();
        let result = ole_initialize(&mut vm, 0);
        assert_eq!(result, S_OK);
    }

    #[test]
    fn test_ole_uninitialize_returns_zero() {
        let mut vm = create_test_vm();
        let result = ole_uninitialize(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_clsid_from_string_null_ptr_returns_invalid_arg() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 4, 0).unwrap(); // null string ptr
        vm.write_u32(stack + 8, 0).unwrap(); // null output ptr
        let result = clsid_from_string(&mut vm, stack);
        assert_eq!(result, E_INVALIDARG);
    }

    #[test]
    fn test_string_from_guid2_null_ptr_returns_zero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 4, 0).unwrap();  // null guid ptr
        vm.write_u32(stack + 8, 0).unwrap();  // null output ptr
        vm.write_u32(stack + 12, 0).unwrap(); // max_len = 0
        let result = string_from_guid2(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_read_utf16_z_empty() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write null terminator
        let _ = vm.write_u16(ptr, 0);
        let result = read_utf16_z(&vm, ptr).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_read_guid() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        let guid_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        vm.write_bytes(ptr, &guid_bytes).unwrap();
        let result = read_guid(&vm, ptr).unwrap();
        assert_eq!(result, guid_bytes);
    }
}
