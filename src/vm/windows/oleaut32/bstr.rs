//! BSTR helpers.

use crate::vm::{Vm, VmError};
use crate::vm_args;

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
pub(super) fn read_utf16_z(vm: &Vm, ptr: u32) -> Result<Vec<u16>, VmError> {
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
pub(super) fn sys_alloc_string(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (src,) = vm_args!(vm, stack_ptr; u32);
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
pub(super) fn sys_alloc_string_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (src, len) = vm_args!(vm, stack_ptr; u32, u32);
    let len = len as usize;
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
pub(super) fn sys_alloc_string_byte_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (src, len) = vm_args!(vm, stack_ptr; u32, u32);
    let len = len as usize;
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
pub(super) fn sys_free_string(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// SysStringLen(wstr)
pub(super) fn sys_string_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 || ptr < 4 {
        return 0;
    }
    vm.read_u32(ptr - 4).unwrap_or(0) / 2
}

// SysStringByteLen(wstr)
pub(super) fn sys_string_byte_len(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 || ptr < 4 {
        return 0;
    }
    vm.read_u32(ptr - 4).unwrap_or(0)
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
    fn test_alloc_bstr() {
        let mut vm = create_test_vm();
        let ptr = alloc_bstr(&mut vm, "Hello").unwrap();
        assert_ne!(ptr, 0);
        // Read back the BSTR
        let text = read_bstr(&vm, ptr).unwrap();
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_alloc_bstr_empty() {
        let mut vm = create_test_vm();
        let ptr = alloc_bstr(&mut vm, "").unwrap();
        assert_ne!(ptr, 0);
        let text = read_bstr(&vm, ptr).unwrap();
        assert_eq!(text, "");
    }

    #[test]
    fn test_read_bstr_null() {
        let vm = create_test_vm();
        let text = read_bstr(&vm, 0).unwrap();
        assert_eq!(text, "");
    }

    #[test]
    fn test_read_bstr_invalid_ptr() {
        let vm = create_test_vm();
        // ptr < 4 should return error
        let result = read_bstr(&vm, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_alloc_bstr_from_utf16() {
        let mut vm = create_test_vm();
        let utf16: Vec<u16> = "Test".encode_utf16().collect();
        let ptr = alloc_bstr_from_utf16(&mut vm, &utf16).unwrap();
        assert_ne!(ptr, 0);
        let text = read_bstr(&vm, ptr).unwrap();
        assert_eq!(text, "Test");
    }

    #[test]
    fn test_read_utf16_z() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write "AB" in UTF-16 with null terminator
        vm.write_u16(ptr, 0x0041).unwrap();     // 'A'
        vm.write_u16(ptr + 2, 0x0042).unwrap(); // 'B'
        vm.write_u16(ptr + 4, 0).unwrap();      // null
        let result = read_utf16_z(&vm, ptr).unwrap();
        assert_eq!(result, vec![0x0041, 0x0042]);
    }

    #[test]
    fn test_read_utf16_z_empty() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_u16(ptr, 0).unwrap(); // just null
        let result = read_utf16_z(&vm, ptr).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_sys_alloc_string_null() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap(); // null source
        let result = sys_alloc_string(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sys_free_string() {
        let mut vm = create_test_vm();
        let result = sys_free_string(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sys_string_len_null() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = sys_string_len(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sys_string_len() {
        let mut vm = create_test_vm();
        let bstr = alloc_bstr(&mut vm, "Hello").unwrap();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, bstr).unwrap();
        let result = sys_string_len(&mut vm, stack);
        assert_eq!(result, 5); // 5 characters
    }

    #[test]
    fn test_sys_string_byte_len_null() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = sys_string_byte_len(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sys_string_byte_len() {
        let mut vm = create_test_vm();
        let bstr = alloc_bstr(&mut vm, "Hello").unwrap();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, bstr).unwrap();
        let result = sys_string_byte_len(&mut vm, stack);
        assert_eq!(result, 10); // 5 chars * 2 bytes
    }
}
