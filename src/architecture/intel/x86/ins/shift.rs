//! x86 shift and rotate instruction handlers.

use crate::vm::{Vm, VmError, REG_CL};

use super::core::{decode_modrm, read_rm32, read_rm8, write_rm32, write_rm8, ModRm, Prefixes};

fn exec_shift_rm32(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    count: u32,
    opcode: u8,
) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let result = value.rotate_left(count);
            let cf = (result & 1) != 0;
            let of = if count == 1 {
                ((result >> 31) & 1) != (cf as u32)
            } else {
                vm.of()
            };
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
        }
        1 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let result = value.rotate_right(count);
            let cf = (result & 0x8000_0000) != 0;
            let of = if count == 1 {
                ((result >> 31) ^ (result >> 30)) & 1 != 0
            } else {
                vm.of()
            };
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        2 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let cf_in = if vm.cf() { 1u64 } else { 0 };
            let full = (cf_in << 32) | value as u64;
            let rotated = ((full << count) | (full >> (33 - count))) & ((1u64 << 33) - 1);
            let result = rotated as u32;
            let cf = ((rotated >> 32) & 1) != 0;
            let of = if count == 1 {
                ((result >> 31) & 1) != (cf as u32)
            } else {
                vm.of()
            };
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        3 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let cf_in = if vm.cf() { 1u64 } else { 0 };
            let full = (cf_in << 32) | value as u64;
            let rotated = ((full >> count) | (full << (33 - count))) & ((1u64 << 33) - 1);
            let result = rotated as u32;
            let cf = ((rotated >> 32) & 1) != 0;
            let of = if count == 1 {
                ((result >> 31) ^ (result >> 30)) & 1 != 0
            } else {
                vm.of()
            };
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        4 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let result = value << count;
            let cf = ((value >> (32 - count)) & 1) != 0;
            let of = if count == 1 {
                ((result ^ value) & 0x8000_0000) != 0
            } else {
                vm.of()
            };
            vm.set_flags(result == 0, (result & 0x8000_0000) != 0, of, cf);
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
        }
        5 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let result = value >> count;
            let cf = ((value >> (count - 1)) & 1) != 0;
            let of = if count == 1 {
                (value & 0x8000_0000) != 0
            } else {
                false
            };
            vm.set_flags(result == 0, (result & 0x8000_0000) != 0, of, cf);
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
        }
        6 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let result = value << count;
            let cf = ((value >> (32 - count)) & 1) != 0;
            let of = if count == 1 {
                ((result ^ value) & 0x8000_0000) != 0
            } else {
                vm.of()
            };
            vm.set_flags(result == 0, (result & 0x8000_0000) != 0, of, cf);
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
        }
        7 => {
            let value = read_rm32(vm, modrm, prefixes.segment_base)?;
            let signed = value as i32;
            let result = (signed >> count) as u32;
            let cf = ((value >> (count - 1)) & 1) != 0;
            vm.set_flags(result == 0, (result & 0x8000_0000) != 0, false, cf);
            write_rm32(vm, modrm, prefixes.segment_base, result)?;
        }
        _ => return Err(VmError::UnsupportedInstruction(opcode)),
    }
    Ok(())
}

