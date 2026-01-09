//! FDIV/FDIVR - Divide

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

/// FDIV ST(0), ST(i) (D8 F0+i)
pub(super) fn fdiv(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let st0 = vm.fpu_st(0)?;
    let sti = vm.fpu_st(idx)?;
    vm.fpu_set_st(0, st0 / sti)
}

/// FDIVR ST(0), ST(i) (D8 F8+i)
pub(super) fn fdivr(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let st0 = vm.fpu_st(0)?;
    let sti = vm.fpu_st(idx)?;
    vm.fpu_set_st(0, sti / st0)
}
