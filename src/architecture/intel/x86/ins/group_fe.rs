//! x86 group FE instruction handlers.

use crate::vm::{Vm, VmError};

use super::add;
use super::core::{decode_modrm, Prefixes};
use super::sub;

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    match modrm.reg {
        0 => {
            add::inc_rm8(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        1 => {
            sub::dec_rm8(vm, &modrm, prefixes)?;
            vm.set_eip(cursor + 1 + modrm.len as u32);
        }
        _ => return Err(VmError::UnsupportedInstruction(0xFE)),
    }
    Ok(())
}
