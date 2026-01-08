use crate::vm::{Vm, VmError};

pub(super) fn vtable_entry(vm: &Vm, instance: u32, offset: u16) -> Result<u32, VmError> {
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

pub(super) fn valid_vtable(vm: &Vm, instance: u32, offset: u16) -> bool {
    let vtable_ptr = vm.read_u32(instance).unwrap_or(0);
    if vtable_ptr == 0 || !vm.contains_addr(vtable_ptr) {
        return false;
    }
    let entry = vm
        .read_u32(vtable_ptr.wrapping_add(offset as u32))
        .unwrap_or(0);
    entry != 0 && vm.contains_addr(entry)
}

pub(super) fn detect_thiscall(vm: &Vm, entry: u32) -> bool {
    let mut bytes = [0u8; 160];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(entry.wrapping_add(idx as u32)).unwrap_or(0);
    }

    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx] == 0x8B
            && bytes[idx + 1] == 0x44
            && bytes[idx + 2] == 0x24
            && bytes[idx + 3] == 0x04
        {
            return false;
        }
        if bytes[idx] == 0xFF
            && bytes[idx + 1] == 0x74
            && bytes[idx + 2] == 0x24
            && bytes[idx + 3] == 0x04
        {
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
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x5D && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x7D && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0xFF && bytes[idx + 1] == 0x75 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8D && bytes[idx + 1] == 0x45 && bytes[idx + 2] == 0x08 {
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
