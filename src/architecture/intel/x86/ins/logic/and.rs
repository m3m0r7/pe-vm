use crate::vm::{Vm, VmError, REG_AL, REG_EAX};

use crate::architecture::intel::x86::ins::core::{
    decode_modrm,
    read_rm16,
    read_rm32,
    read_rm8,
    update_flags_logic16,
    update_flags_logic32,
    update_flags_logic8,
    write_rm16,
    write_rm32,
    write_rm8,
    ModRm,
    Prefixes,
};

pub(crate) fn and_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    // Honor operand-size override for 16-bit AND.
    if prefixes.operand_size_16 {
        let rhs = vm.reg16(modrm.reg);
        let lhs = read_rm16(vm, &modrm, prefixes.segment_base)?;
        let result = lhs & rhs;
        write_rm16(vm, &modrm, prefixes.segment_base, result)?;
        update_flags_logic16(vm, result);
    } else {
        let rhs = vm.reg32(modrm.reg);
        let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
        let result = lhs & rhs;
        write_rm32(vm, &modrm, prefixes.segment_base, result)?;
        update_flags_logic32(vm, result);
    }
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
    // Honor operand-size override for 16-bit AND.
    if prefixes.operand_size_16 {
        let rhs = read_rm16(vm, &modrm, prefixes.segment_base)?;
        let lhs = vm.reg16(modrm.reg);
        let result = lhs & rhs;
        vm.set_reg16(modrm.reg, result);
        update_flags_logic16(vm, result);
    } else {
        let rhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
        let lhs = vm.reg32(modrm.reg);
        let result = lhs & rhs;
        vm.set_reg32(modrm.reg, result);
        update_flags_logic32(vm, result);
    }
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
    prefixes: Prefixes,
) -> Result<(), VmError> {
    // Honor operand-size override for 16-bit AND.
    if prefixes.operand_size_16 {
        let imm = vm.read_u16(cursor + 1)?;
        let result = vm.reg16(REG_EAX) & imm;
        vm.set_reg16(REG_EAX, result);
        update_flags_logic16(vm, result);
        vm.set_eip(cursor + 3);
    } else {
        let imm = vm.read_u32(cursor + 1)?;
        let result = vm.reg32(REG_EAX) & imm;
        vm.set_reg32(REG_EAX, result);
        update_flags_logic32(vm, result);
        vm.set_eip(cursor + 5);
    }
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

pub(crate) fn and_rm32_imm(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    imm: u32,
) -> Result<(), VmError> {
    // Honor operand-size override for 16-bit AND.
    if prefixes.operand_size_16 {
        let dst = read_rm16(vm, modrm, prefixes.segment_base)?;
        let result = dst & imm as u16;
        write_rm16(vm, modrm, prefixes.segment_base, result)?;
        update_flags_logic16(vm, result);
    } else {
        let dst = read_rm32(vm, modrm, prefixes.segment_base)?;
        let result = dst & imm;
        write_rm32(vm, modrm, prefixes.segment_base, result)?;
        update_flags_logic32(vm, result);
    }
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
