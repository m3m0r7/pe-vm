use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use crate::architecture::intel::x86::ins::core::{
    decode_modrm, read_rm32, read_rm8, update_flags_sub32, update_flags_sub8, write_rm32,
    write_rm8, ModRm, Prefixes,
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
