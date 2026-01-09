//! FPU instruction handlers

mod fadd;
mod fdiv;
mod fild;
mod fistp;
mod fld;
mod fldcw;
mod fmul;
mod fsub;
mod fstcw;
mod fstp;

use crate::vm::{Vm, VmError, REG_EAX};

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
    /// FDIV ST(0), ST(i) (D8 F0+i)
    Fdiv,
    /// FDIVR ST(0), ST(i) (D8 F8+i)
    Fdivr,
    /// FSUBP ST(i), ST(0) (DE E8+i)
    Fsubp,
    /// FLD m64real (DD /0)
    Fld,
    /// FLD m32real (D9 /0)
    FldM32,
    /// FLD ST(i) (D9 C0+i)
    FldSt,
    /// FST m32real (D9 /2)
    FstM32,
    /// FSTP m32real (D9 /3)
    FstpM32,
    /// FSTP m64real (DD /3)
    Fstp,
    /// FSTP ST(i) (DD D8+i)
    FstpSt,
    /// FILD m32int (DB /0)
    Fild,
    /// FILD m64int (DF /5)
    FildM64,
    /// FIST m32int (DB /2)
    Fist,
    /// FISTP m32int (DB /3)
    Fistp,
    /// FISTP m64int (DF /7)
    FistpM64,
    /// FLDCW m2byte (D9 /5)
    Fldcw,
    /// FSTCW m2byte (D9 /7)
    Fstcw,
    /// FNCLEX/FCLEX - Clear FPU exception flags (DB E2)
    Fnclex,
    /// FNINIT/FINIT - Initialize FPU (DB E3)
    Fninit,
    /// FRNDINT - Round to integer (D9 FC)
    Frndint,
    /// FSQRT - Square root (D9 FA)
    Fsqrt,
    /// FUCOMPP - Unordered compare ST(0) with ST(1), pop twice (DA E9)
    Fucompp,
    /// FNSTSW AX - Store status word in AX (DF E0)
    Fnstsw,
}

impl FpuInstruction {
    /// Decode FPU instruction from opcode and modrm
    fn decode(opcode: u8, modrm: &ModRm) -> Option<Self> {
        match opcode {
            0xD8 if modrm.mod_bits == 3 => match modrm.reg {
                6 => Some(Self::Fdiv),
                7 => Some(Self::Fdivr),
                _ => None,
            },
            0xD9 => {
                if modrm.mod_bits == 3 {
                    match (modrm.reg, modrm.rm) {
                        (0, _) => Some(Self::FldSt),
                        (7, 2) => Some(Self::Fsqrt),
                        (7, 4) => Some(Self::Frndint),
                        _ => None,
                    }
                } else {
                    match modrm.reg {
                        0 => Some(Self::FldM32),
                        2 => Some(Self::FstM32),
                        3 => Some(Self::FstpM32),
                        5 => Some(Self::Fldcw),
                        7 => Some(Self::Fstcw),
                        _ => None,
                    }
                }
            }
            0xDB => {
                // Register mode (mod_bits == 3) has different encoding
                if modrm.mod_bits == 3 {
                    // DB E0-E7 range: Check full modrm byte
                    // DB E2 = FNCLEX (reg=4, rm=2)
                    // DB E3 = FNINIT (reg=4, rm=3)
                    match (modrm.reg, modrm.rm) {
                        (4, 2) => Some(Self::Fnclex),
                        (4, 3) => Some(Self::Fninit),
                        _ => None,
                    }
                } else {
                    // Memory mode
                    match modrm.reg {
                        0 => Some(Self::Fild),
                        2 => Some(Self::Fist),
                        3 => Some(Self::Fistp),
                        _ => None,
                    }
                }
            }
            0xDA if modrm.mod_bits == 3 => match (modrm.reg, modrm.rm) {
                (5, 1) => Some(Self::Fucompp),
                _ => None,
            },
            0xDC => match modrm.reg {
                0 => Some(Self::Fadd),
                1 => Some(Self::Fmul),
                _ => None,
            },
            0xDD => {
                if modrm.mod_bits == 3 {
                    match modrm.reg {
                        3 => Some(Self::FstpSt),
                        _ => None,
                    }
                } else {
                    match modrm.reg {
                        0 => Some(Self::Fld),
                        3 => Some(Self::Fstp),
                        _ => None,
                    }
                }
            }
            0xDE if modrm.mod_bits == 3 => match modrm.reg {
                0 => Some(Self::Faddp),
                1 => Some(Self::Fmulp),
                5 => Some(Self::Fsubp),
                _ => None,
            },
            0xDF => {
                if modrm.mod_bits == 3 {
                    match (modrm.reg, modrm.rm) {
                        (4, 0) => Some(Self::Fnstsw),
                        _ => None,
                    }
                } else {
                    match modrm.reg {
                        5 => Some(Self::FildM64),
                        7 => Some(Self::FistpM64),
                        _ => None,
                    }
                }
            }
            _ => None,
        }
    }
}

