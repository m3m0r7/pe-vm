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

pub(crate) fn cdq(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let eax = vm.reg32(REG_EAX) as i32;
    let edx = if eax < 0 { 0xFFFF_FFFF } else { 0 };
    vm.set_reg32(REG_EDX, edx);
    vm.set_eip(cursor + 1);
    Ok(())
}
