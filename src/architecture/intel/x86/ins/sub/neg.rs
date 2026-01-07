use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{
    read_rm32, read_rm8, update_flags_sub32, update_flags_sub8, write_rm32, write_rm8, ModRm,
    Prefixes,
};

pub(crate) fn neg_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = (0u8).wrapping_sub(value);
    update_flags_sub8(vm, 0, value, result);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}

pub(crate) fn neg_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = (0u32).wrapping_sub(value);
    update_flags_sub32(vm, 0, value, result);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    Ok(())
}
