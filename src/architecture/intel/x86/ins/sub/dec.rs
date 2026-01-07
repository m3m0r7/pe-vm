use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{
    read_rm32, read_rm8, update_flags_sub32, update_flags_sub8, write_rm32, write_rm8, ModRm,
    Prefixes,
};

pub(crate) fn dec_reg(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let reg = opcode - 0x48;
    let value = vm.reg32(reg);
    let result = value.wrapping_sub(1);
    vm.set_reg32(reg, result);
    update_flags_sub32(vm, value, 1, result);
    vm.set_eip(cursor + 1);
    Ok(())
}

pub(crate) fn dec_rm8(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm8(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_sub(1);
    write_rm8(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub8(vm, value, 1, result);
    Ok(())
}

pub(crate) fn dec_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    let value = read_rm32(vm, modrm, prefixes.segment_base)?;
    let result = value.wrapping_sub(1);
    write_rm32(vm, modrm, prefixes.segment_base, result)?;
    update_flags_sub32(vm, value, 1, result);
    Ok(())
}
