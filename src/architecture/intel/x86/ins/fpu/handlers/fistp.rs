//! FISTP - Store integer and pop

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, write_i64};

/// FISTP m32int (DB /3)
/// Store ST(0) to memory as 32-bit integer and pop the FPU stack
pub(super) fn fistp_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_pop()?;
    // Round to nearest integer (default rounding mode)
    let int_value = value.round() as i32;
    vm.write_u32(addr, int_value as u32)?;
    Ok(())
}

/// FIST m32int (DB /2)
/// Store ST(0) to memory as 32-bit integer without popping
pub(super) fn fist_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_st(0)?;
    let int_value = value.round() as i32;
    vm.write_u32(addr, int_value as u32)?;
    Ok(())
}

/// FISTP m64int (DF /7)
/// Store ST(0) to memory as 64-bit integer and pop the FPU stack
pub(super) fn fistp_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_pop()?;
    if std::env::var("PE_VM_TRACE_FPU").is_ok() {
        eprintln!(
            "[pe_vm] FISTP m64 addr=0x{addr:08X} value={value} eip=0x{:08X}",
            vm.eip()
        );
    }
    let int_value = value.round() as i64;
    write_i64(vm, addr, int_value)
}
