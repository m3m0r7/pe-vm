//! x86 bit test and modify instruction handlers.

use crate::vm::{Vm, VmError};

use super::core::{decode_modrm, read_rm32, write_rm32, Prefixes};

pub(crate) fn group_ba(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let imm = vm.read_u8(cursor + 2 + modrm.len as u32)?;
    let bit = (imm & 0x1f) as u32;
    let mask = 1u32 << bit;
    let value = read_rm32(vm, &modrm, prefixes.segment_base)?;
    let bit_set = (value & mask) != 0;
    let mut new_value = value;
    match modrm.reg {
        4 => {}
        5 => new_value = value | mask,
        6 => new_value = value & !mask,
        7 => new_value = value ^ mask,
        _ => return Err(VmError::UnsupportedInstruction(0xBA)),
    }
    if modrm.reg != 4 {
        write_rm32(vm, &modrm, prefixes.segment_base, new_value)?;
    }
    vm.set_flags(vm.zf(), vm.sf(), vm.of(), bit_set);
    vm.set_eip(cursor + 2 + modrm.len as u32 + 1);
    Ok(())
}
