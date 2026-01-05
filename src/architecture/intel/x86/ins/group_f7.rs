//! x86 group F7 instruction handlers.

use crate::vm::{Vm, VmError, REG_EAX, REG_EDX};

use super::core::{decode_modrm, read_rm32, update_flags_logic32, Prefixes};
use super::{logic, sub};

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    match modrm.reg {
        0 => {
            let imm = vm.read_u32(cursor + 1 + modrm.len as u32)?;
            let lhs = read_rm32(vm, &modrm, prefixes.segment_base)?;
            let result = lhs & imm;
            update_flags_logic32(vm, result);
            vm.set_eip(cursor + 1 + modrm.len as u32 + 4);
        }
        2 => {
            logic::not_rm32(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        3 => {
            sub::neg_rm32(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        4 => {
            let value = read_rm32(vm, &modrm, prefixes.segment_base)?;
            let eax = vm.reg32(REG_EAX);
            let result = (eax as u64) * (value as u64);
            vm.set_reg32(REG_EAX, result as u32);
            vm.set_reg32(REG_EDX, (result >> 32) as u32);
            let overflow = (result >> 32) != 0;
            vm.set_flags(vm.zf(), vm.sf(), overflow, overflow);
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        5 => {
            let value = read_rm32(vm, &modrm, prefixes.segment_base)? as i32 as i64;
            let eax = vm.reg32(REG_EAX) as i32 as i64;
            let result = eax.wrapping_mul(value);
            let low = result as u32;
            let high = (result >> 32) as u32;
            vm.set_reg32(REG_EAX, low);
            vm.set_reg32(REG_EDX, high);
            let sign_ext = if (low & 0x8000_0000) != 0 {
                0xFFFF_FFFF
            } else {
                0
            };
            let overflow = high != sign_ext;
            vm.set_flags(vm.zf(), vm.sf(), overflow, overflow);
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        6 => {
            let divisor = read_rm32(vm, &modrm, prefixes.segment_base)?;
            if divisor == 0 {
                return Err(VmError::DivideError);
            }
            let dividend = ((vm.reg32(REG_EDX) as u64) << 32) | (vm.reg32(REG_EAX) as u64);
            let quotient = dividend / divisor as u64;
            if quotient > u32::MAX as u64 {
                return Err(VmError::DivideError);
            }
            let remainder = dividend % divisor as u64;
            vm.set_reg32(REG_EAX, quotient as u32);
            vm.set_reg32(REG_EDX, remainder as u32);
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        7 => {
            let divisor = read_rm32(vm, &modrm, prefixes.segment_base)? as i32 as i64;
            if divisor == 0 {
                return Err(VmError::DivideError);
            }
            let high = vm.reg32(REG_EDX) as i32 as i64;
            let low = vm.reg32(REG_EAX) as i64;
            let dividend = (high << 32) | (low & 0xFFFF_FFFF);
            let quotient = dividend / divisor;
            if quotient < i32::MIN as i64 || quotient > i32::MAX as i64 {
                return Err(VmError::DivideError);
            }
            let remainder = dividend % divisor;
            vm.set_reg32(REG_EAX, quotient as u32);
            vm.set_reg32(REG_EDX, remainder as u32);
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        _ => return Err(VmError::UnsupportedInstruction(0xF7)),
    }
    Ok(())
}
