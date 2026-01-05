//! x86 SSE instruction helpers (minimal subset).

use crate::vm::{Vm, VmError};

use super::core::{calc_ea, decode_modrm, Prefixes};

pub(crate) fn exec_rm32(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor + 1)?;
    let modrm = decode_modrm(vm, cursor + 2)?;
    match opcode {
        0x57 => xorps(vm, &modrm, prefixes)?,
        0x6F => mov_xmm_from_rm(vm, &modrm, prefixes)?,
        0x7F => mov_rm_from_xmm(vm, &modrm, prefixes)?,
        0xD6 => movq_rm_from_xmm(vm, &modrm, prefixes)?,
        _ => return Err(VmError::UnsupportedInstruction(opcode)),
    }
    vm.set_eip(cursor + 2 + modrm.len as u32);
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
