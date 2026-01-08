//! x86 mov and lea instruction handlers.

use crate::vm::{Vm, VmError, REG_AL, REG_EAX, REG_ECX, REG_EDI, REG_ESI};

use super::core::{
    calc_ea, decode_modrm, read_rm16, read_rm32, read_rm8, segment_value, update_flags_sub16,
    update_flags_sub32, update_flags_sub8, write_rm16, write_rm32, write_rm8, Prefixes,
};

pub(crate) fn mov_rm8_r8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let value = vm.reg8(modrm.reg);
    write_rm8(vm, &modrm, prefixes.segment_base, value)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn mov_rm32_r32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if prefixes.operand_size_16 {
        let value = vm.reg16(modrm.reg);
        write_rm16(vm, &modrm, prefixes.segment_base, value)?;
    } else {
        let value = vm.reg32(modrm.reg);
        write_rm32(vm, &modrm, prefixes.segment_base, value)?;
    }
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn mov_r8_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let value = read_rm8(vm, &modrm, prefixes.segment_base)?;
    vm.set_reg8(modrm.reg, value);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn mov_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if prefixes.operand_size_16 {
        let value = read_rm16(vm, &modrm, prefixes.segment_base)?;
        vm.set_reg16(modrm.reg, value);
    } else {
        let value = read_rm32(vm, &modrm, prefixes.segment_base)?;
        vm.set_reg32(modrm.reg, value);
    }
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn mov_seg_to_rm16(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    if !prefixes.operand_size_16 {
        return Err(VmError::UnsupportedInstruction(0x8C));
    }
    let modrm = decode_modrm(vm, cursor + 1)?;
    let value = segment_value(modrm.reg);
    write_rm16(vm, &modrm, prefixes.segment_base, value)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn lea(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let addr = calc_ea(vm, &modrm, prefixes.segment_base)?;
    vm.set_reg32(modrm.reg, addr);
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn mov_r8_imm8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0xB0;
    let imm = vm.read_u8(cursor + 1)?;
    vm.set_reg8(reg, imm);
    vm.set_eip(cursor + 2);
    Ok(())
}

pub(crate) fn mov_r32_imm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0xB8;
    if prefixes.operand_size_16 {
        let imm = vm.read_u16(cursor + 1)?;
        vm.set_reg16(reg, imm);
        vm.set_eip(cursor + 3);
    } else {
        let imm = vm.read_u32(cursor + 1)?;
        vm.set_reg32(reg, imm);
        vm.set_eip(cursor + 5);
    }
    Ok(())
}

pub(crate) fn mov_rm8_imm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if modrm.reg != 0 {
        return Err(VmError::UnsupportedInstruction(0xC6));
    }
    let imm = vm.read_u8(cursor + 1 + modrm.len as u32)?;
    write_rm8(vm, &modrm, prefixes.segment_base, imm)?;
    vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
    Ok(())
}

pub(crate) fn mov_rm32_imm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    if modrm.reg != 0 {
        return Err(VmError::UnsupportedInstruction(0xC7));
    }
    if prefixes.operand_size_16 {
        let imm = vm.read_u16(cursor + 1 + modrm.len as u32)?;
        write_rm16(vm, &modrm, prefixes.segment_base, imm)?;
        vm.set_eip(cursor + 1 + modrm.len as u32 + 2);
    } else {
        let imm = vm.read_u32(cursor + 1 + modrm.len as u32)?;
        write_rm32(vm, &modrm, prefixes.segment_base, imm)?;
        vm.set_eip(cursor + 1 + modrm.len as u32 + 4);
    }
    Ok(())
}

