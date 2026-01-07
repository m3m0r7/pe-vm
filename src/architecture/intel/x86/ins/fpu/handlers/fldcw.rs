//! FLDCW - Load x87 FPU control word

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::mem_address;

/// FLDCW m2byte (D9 /5)
pub(super) fn fldcw(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let word = vm.read_u16(addr)?;
    vm.fpu_set_control(word);
    Ok(())
}
