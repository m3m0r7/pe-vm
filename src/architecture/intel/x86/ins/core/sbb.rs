use crate::vm::Vm;

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
