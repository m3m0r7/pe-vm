//! x86 group1 instruction handlers.

use crate::vm::{Vm, VmError};

use super::add;
use super::core::{decode_modrm, Prefixes};
use super::logic;
use super::sub;

pub(crate) fn exec_group1_8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let imm = vm.read_u8(cursor + 1 + modrm.len as u32)?;
    match modrm.reg {
        0 => add::add_rm8_imm(vm, &modrm, prefixes, imm)?,
        1 => logic::or_rm8_imm(vm, &modrm, prefixes, imm)?,
        2 => add::adc_rm8_imm(vm, &modrm, prefixes, imm)?,
        3 => sub::sbb_rm8_imm(vm, &modrm, prefixes, imm)?,
        4 => logic::and_rm8_imm(vm, &modrm, prefixes, imm)?,
        5 => sub::sub_rm8_imm(vm, &modrm, prefixes, imm)?,
        6 => logic::xor_rm8_imm(vm, &modrm, prefixes, imm)?,
        7 => sub::cmp_rm8_imm(vm, &modrm, prefixes, imm)?,
        _ => return Err(VmError::UnsupportedInstruction(0x80)),
    }
    vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
    Ok(())
}

pub(crate) fn exec_group1_32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let opcode = vm.read_u8(cursor)?;
    let imm_size = if opcode == 0x83 { 1 } else { 4 };
    let imm = if imm_size == 1 {
        vm.read_u8(cursor + 1 + modrm.len as u32)? as i8 as i32 as u32
    } else {
        vm.read_u32(cursor + 1 + modrm.len as u32)?
    };
    match modrm.reg {
        0 => add::add_rm32_imm(vm, &modrm, prefixes, imm)?,
        1 => logic::or_rm32_imm(vm, &modrm, prefixes, imm)?,
        2 => add::adc_rm32_imm(vm, &modrm, prefixes, imm)?,
        3 => sub::sbb_rm32_imm(vm, &modrm, prefixes, imm)?,
        4 => logic::and_rm32_imm(vm, &modrm, prefixes, imm)?,
        5 => sub::sub_rm32_imm(vm, &modrm, prefixes, imm)?,
        6 => logic::xor_rm32_imm(vm, &modrm, prefixes, imm)?,
        7 => sub::cmp_rm32_imm(vm, &modrm, prefixes, imm)?,
        _ => return Err(VmError::UnsupportedInstruction(0x81)),
    }
    vm.set_eip(cursor + 1 + modrm.len as u32 + imm_size);
    Ok(())
}
