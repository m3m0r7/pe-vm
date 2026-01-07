//! FPU instruction handlers

mod d8;
mod d9;
mod db;
mod dc;
mod dd;
mod de;

use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{calc_ea, decode_modrm, ModRm, Prefixes};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FpuOpcode {
    D8 = 0xD8,
    D9 = 0xD9,
    DA = 0xDA,
    DB = 0xDB,
    DC = 0xDC,
    DD = 0xDD,
    DE = 0xDE,
    DF = 0xDF,
}

impl TryFrom<u8> for FpuOpcode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xD8 => Ok(Self::D8),
            0xD9 => Ok(Self::D9),
            0xDA => Ok(Self::DA),
            0xDB => Ok(Self::DB),
            0xDC => Ok(Self::DC),
            0xDD => Ok(Self::DD),
            0xDE => Ok(Self::DE),
            0xDF => Ok(Self::DF),
            _ => Err(value),
        }
    }
}

pub(super) fn read_f64(vm: &Vm, addr: u32) -> Result<f64, VmError> {
    let bits = vm.read_u64(addr)?;
    Ok(f64::from_bits(bits))
}

pub(super) fn write_f64(vm: &mut Vm, addr: u32, value: f64) -> Result<(), VmError> {
    vm.write_u64(addr, value.to_bits())
}

pub(super) fn read_i32(vm: &Vm, addr: u32) -> Result<i32, VmError> {
    Ok(vm.read_u32(addr)? as i32)
}

pub(super) fn mem_address(vm: &Vm, modrm: &ModRm, segment: u32) -> Result<u32, VmError> {
    if modrm.mod_bits == 3 {
        return Err(VmError::UnsupportedInstruction(0));
    }
    calc_ea(vm, modrm, segment)
}

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let opcode_byte = vm.read_u8(cursor)?;
    let opcode = FpuOpcode::try_from(opcode_byte)
        .map_err(|b| VmError::UnsupportedInstruction(b))?;
    let modrm = decode_modrm(vm, cursor + 1)?;
    let next = cursor + 1 + modrm.len as u32;
    match opcode {
        FpuOpcode::D8 => d8::handle(vm, &modrm, prefixes.segment_base)?,
        FpuOpcode::D9 => d9::handle(vm, &modrm, prefixes.segment_base)?,
        FpuOpcode::DA => return Err(VmError::UnsupportedInstruction(opcode as u8)),
        FpuOpcode::DB => db::handle(vm, &modrm, prefixes.segment_base)?,
        FpuOpcode::DC => dc::handle(vm, &modrm, prefixes.segment_base)?,
        FpuOpcode::DD => dd::handle(vm, &modrm, prefixes.segment_base)?,
        FpuOpcode::DE => de::handle(vm, &modrm)?,
        FpuOpcode::DF => return Err(VmError::UnsupportedInstruction(opcode as u8)),
    }
    vm.set_eip(next);
    Ok(())
}
