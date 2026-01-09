//! FSTP - Store floating point value and pop

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, write_f32, write_f64};

/// FSTP m64real (DD /3)
pub(super) fn fstp_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_pop()?;
    if std::env::var("PE_VM_TRACE_FPU").is_ok() {
        eprintln!(
            "[pe_vm] FSTP m64 addr=0x{addr:08X} value={value} eip=0x{:08X}",
            vm.eip()
        );
    }
    write_f64(vm, addr, value)
}

/// FST m32real (D9 /2)
pub(super) fn fst_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_st(0)?;
    write_f32(vm, addr, value)
}

/// FSTP m32real (D9 /3)
pub(super) fn fstp_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = vm.fpu_pop()?;
    write_f32(vm, addr, value)
}

/// FSTP ST(i) (DD D8+i)
pub(super) fn fstp_st(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let value = vm.fpu_st(0)?;
    vm.fpu_set_st(idx, value)?;
    let _ = vm.fpu_pop()?;
    Ok(())
}
