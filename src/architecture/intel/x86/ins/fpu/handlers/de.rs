//! DE opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

pub(super) fn handle(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    if modrm.mod_bits != 3 {
        return Err(VmError::UnsupportedInstruction(0xDE));
    }
    let idx = modrm.rm as usize;
    match modrm.reg {
        0 => {
            let st0 = vm.fpu_st(0)?;
            let sti = vm.fpu_st(idx)?;
            vm.fpu_set_st(idx, sti + st0)?;
            let _ = vm.fpu_pop()?;
            Ok(())
        }
        1 => {
            let st0 = vm.fpu_st(0)?;
            let sti = vm.fpu_st(idx)?;
            vm.fpu_set_st(idx, sti * st0)?;
            let _ = vm.fpu_pop()?;
            Ok(())
        }
        _ => Err(VmError::UnsupportedInstruction(0xDE)),
    }
}
