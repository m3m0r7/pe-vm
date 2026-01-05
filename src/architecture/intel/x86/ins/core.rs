//! x86 instruction decoding and helpers.

use crate::vm::{Vm, VmError};

#[derive(Default, Clone, Copy)]
pub(crate) struct Prefixes {
    pub(crate) segment_base: u32,
    pub(crate) operand_size_16: bool,
    pub(crate) rep: bool,
    pub(crate) repne: bool,
}

pub(crate) fn parse_prefixes(vm: &Vm, cursor: u32) -> Result<(u32, Prefixes), VmError> {
    let mut cursor = cursor;
    let mut prefixes = Prefixes::default();
    loop {
        let byte = vm.read_u8(cursor)?;
        match byte {
            0xF0 | 0xF2 | 0xF3 => {
                if byte == 0xF2 {
                    prefixes.repne = true;
                    prefixes.rep = false;
                } else if byte == 0xF3 {
                    prefixes.rep = true;
                    prefixes.repne = false;
                }
                cursor = cursor.wrapping_add(1);
            }
            0x64 => {
                prefixes.segment_base = vm.fs_base();
                cursor = cursor.wrapping_add(1);
            }
            0x65 => {
                prefixes.segment_base = vm.gs_base();
                cursor = cursor.wrapping_add(1);
            }
            0x66 => {
                prefixes.operand_size_16 = true;
                cursor = cursor.wrapping_add(1);
            }
            _ => break,
        }
    }
    Ok((cursor, prefixes))
}

#[derive(Debug, Clone)]
pub(crate) struct ModRm {
    pub(crate) mod_bits: u8,
    pub(crate) reg: u8,
    pub(crate) rm: u8,
    pub(crate) disp: i32,
    pub(crate) sib: Option<Sib>,
    pub(crate) len: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Sib {
    pub(crate) scale: u8,
    pub(crate) index: u8,
    pub(crate) base: u8,
}

pub(crate) fn decode_modrm(vm: &Vm, addr: u32) -> Result<ModRm, VmError> {
    let modrm = vm.read_u8(addr)?;
    let mod_bits = (modrm >> 6) & 0x3;
    let reg = (modrm >> 3) & 0x7;
    let rm = modrm & 0x7;
    let mut len = 1usize;
    let mut sib = None;
    let mut disp = 0i32;

    if mod_bits != 3 && rm == 4 {
        let sib_byte = vm.read_u8(addr + len as u32)?;
        len += 1;
        sib = Some(Sib {
            scale: (sib_byte >> 6) & 0x3,
            index: (sib_byte >> 3) & 0x7,
            base: sib_byte & 0x7,
        });
    }

    match mod_bits {
        0 => {
            if rm == 5 {
                disp = vm.read_u32(addr + len as u32)? as i32;
                len += 4;
            } else if let Some(sib_val) = &sib {
                if sib_val.base == 5 {
                    disp = vm.read_u32(addr + len as u32)? as i32;
                    len += 4;
                }
            }
        }
        1 => {
            disp = vm.read_u8(addr + len as u32)? as i8 as i32;
            len += 1;
        }
        2 => {
            disp = vm.read_u32(addr + len as u32)? as i32;
            len += 4;
        }
        _ => {}
    }

    Ok(ModRm {
        mod_bits,
        reg,
        rm,
        disp,
        sib,
        len,
    })
}

pub(crate) fn calc_ea(vm: &Vm, modrm: &ModRm, segment_base: u32) -> Result<u32, VmError> {
    if modrm.mod_bits == 3 {
        return Err(VmError::UnsupportedInstruction(0));
    }
    let mut base = 0u32;
    if let Some(sib) = &modrm.sib {
        if sib.index != 4 {
            let index = vm.reg32(sib.index);
            base = base.wrapping_add(index << sib.scale);
        }
        if !(modrm.mod_bits == 0 && sib.base == 5) {
            base = base.wrapping_add(vm.reg32(sib.base));
        }
    } else if !(modrm.mod_bits == 0 && modrm.rm == 5) {
        base = base.wrapping_add(vm.reg32(modrm.rm));
    }

    Ok(segment_base.wrapping_add(base).wrapping_add(modrm.disp as u32))
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

pub(crate) fn update_flags_logic32(vm: &mut Vm, result: u32) {
    let zf = result == 0;
    let sf = (result & 0x8000_0000) != 0;
    vm.set_flags(zf, sf, false, false);
}

pub(crate) fn update_flags_logic8(vm: &mut Vm, result: u8) {
    let zf = result == 0;
    let sf = (result & 0x80) != 0;
    vm.set_flags(zf, sf, false, false);
}

pub(crate) fn update_flags_logic16(vm: &mut Vm, result: u16) {
    // Operand-size override uses 16-bit flags.
    let zf = result == 0;
    let sf = (result & 0x8000) != 0;
    vm.set_flags(zf, sf, false, false);
}

pub(crate) fn update_flags_add32(vm: &mut Vm, a: u32, b: u32, result: u32) {
    let sign = 0x8000_0000;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ result) & (b ^ result) & sign) != 0;
    let cf = (a as u64 + b as u64) > 0xFFFF_FFFF;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_sub32(vm: &mut Vm, a: u32, b: u32, result: u32) {
    let sign = 0x8000_0000;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ b) & (a ^ result) & sign) != 0;
    let cf = a < b;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_sub32_with_cf(vm: &mut Vm, a: u32, b: u32, result: u32, cf: bool) {
    let sign = 0x8000_0000;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ b) & (a ^ result) & sign) != 0;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_sub16(vm: &mut Vm, a: u16, b: u16, result: u16) {
    let sign = 0x8000;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ b) & (a ^ result) & sign) != 0;
    let cf = a < b;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_add8(vm: &mut Vm, a: u8, b: u8, result: u8) {
    let sign = 0x80;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ result) & (b ^ result) & sign) != 0;
    let cf = (a as u16 + b as u16) > 0xFF;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_sub8(vm: &mut Vm, a: u8, b: u8, result: u8) {
    let sign = 0x80;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ b) & (a ^ result) & sign) != 0;
    let cf = a < b;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn update_flags_sub8_with_cf(vm: &mut Vm, a: u8, b: u8, result: u8, cf: bool) {
    let sign = 0x80;
    let zf = result == 0;
    let sf = (result & sign) != 0;
    let of = ((a ^ b) & (a ^ result) & sign) != 0;
    vm.set_flags(zf, sf, of, cf);
}

pub(crate) fn sbb32(vm: &mut Vm, a: u32, b: u32) -> (u32, bool) {
    let carry = if vm.cf() { 1u64 } else { 0 };
    let b_total = b as u64 + carry;
    let result = a.wrapping_sub(b_total as u32);
    let cf = (a as u64) < b_total;
    (result, cf)
}

pub(crate) fn sbb8(vm: &mut Vm, a: u8, b: u8) -> (u8, bool) {
    let carry = if vm.cf() { 1u16 } else { 0 };
    let b_total = b as u16 + carry;
    let result = a.wrapping_sub(b_total as u8);
    let cf = (a as u16) < b_total;
    (result, cf)
}

pub(crate) fn pack_eflags(vm: &Vm) -> u32 {
    let mut value = 1u32 << 1;
    if vm.cf() {
        value |= 1;
    }
    if vm.zf() {
        value |= 1 << 6;
    }
    if vm.sf() {
        value |= 1 << 7;
    }
    if vm.of() {
        value |= 1 << 11;
    }
    value
}

pub(crate) fn segment_value(_seg: u8) -> u16 {
    0
}
