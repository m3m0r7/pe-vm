//! FLD - Load floating point value

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_f64};

/// FLD m64real (DD /0)
pub(super) fn fld_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = read_f64(vm, addr)?;
    vm.fpu_push(value)
}
