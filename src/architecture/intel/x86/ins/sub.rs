//! x86 sub/sbb/cmp/neg instruction handlers.

use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use super::core::{
    decode_modrm, read_rm16, read_rm32, read_rm8, sbb32, sbb8, update_flags_sub16,
    update_flags_sub32, update_flags_sub32_with_cf, update_flags_sub8,
    update_flags_sub8_with_cf, write_rm32, write_rm8, ModRm, Prefixes,
};

pub(crate) fn sub_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg8(modrm.reg);
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let result = lhs.wrapping_sub(rhs);
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_sub8(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sub_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = vm.reg32(modrm.reg);
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let result = lhs.wrapping_sub(rhs);
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_sub32(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sub_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg8(modrm.reg);
    let result = lhs.wrapping_sub(rhs);
    vm.set_reg8(modrm.reg, result);
    update_flags_sub8(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sub_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let lhs = vm.reg32(modrm.reg);
    let result = lhs.wrapping_sub(rhs);
    vm.set_reg32(modrm.reg, result);
    update_flags_sub32(vm, lhs, rhs, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sub_al_imm8(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let lhs = vm.reg8(REG_AL);
    let result = lhs.wrapping_sub(imm);
    vm.set_reg8(REG_AL, result);
    update_flags_sub8(vm, lhs, imm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn sub_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let lhs = vm.reg32(REG_EAX);
    let result = lhs.wrapping_sub(imm);
    vm.set_reg32(REG_EAX, result);
    update_flags_sub32(vm, lhs, imm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn dec_reg(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x48;
    let value = vm.reg32(reg);
    let result = value.wrapping_sub(1);
    vm.set_reg32(reg, result);
    update_flags_sub32(vm, value, 1, result);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn dec_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_sub(1);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub8(vm, value, 1, result);
    Ok(())
}

pub(crate) fn dec_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_sub(1);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub32(vm, value, 1, result);
    Ok(())
}

pub(crate) fn cmp_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let dst = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let src = vm.reg8(modrm.reg);
    let result = dst.wrapping_sub(src);
    update_flags_sub8(vm, dst, src, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn cmp_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if prefixes.operand_size_16 {
        let dst = read_rm16(vm, &modrm, prefixes.segment_base)?;
        let src = vm.reg16(modrm.reg);
        let result = dst.wrapping_sub(src);
        update_flags_sub16(vm, dst, src, result);
    } else {
        let dst = read_rm32(vm, &modrm, prefixes.segment_base)?;
        let src = vm.reg32(modrm.reg);
        let result = dst.wrapping_sub(src);
        update_flags_sub32(vm, dst, src, result);
    }
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn cmp_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let dst = vm.reg8(modrm.reg);
    let result = dst.wrapping_sub(src);
    update_flags_sub8(vm, dst, src, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn cmp_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let src = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let dst = vm.reg32(modrm.reg);
    let result = dst.wrapping_sub(src);
    update_flags_sub32(vm, dst, src, result);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn cmp_al_imm8(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let dst = vm.reg8(REG_AL);
    let result = dst.wrapping_sub(imm);
    update_flags_sub8(vm, dst, imm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn cmp_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let dst = vm.reg32(REG_EAX);
    let result = dst.wrapping_sub(imm);
    update_flags_sub32(vm, dst, imm, result);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn sbb_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let rhs = vm.reg8(modrm.reg);
    let (result, cf) = sbb8(vm, lhs, rhs);
    write_rm8(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_sub8_with_cf(vm, lhs, rhs, result, cf);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sbb_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let rhs = vm.reg32(modrm.reg);
    let (result, cf) = sbb32(vm, lhs, rhs);
    write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    update_flags_sub32_with_cf(vm, lhs, rhs, result, cf);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sbb_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = vm.reg8(modrm.reg);
    let rhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
    let (result, cf) = sbb8(vm, lhs, rhs);
    vm.set_reg8(modrm.reg, result);
    update_flags_sub8_with_cf(vm, lhs, rhs, result, cf);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sbb_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let lhs = vm.reg32(modrm.reg);
    let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let (result, cf) = sbb32(vm, lhs, rhs);
    vm.set_reg32(modrm.reg, result);
    update_flags_sub32_with_cf(vm, lhs, rhs, result, cf);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn sbb_al_imm8(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let lhs = vm.reg8(REG_AL);
    let (result, cf) = sbb8(vm, lhs, imm);
    vm.set_reg8(REG_AL, result);
    update_flags_sub8_with_cf(vm, lhs, imm, result, cf);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn sbb_eax_imm32(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let lhs = vm.reg32(REG_EAX);
    let (result, cf) = sbb32(vm, lhs, imm);
    vm.set_reg32(REG_EAX, result);
    update_flags_sub32_with_cf(vm, lhs, imm, result, cf);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn neg_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = (0u8).wrapping_sub(value);
    update_flags_sub8(vm, 0, value, result);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}

pub(crate) fn neg_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = (0u32).wrapping_sub(value);
    update_flags_sub32(vm, 0, value, result);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}

pub(crate) fn sub_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_sub(imm);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub32(vm, dst, imm, result);
    Ok(())
}

pub(crate) fn sbb_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let (result, cf) = sbb32(vm, dst, imm);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub32_with_cf(vm, dst, imm, result, cf);
    Ok(())
}

pub(crate) fn sub_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_sub(imm);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub8(vm, dst, imm, result);
    Ok(())
}

pub(crate) fn sbb_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let (result, cf) = sbb8(vm, dst, imm);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub8_with_cf(vm, dst, imm, result, cf);
    Ok(())
}

pub(crate) fn cmp_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_sub(imm);
    update_flags_sub32(vm, dst, imm, result);
    Ok(())
}

pub(crate) fn cmp_rm8_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u8,
) -> Result<(), VmError> {
    let dst = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = dst.wrapping_sub(imm);
    update_flags_sub8(vm, dst, imm, result);
    Ok(())
}
