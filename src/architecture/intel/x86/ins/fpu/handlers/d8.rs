//! D8 opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

pub(super) fn handle(_vm: &mut Vm, _modrm: &ModRm, _segment: u32) -> Result<(), VmError> {
    Err(VmError::UnsupportedInstruction(0xD8))
}