fn exec_shift_rm8(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    count: u32,
    opcode: u8,
) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let count = count % 8;
            if count == 0 {
                return Ok(());
            }
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let result = value.rotate_left(count);
            let cf = (result & 1) != 0;
            let of = if count == 1 {
                ((result >> 7) & 1) != (cf as u8)
            } else {
                vm.of()
            };
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
        }
        1 => {
            let count = count % 8;
            if count == 0 {
                return Ok(());
            }
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let result = value.rotate_right(count);
            let cf = (result & 0x80) != 0;
            let of = if count == 1 {
                ((result >> 7) ^ (result >> 6)) & 1 != 0
            } else {
                vm.of()
            };
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        2 => {
            let count = count % 9;
            if count == 0 {
                return Ok(());
            }
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let cf_in = if vm.cf() { 1u16 } else { 0 };
            let full = (cf_in << 8) | value as u16;
            let rotated = ((full << count) | (full >> (9 - count))) & ((1u16 << 9) - 1);
            let result = rotated as u8;
            let cf = ((rotated >> 8) & 1) != 0;
            let of = if count == 1 {
                ((result >> 7) & 1) != (cf as u8)
            } else {
                vm.of()
            };
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        3 => {
            let count = count % 9;
            if count == 0 {
                return Ok(());
            }
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let cf_in = if vm.cf() { 1u16 } else { 0 };
            let full = (cf_in << 8) | value as u16;
            let rotated = ((full >> count) | (full << (9 - count))) & ((1u16 << 9) - 1);
            let result = rotated as u8;
            let cf = ((rotated >> 8) & 1) != 0;
            let of = if count == 1 {
                ((result >> 7) ^ (result >> 6)) & 1 != 0
            } else {
                vm.of()
            };
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
            vm.set_flags(vm.zf(), vm.sf(), of, cf);
        }
        4 => {
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let (result, cf) = if count >= 8 {
                (0, false)
            } else {
                (value.wrapping_shl(count), ((value >> (8 - count)) & 1) != 0)
            };
            let of = if count == 1 {
                ((result ^ value) & 0x80) != 0
            } else {
                vm.of()
            };
            vm.set_flags(result == 0, (result & 0x80) != 0, of, cf);
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
        }
        5 => {
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let (result, cf) = if count >= 8 {
                (0, false)
            } else {
                (value.wrapping_shr(count), ((value >> (count - 1)) & 1) != 0)
            };
            let of = if count == 1 {
                (value & 0x80) != 0
            } else {
                false
            };
            vm.set_flags(result == 0, (result & 0x80) != 0, of, cf);
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
        }
        6 => {
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let (result, cf) = if count >= 8 {
                (0, false)
            } else {
                (value.wrapping_shl(count), ((value >> (8 - count)) & 1) != 0)
            };
            let of = if count == 1 {
                ((result ^ value) & 0x80) != 0
            } else {
                vm.of()
            };
            vm.set_flags(result == 0, (result & 0x80) != 0, of, cf);
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
        }
        7 => {
            let value = read_rm8(vm, modrm, prefixes.segment_base)?;
            let (result, cf) = if count >= 8 {
                (if (value & 0x80) != 0 { 0xFF } else { 0x00 }, false)
            } else {
                (
                    ((value as i8) >> count) as u8,
                    ((value >> (count - 1)) & 1) != 0,
                )
            };
            vm.set_flags(result == 0, (result & 0x80) != 0, false, cf);
            write_rm8(vm, modrm, prefixes.segment_base, result)?;
        }
        _ => return Err(VmError::UnsupportedInstruction(opcode)),
    }
    Ok(())
}

pub(crate) fn shift_rm8_imm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let imm = vm.read_u8(cursor + 1 + modrm.len as u32)?;
    let count = (imm & 0x1f) as u32;
    if count == 0 {
        vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
        return Ok(());
    }
    exec_shift_rm8(vm, &modrm, prefixes, count, 0xC0)?;
    vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
    Ok(())
}

pub(crate) fn shift_rm32_imm8(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let imm = vm.read_u8(cursor + 1 + modrm.len as u32)?;
    let count = (imm & 0x1f) as u32;
    if count == 0 {
        vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
        return Ok(());
    }
    exec_shift_rm32(vm, &modrm, prefixes, count, 0xC1)?;
    vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
    Ok(())
}

pub(crate) fn shift_rm8_1(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    exec_shift_rm8(vm, &modrm, prefixes, 1, 0xD0)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn shift_rm32_1(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    exec_shift_rm32(vm, &modrm, prefixes, 1, 0xD1)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn shift_rm8_cl(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let count = (vm.reg8(REG_CL) & 0x1f) as u32;
    if count == 0 {
        vm.set_eip(cursor + 1 + modrm.len as u32);
        return Ok(());
    }
    exec_shift_rm8(vm, &modrm, prefixes, count, 0xD2)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}

pub(crate) fn shift_rm32_cl(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    let count = (vm.reg8(REG_CL) & 0x1f) as u32;
    if count == 0 {
        vm.set_eip(cursor + 1 + modrm.len as u32);
        return Ok(());
    }
    exec_shift_rm32(vm, &modrm, prefixes, count, 0xD3)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}
