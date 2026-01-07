//! FSTP - Store floating point value and pop

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, write_f64};

/// FSTP m64real (DD /3)
pub(super) fn fstp_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_pop()?;
    write_f64(vm, addr, value)
}
