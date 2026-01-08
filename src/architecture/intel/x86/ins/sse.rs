//! x86 SSE instruction helpers (minimal subset).

use crate::vm::{Vm, VmError};

use super::core::{calc_ea, decode_modrm, Prefixes};

pub(crate) fn exec_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor + 1)?;
    let modrm = decode_modrm(vm, cursor + 2)?;
    match opcode {
        0x57 => xorps(vm, &modrm, prefixes)?,
        0x60 => punpcklbw(vm, &modrm, prefixes)?, // PUNPCKLBW
        0x61 => punpcklwd(vm, &modrm, prefixes)?, // PUNPCKLWD
        0x6E => movd_xmm_from_r32(vm, &modrm, prefixes)?, // MOVD xmm, r/m32
        0x6F => mov_xmm_from_rm(vm, &modrm, prefixes)?,
        0x70 => pshufd(
            vm,
            &modrm,
            prefixes,
            vm.read_u8(cursor + 2 + modrm.len as u32)?,
        )?, // PSHUFD
        0x7E => movd_r32_from_xmm(vm, &modrm, prefixes)?, // MOVD r32, xmm
        0x7F => mov_rm_from_xmm(vm, &modrm, prefixes)?,
        0xD6 => movq_rm_from_xmm(vm, &modrm, prefixes)?,
        0xEF => pxor(vm, &modrm, prefixes)?, // PXOR
        _ => return Err(VmError::UnsupportedInstruction(opcode)),
    }
    // PSHUFD has an extra immediate byte
    let extra = if opcode == 0x70 { 1 } else { 0 };
    vm.set_eip(cursor + 2 + modrm.len as u32 + extra);
    Ok(())
}

fn xorps(vm: &mut Vm, modrm: &super::core::ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let dst = modrm.reg;
    let mut value = vm.xmm(dst);
    let src = if modrm.mod_bits == 3 {
        vm.xmm(modrm.rm)
    } else {
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        read_m128(vm, addr)?
    };
    for (idx, slot) in value.iter_mut().enumerate() {
        *slot ^= src[idx];
    }
    vm.set_xmm(dst, value);
    Ok(())
}

/// PXOR - Packed XOR (0F EF)
fn pxor(vm: &mut Vm, modrm: &super::core::ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    // PXOR is essentially the same as XORPS for our purposes
    xorps(vm, modrm, prefixes)
}

/// MOVD r/m32, xmm (0F 7E) - Move doubleword from XMM to GPR/memory
fn movd_r32_from_xmm(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let xmm_value = vm.xmm(modrm.reg);
    // Get low 32 bits
    let value = u32::from_le_bytes([xmm_value[0], xmm_value[1], xmm_value[2], xmm_value[3]]);

    if modrm.mod_bits == 3 {
        // Move to GPR
        vm.set_reg32(modrm.rm, value);
    } else {
        // Move to memory
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        vm.write_u32(addr, value)?;
    }
    Ok(())
}

/// MOVD xmm, r/m32 (0F 6E) - Move doubleword from GPR/memory to XMM
fn movd_xmm_from_r32(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let value = if modrm.mod_bits == 3 {
        // Move from GPR
        vm.reg32(modrm.rm)
    } else {
        // Move from memory
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        vm.read_u32(addr)?
    };

    // Zero-extend to 128 bits and store in XMM register
    let mut xmm_value = [0u8; 16];
    xmm_value[0..4].copy_from_slice(&value.to_le_bytes());
    vm.set_xmm(modrm.reg, xmm_value);
    Ok(())
}

/// PUNPCKLBW xmm, xmm/m128 (66 0F 60) - Unpack low bytes
fn punpcklbw(vm: &mut Vm, modrm: &super::core::ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let dst = vm.xmm(modrm.reg);
    let src = if modrm.mod_bits == 3 {
        vm.xmm(modrm.rm)
    } else {
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        read_m128(vm, addr)?
    };

    // Interleave low 8 bytes
    let mut result = [0u8; 16];
    for i in 0..8 {
        result[i * 2] = dst[i];
        result[i * 2 + 1] = src[i];
    }
    vm.set_xmm(modrm.reg, result);
    Ok(())
}