pub(crate) fn mov_moffs_to_eax(
    vm: &mut Vm,
    cursor: u32,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let addr = vm.read_u32(cursor + 1)?.wrapping_add(prefixes.segment_base);
    if prefixes.operand_size_16 {
        let value = vm.read_u16(addr)?;
        vm.set_reg16(0, value);
    } else {
        let value = vm.read_u32(addr)?;
        vm.set_reg32(REG_EAX, value);
    }
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn mov_moffs_to_al(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let addr = vm.read_u32(cursor + 1)?.wrapping_add(prefixes.segment_base);
    let value = vm.read_u8(addr)?;
    vm.set_reg8(REG_AL, value);
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn mov_al_to_moffs(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let addr = vm.read_u32(cursor + 1)?.wrapping_add(prefixes.segment_base);
    let value = vm.reg8(REG_AL);
    vm.write_u8(addr, value)?;
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn mov_eax_to_moffs(
    vm: &mut Vm,
    cursor: u32,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let addr = vm.read_u32(cursor + 1)?.wrapping_add(prefixes.segment_base);
    if prefixes.operand_size_16 {
        let value = vm.reg16(0);
        vm.write_u16(addr, value)?;
    } else {
        let value = vm.reg32(REG_EAX);
        vm.write_u32(addr, value)?;
    }
    vm.set_eip(cursor + 5);
    Ok(())
}

pub(crate) fn movsd(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let size = if prefixes.operand_size_16 { 2 } else { 4 };
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut src = vm.reg32(REG_ESI);
    let mut dst = vm.reg32(REG_EDI);
    for _ in 0..count {
        if size == 2 {
            let value = vm.read_u16(src)?;
            vm.write_u16(dst, value)?;
            src = src.wrapping_add(2);
            dst = dst.wrapping_add(2);
        } else {
            let value = vm.read_u32(src)?;
            vm.write_u32(dst, value)?;
            src = src.wrapping_add(4);
            dst = dst.wrapping_add(4);
        }
    }
    vm.set_reg32(REG_ESI, src);
    vm.set_reg32(REG_EDI, dst);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn movsb(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut src = vm.reg32(REG_ESI);
    let mut dst = vm.reg32(REG_EDI);
    for _ in 0..count {
        let value = vm.read_u8(src)?;
        vm.write_u8(dst, value)?;
        src = src.wrapping_add(1);
        dst = dst.wrapping_add(1);
    }
    vm.set_reg32(REG_ESI, src);
    vm.set_reg32(REG_EDI, dst);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn stosb(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut dst = vm.reg32(REG_EDI);
    let value = vm.reg8(REG_AL);
    for _ in 0..count {
        vm.write_u8(dst, value)?;
        dst = dst.wrapping_add(1);
    }
    vm.set_reg32(REG_EDI, dst);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn stosd(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let size = if prefixes.operand_size_16 { 2 } else { 4 };
    let count = if prefixes.rep { vm.reg32(REG_ECX) } else { 1 };
    let mut dst = vm.reg32(REG_EDI);
    let value32 = vm.reg32(REG_EAX);
    for _ in 0..count {
        if size == 2 {
            vm.write_u16(dst, value32 as u16)?;
            dst = dst.wrapping_add(2);
        } else {
            vm.write_u32(dst, value32)?;
            dst = dst.wrapping_add(4);
        }
    }
    vm.set_reg32(REG_EDI, dst);
    if prefixes.rep {
        vm.set_reg32(REG_ECX, 0);
    }
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn scasb(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    scas_common(vm, cursor, prefixes, 1)
}

pub(crate) fn scasd(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let size = if prefixes.operand_size_16 { 2 } else { 4 };
    scas_common(vm, cursor, prefixes, size)
}

fn scas_common(vm: &mut Vm, cursor: u32, prefixes: Prefixes, size: u32) -> Result<(), VmError> {
    let repeats = prefixes.rep || prefixes.repne;
    let mut remaining = if repeats { vm.reg32(REG_ECX) } else { 1 };
    let mut edi = vm.reg32(REG_EDI);
    while remaining > 0 {
        match size {
            1 => {
                let dst = vm.read_u8(edi)?;
                let src = vm.reg8(REG_AL);
                let result = dst.wrapping_sub(src);
                update_flags_sub8(vm, dst, src, result);
            }
            2 => {
                let dst = vm.read_u16(edi)?;
                let src = vm.reg16(REG_EAX);
                let result = dst.wrapping_sub(src);
                update_flags_sub16(vm, dst, src, result);
            }
            4 => {
                let dst = vm.read_u32(edi)?;
                let src = vm.reg32(REG_EAX);
                let result = dst.wrapping_sub(src);
                update_flags_sub32(vm, dst, src, result);
            }
            _ => unreachable!(),
        }
        edi = edi.wrapping_add(size);
        remaining = remaining.wrapping_sub(1);
        if !repeats {
            break;
        }
        let condition = if prefixes.rep { vm.zf() } else { !vm.zf() };
        if !condition || remaining == 0 {
            break;
        }
    }
    if repeats {
        vm.set_reg32(REG_ECX, remaining);
    }
    vm.set_reg32(REG_EDI, edi);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn movzx_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm8(vm, &modrm, prefixes.segment_base)? as u32;
    vm.set_reg32(modrm.reg, value);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

pub(crate) fn movzx_rm16(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm16(vm, &modrm, prefixes.segment_base)? as u32;
    vm.set_reg32(modrm.reg, value);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

pub(crate) fn movsx_rm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm8(vm, &modrm, prefixes.segment_base)? as i8 as i32 as u32;
    vm.set_reg32(modrm.reg, value);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

pub(crate) fn movsx_rm16(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm16(vm, &modrm, prefixes.segment_base)? as i16 as i32 as u32;
    vm.set_reg32(modrm.reg, value);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}
