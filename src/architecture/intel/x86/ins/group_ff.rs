//! x86 group FF instruction handlers.

use crate::vm::{Vm, VmError};

use super::add;
use super::control;
use super::core::{decode_modrm, Prefixes};
use super::stack;
use super::sub;

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    match modrm.reg {
        0 => {
            add::inc_rm32(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        1 => {
            sub::dec_rm32(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        2 => {
            let next = cursor + 1 + modrm.len as u32;
            control::call_rm32(vm, &modrm, prefixes, next)?;
        }
        4 => {
            control::jmp_rm32(vm, &modrm, prefixes)?;
        }
        6 => {
            stack::push_rm32(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        _ => return Err(VmError::UnsupportedInstruction(0xFF)),
    }
    Ok(())
}
