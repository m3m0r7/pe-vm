//! FSTCW/FNSTCW - Store x87 FPU control word

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::mem_address;

/// FSTCW m2byte (D9 /7)
pub(super) fn fstcw(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    vm.write_u16(addr, vm.fpu_control())?;
    Ok(())
}
