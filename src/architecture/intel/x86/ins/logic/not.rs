use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{
    read_rm16,
    read_rm32,
    read_rm8,
    write_rm16,
    write_rm32,
    write_rm8,
    ModRm,
    Prefixes,
};

pub(crate) fn not_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    write_rm8(vm, modrm, prefixes.segment_base, !value)?;
    Ok(())
}

pub(crate) fn not_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    // Honor operand-size override for 16-bit NOT.
    if prefixes.operand_size_16 {
        let value = read_rm16(vm, modrm, prefixes.segment_base)?;
        write_rm16(vm, modrm, prefixes.segment_base, !value)?;
    } else {
        let value = read_rm32(vm, modrm, prefixes.segment_base)?;
        write_rm32(vm, modrm, prefixes.segment_base, !value)?;
    }
    Ok(())
}