/// PUNPCKLWD xmm, xmm/m128 (66 0F 61) - Unpack low words
fn punpcklwd(vm: &mut Vm, modrm: &super::core::ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let dst = vm.xmm(modrm.reg);
    let src = if modrm.mod_bits == 3 {
        vm.xmm(modrm.rm)
    } else {
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        read_m128(vm, addr)?
    };

    // Interleave low 4 words (8 bytes)
    let mut result = [0u8; 16];
    for i in 0..4 {
        result[i * 4] = dst[i * 2];
        result[i * 4 + 1] = dst[i * 2 + 1];
        result[i * 4 + 2] = src[i * 2];
        result[i * 4 + 3] = src[i * 2 + 1];
    }
    vm.set_xmm(modrm.reg, result);
    Ok(())
}

/// PSHUFD xmm, xmm/m128, imm8 (66 0F 70) - Shuffle packed doublewords
fn pshufd(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
    imm8: u8,
) -> Result<(), VmError> {
    let src = if modrm.mod_bits == 3 {
        vm.xmm(modrm.rm)
    } else {
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        read_m128(vm, addr)?
    };

    // Get the 4 dwords from source
    let dwords: [u32; 4] = [
        u32::from_le_bytes([src[0], src[1], src[2], src[3]]),
        u32::from_le_bytes([src[4], src[5], src[6], src[7]]),
        u32::from_le_bytes([src[8], src[9], src[10], src[11]]),
        u32::from_le_bytes([src[12], src[13], src[14], src[15]]),
    ];

    // Shuffle according to imm8
    let result_dwords: [u32; 4] = [
        dwords[(imm8 & 0x03) as usize],
        dwords[((imm8 >> 2) & 0x03) as usize],
        dwords[((imm8 >> 4) & 0x03) as usize],
        dwords[((imm8 >> 6) & 0x03) as usize],
    ];

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&result_dwords[0].to_le_bytes());
    result[4..8].copy_from_slice(&result_dwords[1].to_le_bytes());
    result[8..12].copy_from_slice(&result_dwords[2].to_le_bytes());
    result[12..16].copy_from_slice(&result_dwords[3].to_le_bytes());

    vm.set_xmm(modrm.reg, result);
    Ok(())
}

fn mov_xmm_from_rm(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let value = if modrm.mod_bits == 3 {
        vm.xmm(modrm.rm)
    } else {
        let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
        read_m128(vm, addr)?
    };
    vm.set_xmm(modrm.reg, value);
    Ok(())
}

fn mov_rm_from_xmm(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let value = vm.xmm(modrm.reg);
    if modrm.mod_bits == 3 {
        vm.set_xmm(modrm.rm, value);
        return Ok(());
    }
    let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
    write_m128(vm, addr, &value)
}

fn movq_rm_from_xmm(
    vm: &mut Vm,
    modrm: &super::core::ModRm,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let value = vm.xmm(modrm.reg);
    if modrm.mod_bits == 3 {
        let mut dst = vm.xmm(modrm.rm);
        dst[..8].copy_from_slice(&value[..8]);
        dst[8..].fill(0);
        vm.set_xmm(modrm.rm, dst);
        return Ok(());
    }
    let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
    vm.write_bytes(addr, &value[..8])
}

fn read_m128(vm: &Vm, addr: u32) -> Result<[u8; 16], VmError> {
    let mut out = [0u8; 16];
    for (idx, slot) in out.iter_mut().enumerate() {
        *slot = vm.read_u8(addr.wrapping_add(idx as u32))?;
    }
    Ok(out)
}

fn write_m128(vm: &mut Vm, addr: u32, value: &[u8; 16]) -> Result<(), VmError> {
    vm.write_bytes(addr, value)
}
