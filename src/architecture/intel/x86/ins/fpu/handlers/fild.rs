//! FILD - Load integer

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_i32};

/// FILD m32int (DB /0)
pub(super) fn fild_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = read_i32(vm, addr)? as f64;
    vm.fpu_push(value)
}
