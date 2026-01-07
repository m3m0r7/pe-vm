//! FPU instruction handlers

mod fadd;
mod fild;
mod fld;
mod fldcw;
mod fmul;
mod fstcw;
mod fstp;

use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{calc_ea, decode_modrm, ModRm, Prefixes};

/// FPU instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FpuInstruction {
    /// FADD m64real (DC /0)
    Fadd,
    /// FADDP ST(i), ST(0) (DE C0+i)
    Faddp,
    /// FMUL m64real (DC /1)
    Fmul,
    /// FMULP ST(i), ST(0) (DE C8+i)
    Fmulp,
    /// FLD m64real (DD /0)
    Fld,
    /// FSTP m64real (DD /3)
    Fstp,
    /// FILD m32int (DB /0)
    Fild,
    /// FLDCW m2byte (D9 /5)
    Fldcw,
    /// FSTCW m2byte (D9 /7)
    Fstcw,
}

impl FpuInstruction {
    /// Decode FPU instruction from opcode and modrm
    fn decode(opcode: u8, modrm: &ModRm) -> Option<Self> {
        match opcode {
            0xD9 => match modrm.reg {
                5 => Some(Self::Fldcw),
                7 => Some(Self::Fstcw),
                _ => None,
            },
            0xDB => match modrm.reg {
                0 => Some(Self::Fild),
                _ => None,
            },
            0xDC => match modrm.reg {
                0 => Some(Self::Fadd),
                1 => Some(Self::Fmul),
                _ => None,
            },
            0xDD => match modrm.reg {
                0 => Some(Self::Fld),
                3 => Some(Self::Fstp),
                _ => None,
            },
            0xDE if modrm.mod_bits == 3 => match modrm.reg {
                0 => Some(Self::Faddp),
                1 => Some(Self::Fmulp),
                _ => None,
            },
            _ => None,
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
    let opcode = vm.read_u8(cursor)?;
    let modrm = decode_modrm(vm, cursor + 1)?;
    let next = cursor + 1 + modrm.len as u32;

    let instruction = FpuInstruction::decode(opcode, &modrm)
        .ok_or(VmError::UnsupportedInstruction(opcode))?;

    match instruction {
        FpuInstruction::Fadd => fadd::fadd_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Faddp => fadd::faddp(vm, &modrm)?,
        FpuInstruction::Fmul => fmul::fmul_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fmulp => fmul::fmulp(vm, &modrm)?,
        FpuInstruction::Fld => fld::fld_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fstp => fstp::fstp_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fild => fild::fild_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fldcw => fldcw::fldcw(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fstcw => fstcw::fstcw(vm, &modrm, prefixes.segment_base)?,
    }

    vm.set_eip(next);
    Ok(())
}
