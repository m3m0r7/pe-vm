//! x86 imul instruction handlers.

use crate::vm::{Vm, VmError};

use super::core::{decode_modrm, read_rm32, Prefixes};

pub(crate) fn imul_rm32_imm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let imm = vm.read_u8(cursor + 1 + modrm.len as u32)? as i8 as i32;
    let value = read_rm32(vm, &modrm, prefixes.segment_base)? as i32 as i64;
    let result = value.wrapping_mul(imm as i64);
    let low = result as u32;
    let high = (result >> 32) as u32;
    vm.set_reg32(modrm.reg, low);
    let sign_ext = if (low & 0x8000_0000) != 0 {
        0xFFFF_FFFF
    } else {
        0
    };
    let overflow = high != sign_ext;
    vm.set_flags(vm.zf(), vm.sf(), overflow, overflow);
    vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
    Ok(())
}

pub(crate) fn imul_rm32_imm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let imm = vm.read_u32(cursor + 1 + modrm.len as u32)? as i32 as i64;
    let value = read_rm32(vm, &modrm, prefixes.segment_base)? as i32 as i64;
    let result = value.wrapping_mul(imm);
    let low = result as u32;
    let high = (result >> 32) as u32;
    vm.set_reg32(modrm.reg, low);
    let sign_ext = if (low & 0x8000_0000) != 0 {
        0xFFFF_FFFF
    } else {
        0
    };
    let overflow = high != sign_ext;
    vm.set_flags(vm.zf(), vm.sf(), overflow, overflow);
    vm.set_eip(cursor + 1 + modrm.len as u32 + 4);
    Ok(())
}

pub(crate) fn imul_r32_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let value = read_rm32(vm, &modrm, prefixes.segment_base)? as i32 as i64;
    let lhs = vm.reg32(modrm.reg) as i32 as i64;
    let result = lhs.wrapping_mul(value);
    let low = result as u32;
    let high = (result >> 32) as u32;
    vm.set_reg32(modrm.reg, low);
    let sign_ext = if (low & 0x8000_0000) != 0 {
        0xFFFF_FFFF
    } else {
        0
    };
    let overflow = high != sign_ext;
    vm.set_flags(vm.zf(), vm.sf(), overflow, overflow);
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}
