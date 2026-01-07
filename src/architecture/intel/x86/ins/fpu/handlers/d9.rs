//! D9 opcode handler

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::mem_address;

pub(super) fn handle(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    match modrm.reg {
        5 => {
            let addr = mem_address(vm, modrm, segment)?;
            let word = vm.read_u16(addr)?;
            vm.fpu_set_control(word);
            Ok(())
        }
        7 => {
            let addr = mem_address(vm, modrm, segment)?;
            vm.write_u16(addr, vm.fpu_control())?;
            Ok(())
        }
        _ => Err(VmError::UnsupportedInstruction(0xD9)),
    }
}
