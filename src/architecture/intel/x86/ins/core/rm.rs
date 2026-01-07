use crate::vm::{Vm, VmError};

use super::decode::{calc_ea, ModRm};

pub(crate) fn segment_value(reg: u8) -> u16 {
    match reg & 0x7 {
        0 => 0, // ES
        1 => 0, // CS
        2 => 0, // SS
        3 => 0, // DS
        4 => 0, // FS
        5 => 0, // GS
        _ => 0,
    }
}

pub(crate) fn read_rm32(vm: &Vm, modrm: &ModRm, segment_base: u32) -> Result<u32, VmError> {
    if modrm.mod_bits == 3 {
        Ok(vm.reg32(modrm.rm))
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.read_u32(addr)
    }
}

pub(crate) fn write_rm32(
    vm: &mut Vm,
    modrm: &ModRm,
    segment_base: u32,
    value: u32,
) -> Result<(), VmError> {
    if modrm.mod_bits == 3 {
        vm.set_reg32(modrm.rm, value);
        Ok(())
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.write_u32(addr, value)
    }
}

pub(crate) fn read_rm16(vm: &Vm, modrm: &ModRm, segment_base: u32) -> Result<u16, VmError> {
    if modrm.mod_bits == 3 {
        Ok(vm.reg16(modrm.rm))
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.read_u16(addr)
    }
}

pub(crate) fn write_rm16(
    vm: &mut Vm,
    modrm: &ModRm,
    segment_base: u32,
    value: u16,
) -> Result<(), VmError> {
    if modrm.mod_bits == 3 {
        vm.set_reg16(modrm.rm, value);
        Ok(())
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.write_u16(addr, value)
    }
}

pub(crate) fn read_rm8(vm: &Vm, modrm: &ModRm, segment_base: u32) -> Result<u8, VmError> {
    if modrm.mod_bits == 3 {
        Ok(vm.reg8(modrm.rm))
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.read_u8(addr)
    }
}

pub(crate) fn write_rm8(
    vm: &mut Vm,
    modrm: &ModRm,
    segment_base: u32,
    value: u8,
) -> Result<(), VmError> {
    if modrm.mod_bits == 3 {
        vm.set_reg8(modrm.rm, value);
        Ok(())
    } else {
        let addr = calc_ea(vm, modrm, segment_base)?;
        vm.write_u8(addr, value)
    }
}
