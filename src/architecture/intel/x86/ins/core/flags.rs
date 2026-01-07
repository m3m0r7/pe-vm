use crate::vm::Vm;

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
