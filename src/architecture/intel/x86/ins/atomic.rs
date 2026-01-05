//! x86 atomic instruction handlers.

use crate::vm::{Vm, VmError, REG_EAX};

use super::core::{
    decode_modrm, read_rm32, update_flags_add32, update_flags_sub32, write_rm32, Prefixes,
};

pub(crate) fn xchg_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let reg_val = vm.reg32(modrm.reg);
    if modrm.mod_bits == 3 {
        let rm_val = vm.reg32(modrm.rm);
        vm.set_reg32(modrm.rm, reg_val);
        vm.set_reg32(modrm.reg, rm_val);
    } else {
        let mem_val = read_rm32(vm, &modrm, prefixes.segment_base)?;
        write_rm32(vm, &modrm, prefixes.segment_base, reg_val)?;
        vm.set_reg32(modrm.reg, mem_val);
    }
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xchg_eax_reg(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x90;
    if reg == 0 {
        vm.set_eip(cursor + 1);
        return Ok(());
    }
    let eax = vm.reg32(REG_EAX);
    let other = vm.reg32(reg);
    vm.set_reg32(REG_EAX, other);
    vm.set_reg32(reg, eax);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn cmpxchg_rm32_r32(
    vm: &mut Vm,
    cursor: u32,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let eax = vm.reg32(REG_EAX);
    let result = value.wrapping_sub(eax);
    update_flags_sub32(vm, value, eax, result);
    if value == eax {
        let src = vm.reg32(modrm.reg);
        write_rm32(vm, &modrm, prefixes.segment_base, src)?;
    } else {
        vm.set_reg32(REG_EAX, value);
    }
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

pub(crate) fn xadd_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let src = vm.reg32(modrm.reg);
    let dst = if modrm.mod_bits == 3 {
        vm.reg32(modrm.rm)
    } else {
        read_rm32(vm, &modrm, prefixes.segment_base)?
    };
    let result = dst.wrapping_add(src);
    update_flags_add32(vm, dst, src, result);
    if modrm.mod_bits == 3 {
        vm.set_reg32(modrm.rm, result);
    } else {
        write_rm32(vm, &modrm, prefixes.segment_base, result)?;
    }
    vm.set_reg32(modrm.reg, dst);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}
