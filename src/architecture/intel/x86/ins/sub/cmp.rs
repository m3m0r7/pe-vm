use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use crate::architecture::intel::x86::ins::core::{
    decode_modrm, read_rm16, read_rm32, read_rm8, update_flags_sub16, update_flags_sub32,
    update_flags_sub8, ModRm, Prefixes,
};

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

pub(crate) fn cmp_al_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u8(cursor + 1)?;
    let dst = vm.reg8(REG_AL);
    let result = dst.wrapping_sub(imm);
    update_flags_sub8(vm, dst, imm, result);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn cmp_eax_imm32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let imm = vm.read_u32(cursor + 1)?;
    let dst = vm.reg32(REG_EAX);
    let result = dst.wrapping_sub(imm);
    update_flags_sub32(vm, dst, imm, result);
    vm.set_eip(cursor + 5);
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
