//! x86 logic and test instruction handlers.

use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use super::core::{
    decode_modrm, read_rm32, read_rm8, update_flags_logic32, update_flags_logic8, write_rm32,
    write_rm8, ModRm, Prefixes,
};

pub(crate) fn or_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg32(modrm.reg);
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = lhs | rhs;
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn or_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg8(modrm.reg);
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = lhs | rhs;
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn or_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg32(modrm.reg);
    let result = lhs | rhs;
    vm.set_reg32(modrm.reg, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn or_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg8(modrm.reg);
    let result = lhs | rhs;
    vm.set_reg8(modrm.reg, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn or_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let result = vm.reg32(REG_EAX) | imm;
    vm.set_reg32(REG_EAX, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn or_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let result = vm.reg8(REG_AL) | imm;
    vm.set_reg8(REG_AL, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn and_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg32(modrm.reg);
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = lhs & rhs;
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn and_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg8(modrm.reg);
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = lhs & rhs;
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn and_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg32(modrm.reg);
    let result = lhs & rhs;
    vm.set_reg32(modrm.reg, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn and_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg8(modrm.reg);
    let result = lhs & rhs;
    vm.set_reg8(modrm.reg, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn and_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let result = vm.reg32(REG_EAX) & imm;
    vm.set_reg32(REG_EAX, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn and_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let result = vm.reg8(REG_AL) & imm;
    vm.set_reg8(REG_AL, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn xor_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = vm.reg8(modrm.reg);
    let dst = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = dst ^ src;
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xor_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = vm.reg32(modrm.reg);
    let dst = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = dst ^ src;
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xor_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let dst = vm.reg8(modrm.reg);
    let result = dst ^ src;
    vm.set_reg8(modrm.reg, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xor_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let dst = vm.reg32(modrm.reg);
    let result = dst ^ src;
    vm.set_reg32(modrm.reg, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xor_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let result = vm.reg32(REG_EAX) ^ imm;
    vm.set_reg32(REG_EAX, result);
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn xor_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let result = vm.reg8(REG_AL) ^ imm;
    vm.set_reg8(REG_AL, result);
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn test_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let rhs = vm.reg8(modrm.reg);
    let result = lhs & rhs;
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn test_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let rhs = vm.reg32(modrm.reg);
    let result = lhs & rhs;
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn test_al_imm8(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let result = vm.reg8(REG_AL) & imm;
    update_flags_logic8(vm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn test_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let result = vm.reg32(REG_EAX) & imm;
    update_flags_logic32(vm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn not_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    write_rm8(vm, modrm, prefixes.segment_base, !value)?;
    Ok(())
}

pub(crate) fn not_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    write_rm32(vm, modrm, prefixes.segment_base, !value)?;
    Ok(())
}

pub(crate) fn or_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst | imm;
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    Ok(())
}

pub(crate) fn or_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst | imm;
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    Ok(())
}

pub(crate) fn and_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst & imm;
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    Ok(())
}

pub(crate) fn and_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst & imm;
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    Ok(())
}

pub(crate) fn xor_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst ^ imm;
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic32(vm, result);
    Ok(())
}

pub(crate) fn xor_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst ^ imm;
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_logic8(vm, result);
    Ok(())
}