pub(super) fn read_f32(vm: &Vm, addr: u32) -> Result<f64, VmError> {
    let bits = vm.read_u32(addr)?;
    Ok(f32::from_bits(bits) as f64)
}

pub(super) fn read_f64(vm: &Vm, addr: u32) -> Result<f64, VmError> {
    let bits = vm.read_u64(addr)?;
    Ok(f64::from_bits(bits))
}

pub(super) fn write_f32(vm: &mut Vm, addr: u32, value: f64) -> Result<(), VmError> {
    vm.write_u32(addr, (value as f32).to_bits())
}

pub(super) fn write_f64(vm: &mut Vm, addr: u32, value: f64) -> Result<(), VmError> {
    vm.write_u64(addr, value.to_bits())
}

pub(super) fn read_i32(vm: &Vm, addr: u32) -> Result<i32, VmError> {
    Ok(vm.read_u32(addr)? as i32)
}

pub(super) fn read_i64(vm: &Vm, addr: u32) -> Result<i64, VmError> {
    Ok(vm.read_u64(addr)? as i64)
}

pub(super) fn write_i64(vm: &mut Vm, addr: u32, value: i64) -> Result<(), VmError> {
    vm.write_u64(addr, value as u64)
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

    let instruction =
        FpuInstruction::decode(opcode, &modrm).ok_or(VmError::UnsupportedInstruction(opcode))?;

    match instruction {
        FpuInstruction::Fadd => fadd::fadd_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Faddp => fadd::faddp(vm, &modrm)?,
        FpuInstruction::Fmul => fmul::fmul_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fmulp => fmul::fmulp(vm, &modrm)?,
        FpuInstruction::Fdiv => fdiv::fdiv(vm, &modrm)?,
        FpuInstruction::Fdivr => fdiv::fdivr(vm, &modrm)?,
        FpuInstruction::Fsubp => fsub::fsubp(vm, &modrm)?,
        FpuInstruction::Fld => fld::fld_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FldM32 => fld::fld_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FldSt => fld::fld_st(vm, &modrm)?,
        FpuInstruction::FstM32 => fstp::fst_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FstpM32 => fstp::fstp_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fstp => fstp::fstp_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FstpSt => fstp::fstp_st(vm, &modrm)?,
        FpuInstruction::Fild => fild::fild_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FildM64 => fild::fild_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fist => fistp::fist_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fistp => fistp::fistp_m32(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::FistpM64 => fistp::fistp_m64(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fldcw => fldcw::fldcw(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fstcw => fstcw::fstcw(vm, &modrm, prefixes.segment_base)?,
        FpuInstruction::Fnclex => {
            // Clear FPU exception flags - clear exception bits in status word
            let status = vm.fpu_status();
            vm.fpu_set_status(status & 0xFF00); // Clear lower 8 bits (exception flags)
        }
        FpuInstruction::Fninit => {
            // Initialize FPU
            vm.fpu_reset();
        }
        FpuInstruction::Frndint => {
            let value = vm.fpu_st(0)?;
            if std::env::var("PE_VM_TRACE_FPU").is_ok() {
                eprintln!(
                    "[pe_vm] FRNDINT value={value} eip=0x{:08X}",
                    vm.eip()
                );
            }
            vm.fpu_set_st(0, value.round())?;
        }
        FpuInstruction::Fsqrt => {
            let value = vm.fpu_st(0)?;
            vm.fpu_set_st(0, value.sqrt())?;
        }
        FpuInstruction::Fucompp => {
            let left = vm.fpu_st(0)?;
            let right = vm.fpu_st(1)?;
            let status = vm.fpu_status();
            let cleared = status & !(C0 | C1 | C2 | C3);
            let next = match left.partial_cmp(&right) {
                Some(std::cmp::Ordering::Less) => cleared | C0,
                Some(std::cmp::Ordering::Equal) => cleared | C3,
                Some(std::cmp::Ordering::Greater) => cleared,
                None => cleared | (C0 | C2 | C3),
            };
            vm.fpu_set_status(next);
            let _ = vm.fpu_pop()?;
            let _ = vm.fpu_pop()?;
        }
        FpuInstruction::Fnstsw => {
            let status = vm.fpu_status();
            vm.set_reg16(REG_EAX, status);
        }
    }

    vm.set_eip(next);
    Ok(())
}

const C0: u16 = 1 << 8;
const C1: u16 = 1 << 9;
const C2: u16 = 1 << 10;
const C3: u16 = 1 << 14;
