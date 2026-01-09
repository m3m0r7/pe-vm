//! FSUBP - Subtract and pop

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

/// FSUBP ST(i), ST(0) (DE E8+i)
pub(super) fn fsubp(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let st0 = vm.fpu_st(0)?;
    let sti = vm.fpu_st(idx)?;
    vm.fpu_set_st(idx, sti - st0)?;
    let _ = vm.fpu_pop()?;
    Ok(())
}
