//! x86 group F6 instruction handlers.

use crate::vm::{Vm, VmError};

use super::core::{decode_modrm, read_rm8, update_flags_logic8, Prefixes};
use super::{logic, sub};

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    match modrm.reg {
        0 => {
            let imm = vm.read_u8(cursor + 1 + modrm.len as u32)?;
            let lhs = read_rm8(vm, &modrm, prefixes.segment_base)?;
            let result = lhs & imm;
            update_flags_logic8(vm, result);
            vm.set_eip(cursor + 1 + modrm.len as u32 + 1);
        }
        2 => {
            logic::not_rm8(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        3 => {
            sub::neg_rm8(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        _ => return Err(VmError::UnsupportedInstruction(0xF6)),
    }
    Ok(())
}
