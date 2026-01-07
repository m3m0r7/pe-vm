//! FADD - Add floating point
//! FADDP - Add floating point and pop

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_f64};

/// FADD m64real (DC /0)
pub(super) fn fadd_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let operand = read_f64(vm, addr)?;
    let st0 = vm.fpu_st(0)?;
    vm.fpu_set_st(0, st0 + operand)
}

/// FADDP ST(i), ST(0) (DE C0+i)
pub(super) fn faddp(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let st0 = vm.fpu_st(0)?;
    let sti = vm.fpu_st(idx)?;
    vm.fpu_set_st(idx, sti + st0)?;
    let _ = vm.fpu_pop()?;
    Ok(())
}
