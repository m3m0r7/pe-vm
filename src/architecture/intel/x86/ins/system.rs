//! x86 system instruction handlers.

use crate::vm::{Vm, VmError, REG_AL, REG_EAX, REG_EBX, REG_ECX, REG_EDX};

use super::core::Prefixes;

pub(crate) fn nop(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn int3(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn int(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn cpuid(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    match vm.reg32(REG_EAX) {
        0 => {
            vm.set_reg32(REG_EAX, 1);
            vm.set_reg32(REG_ECX, 0x756e_6547);
            vm.set_reg32(REG_EDX, 0x6c65_746e);
            vm.set_reg32(REG_EBX, 0x4965_6e69);
        }
        _ => {
            vm.set_reg32(REG_EAX, 0);
            vm.set_reg32(REG_ECX, 0);
            vm.set_reg32(REG_EDX, 0);
            vm.set_reg32(REG_EBX, 0);
        }
    }
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn xgetbv(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = vm.read_u8(cursor + 2)?;
    if modrm != 0xD0 {
        return Err(VmError::UnsupportedInstruction(0x01));
    }
    vm.set_reg32(REG_EAX, 0);
    vm.set_reg32(REG_EDX, 0);
    vm.set_eip(cursor + 3);
    Ok(())
}

pub(crate) fn salc(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let value = if vm.cf() { 0xFF } else { 0x00 };
    vm.set_reg8(REG_AL, value);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn cld(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_df(false);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn std(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_df(true);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn cdq(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let eax = vm.reg32(REG_EAX) as i32;
    let edx = if eax < 0 { 0xFFFF_FFFF } else { 0 };
    vm.set_reg32(REG_EDX, edx);
    vm.set_eip(cursor + 1);
    Ok(())
}

/// OUT DX, AL (EE) - Output byte to I/O port
/// In VM, this is a no-op as we don't support I/O ports
pub(crate) fn out_dx_al(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_eip(cursor + 1);
    Ok(())
}

/// OUT DX, EAX (EF) - Output dword to I/O port
/// In VM, this is a no-op as we don't support I/O ports
pub(crate) fn out_dx_eax(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_eip(cursor + 1);
    Ok(())
}

/// IN AL, DX (EC) - Input byte from I/O port
/// In VM, returns 0 as we don't support I/O ports
pub(crate) fn in_al_dx(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_reg8(REG_AL, 0);
    vm.set_eip(cursor + 1);
    Ok(())
}

/// IN EAX, DX (ED) - Input dword from I/O port
/// In VM, returns 0 as we don't support I/O ports
pub(crate) fn in_eax_dx(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    vm.set_reg32(REG_EAX, 0);
    vm.set_eip(cursor + 1);
    Ok(())
}

/// OUTSB (6E) - Output string byte to I/O port
/// In VM, this is a no-op as we don't support I/O ports
pub(crate) fn outsb(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    use crate::vm::{REG_ECX, REG_ESI};
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let esi = vm.reg32(REG_ESI);
    let delta = if vm.df() { count.wrapping_neg() } else { count };
    // Just advance ESI by count bytes (we don't actually output anything)
    vm.set_reg32(REG_ESI, esi.wrapping_add(delta));
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

/// OUTSD (6F) - Output string dword to I/O port
/// In VM, this is a no-op as we don't support I/O ports
pub(crate) fn outsd(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    use crate::vm::{REG_ECX, REG_ESI};
    let size = if prefixes.operand_size_16 { 2u32 } else { 4u32 };
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let esi = vm.reg32(REG_ESI);
    let delta = count.wrapping_mul(size);
    let delta = if vm.df() {
        delta.wrapping_neg()
    } else {
        delta
    };
    // Just advance ESI by count * size bytes (we don't actually output anything)
    vm.set_reg32(REG_ESI, esi.wrapping_add(delta));
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

/// INSB (6C) - Input string byte from I/O port
/// In VM, fills with zeros as we don't support I/O ports
pub(crate) fn insb(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    use crate::vm::{REG_ECX, REG_EDI};
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut edi = vm.reg32(REG_EDI);
    // Fill destination with zeros
    for _ in 0..count {
        vm.write_u8(edi, 0)?;
        if vm.df() {
            edi = edi.wrapping_sub(1);
        } else {
            edi = edi.wrapping_add(1);
        }
    }
    vm.set_reg32(REG_EDI, edi);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

/// INSD (6D) - Input string dword from I/O port
/// In VM, fills with zeros as we don't support I/O ports
pub(crate) fn insd(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    use crate::vm::{REG_ECX, REG_EDI};
    let size = if prefixes.operand_size_16 { 2u32 } else { 4u32 };
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut edi = vm.reg32(REG_EDI);
    // Fill destination with zeros
    for _ in 0..count {
        if size == 2 {
            vm.write_u16(edi, 0)?;
            if vm.df() {
                edi = edi.wrapping_sub(2);
            } else {
                edi = edi.wrapping_add(2);
            }
        } else {
            vm.write_u32(edi, 0)?;
            if vm.df() {
                edi = edi.wrapping_sub(4);
            } else {
                edi = edi.wrapping_add(4);
            }
        }
    }
    vm.set_reg32(REG_EDI, edi);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}
