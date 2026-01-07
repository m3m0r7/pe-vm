//! DD opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_f64, write_f64};

pub(super) fn handle(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        0 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = read_f64(vm, addr)?;
            vm.fpu_push(value)
        }
        3 => {
            let addr = mem_address(vm, modrm, segment)?;
            let value = vm.fpu_pop()?;
            write_f64(vm, addr, value)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDD)),
    }
}
