//! x86 stack instruction handlers.

use crate::vm::{Vm, VmError, REG_EBP, REG_ESP};

use super::core::{decode_modrm, pack_eflags, read_rm32, write_rm32, ModRm, Prefixes};

pub(crate) fn push_reg(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x50;
    let value = vm.reg32(reg);
    vm.push(value)?;
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn pop_reg(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x58;
    let value = vm.pop()?;
    vm.set_reg32(reg, value);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn push_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)? as i8 as i32 as u32;
    vm.push(imm)?;
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn push_imm32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    vm.push(imm)?;
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn pop_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if modrm.reg != 0 {
        return Err(VmError::UnsupportedInstruction(0x8F));
    }
    let value = vm.pop()?;
    write_rm32(vm, &modrm, prefixes.segment_base, value)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn push_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    vm.push(value)?;
    Ok(())
}

pub(crate) fn pushfd(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let flags = pack_eflags(vm);
    vm.push(flags)?;
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn leave(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let ebp = vm.reg32(REG_EBP);
    vm.set_reg32(REG_ESP, ebp);
    let value = vm.pop()?;
    vm.set_reg32(REG_EBP, value);
    vm.set_eip(cursor + 1);
    Ok(())
}
