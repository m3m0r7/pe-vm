//! Minimal x87 opcode support

use crate::vm::{Vm, VmError};

use super::core::{calc_ea, decode_modrm, Prefixes};

fn read_f64(vm: &Vm, addr: u32) -> Result<f64, VmError> {
    let bits = vm.read_u64(addr)?;
    Ok(f64::from_bits(bits))
}

fn write_f64(vm: &mut Vm, addr: u32, value: f64) -> Result<(), VmError> {
    vm.write_u64(addr, value.to_bits())
}

fn read_i32(vm: &Vm, addr: u32) -> Result<i32, VmError> {
    Ok(vm.read_u32(addr)? as i32)
}

fn mem_address(vm: &Vm, modrm: &super::core::ModRm, segment: u32) -> Result<u32, VmError> {
    if modrm.mod_bits == 3 {
        return Err(VmError::UnsupportedInstruction(0));
    }
    calc_ea(vm, modrm, segment)
}

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let modrm = decode_modrm(vm, cursor + 1)?;
    let next = cursor + 1 + modrm.len as u32;
    match opcode {
        0xD8 => handle_d8(vm, &modrm, prefixes.segment_base)?,
        0xD9 => handle_d9(vm, &modrm, prefixes.segment_base)?,
        0xDA => return Err(VmError::UnsupportedInstruction(opcode)),
        0xDB => handle_db(vm, &modrm, prefixes.segment_base)?,
        0xDC => handle_dc(vm, &modrm, prefixes.segment_base)?,
        0xDD => handle_dd(vm, &modrm, prefixes.segment_base)?,
        0xDE => handle_de(vm, &modrm)?,
        0xDF => return Err(VmError::UnsupportedInstruction(opcode)),
        _ => return Err(VmError::UnsupportedInstruction(opcode)),
    }
    vm.set_eip(next);
    Ok(())
}

fn handle_d8(vm: &mut Vm, _modrm: &super::core::ModRm, _segment: u32) -> Result<(), VmError> {
    Err(VmError::UnsupportedInstruction(0xD8))
}

fn handle_d9(vm: &mut Vm, modrm: &super::core::ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        5 => {
            let addr = mem_address(vm, modrm, segment)?;
            let word = vm.read_u16(addr)?;
            vm.fpu_set_control(word);
            Ok(())
        }
        7 => {
            let addr = mem_address(vm, modrm, segment)?;
            vm.write_u16(addr, vm.fpu_control())?;
            Ok(())
        }
        _ => Err(VmError::UnsupportedInstruction(0xD9)),
    }
}

fn handle_db(vm: &mut Vm, modrm: &super::core::ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = read_i32(vm, addr)? as f64;
            vm.fpu_push(value)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDB)),
    }
}

fn handle_dc(vm: &mut Vm, modrm: &super::core::ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    match modrm.reg {
        0 => {
            let operand = read_f64(vm, addr)?;
            let st0 = vm.fpu_st(0)?;
            vm.fpu_set_st(0, st0 + operand)
        }
        1 => {
            let operand = read_f64(vm, addr)?;
            let st0 = vm.fpu_st(0)?;
            vm.fpu_set_st(0, st0 * operand)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDC)),
    }
}

fn handle_dd(vm: &mut Vm, modrm: &super::core::ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = read_f64(vm, addr)?;
            vm.fpu_push(value)
        }
        3 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = vm.fpu_pop()?;
            write_f64(vm, addr, value)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDD)),
    }
}

fn handle_de(vm: &mut Vm, modrm: &super::core::ModRm) -> Result<(), VmError> {
    if modrm.mod_bits != 3 {
        return Err(VmError::UnsupportedInstruction(0xDE));
    }
    let idx = modrm.rm as usize;
    match modrm.reg {
        0 => {
            let st0 = vm.fpu_st(0)?;
            let sti = vm.fpu_st(idx)?;
            vm.fpu_set_st(idx, sti + st0)?;
            let _ = vm.fpu_pop()?;
            Ok(())
        }
        1 => {
            let st0 = vm.fpu_st(0)?;
            let sti = vm.fpu_st(idx)?;
            vm.fpu_set_st(idx, sti * st0)?;
            let _ = vm.fpu_pop()?;
            Ok(())
        }
        _ => Err(VmError::UnsupportedInstruction(0xDE)),
    }
}
