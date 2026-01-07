//! x86 add/adc instruction handlers.

use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use super::core::{
    decode_modrm, read_rm32, read_rm8, update_flags_add32, update_flags_add8, write_rm32,
    write_rm8, ModRm, Prefixes,
};

pub(crate) fn add_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg8(modrm.reg);
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = lhs.wrapping_add(rhs);
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_add8(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn add_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg32(modrm.reg);
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = lhs.wrapping_add(rhs);
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_add32(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn add_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg8(modrm.reg);
    let result = lhs.wrapping_add(rhs);
    vm.set_reg8(modrm.reg, result);
    update_flags_add8(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn add_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg32(modrm.reg);
    let result = lhs.wrapping_add(rhs);
    vm.set_reg32(modrm.reg, result);
    update_flags_add32(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn add_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let lhs = vm.reg8(REG_AL);
    let result = lhs.wrapping_add(imm);
    vm.set_reg8(REG_AL, result);
    update_flags_add8(vm, lhs, imm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn add_eax_imm32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let lhs = vm.reg32(REG_EAX);
    let result = lhs.wrapping_add(imm);
    vm.set_reg32(REG_EAX, result);
    update_flags_add32(vm, lhs, imm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn adc_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg8(modrm.reg);
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = adc8(vm, lhs, rhs);
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn adc_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg32(modrm.reg);
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = adc32(vm, lhs, rhs);
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn adc_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg8(modrm.reg);
    let result = adc8(vm, lhs, rhs);
    vm.set_reg8(modrm.reg, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn adc_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg32(modrm.reg);
    let result = adc32(vm, lhs, rhs);
    vm.set_reg32(modrm.reg, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn adc_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let lhs = vm.reg8(REG_AL);
    let result = adc8(vm, lhs, imm);
    vm.set_reg8(REG_AL, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn adc_eax_imm32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let lhs = vm.reg32(REG_EAX);
    let result = adc32(vm, lhs, imm);
    vm.set_reg32(REG_EAX, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn inc_reg(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x40;
    let value = vm.reg32(reg);
    let result = value.wrapping_add(1);
    vm.set_reg32(reg, result);
    update_flags_add32(vm, value, 1, result);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn inc_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_add(1);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_add32(vm, value, 1, result);
    Ok(())
}

pub(crate) fn inc_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_add(1);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_add8(vm, value, 1, result);
    Ok(())
}

pub(crate) fn add_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_add(imm);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_add32(vm, dst, imm, result);
    Ok(())
}

pub(crate) fn add_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_add(imm);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_add8(vm, dst, imm, result);
    Ok(())
}

pub(crate) fn adc_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = adc32(vm, dst, imm);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}

pub(crate) fn adc_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = adc8(vm, dst, imm);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}

fn adc8(vm: &mut Vm, a: u8, b: u8) -> u8 {
    let carry = if vm.cf() { 1u16 } else { 0 };
    let sum = a as u16 + b as u16 + carry;
    let result = sum as u8;
    let cf = sum > 0xFF;
    let b_with_carry = b.wrapping_add(carry as u8);
    let sign = 0x80;
    let of = ((a ^ result) & (b_with_carry ^ result) & sign) != 0;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    vm.set_flags(zf, sf, of, cf);
    result
}

fn adc32(vm: &mut Vm, a: u32, b: u32) -> u32 {
    let carry = if vm.cf() { 1u64 } else { 0 };
    let sum = a as u64 + b as u64 + carry;
    let result = sum as u32;
    let cf = sum > 0xFFFF_FFFF;
    let b_with_carry = b.wrapping_add(carry as u32);
    let sign = 0x8000_0000;
    let of = ((a ^ result) & (b_with_carry ^ result) & sign) != 0;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    vm.set_flags(zf, sf, of, cf);
    result
}
