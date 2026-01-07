//! DB opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_i32};

pub(super) fn handle(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = read_i32(vm, addr)? as f64;
            vm.fpu_push(value)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDB)),
    }
}
