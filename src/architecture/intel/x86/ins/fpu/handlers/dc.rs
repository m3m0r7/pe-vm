//! DC opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_f64};

pub(super) fn handle(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    match modrm.reg {
        0 => {
            let operand = read_f64(vm, addr)?;
            let st0 = vm.fpu_st(0)?;
            vm.fpu_set_st(0, st0 + operand)
        }
        1 => {
            let operand = read_f64(vm, addr)?;
            let st0 = vm.fpu_st(0)?;
            vm.fpu_set_st(0, st0 * operand)
        }
        _ => Err(VmError::UnsupportedInstruction(0xDC)),
    }
}
