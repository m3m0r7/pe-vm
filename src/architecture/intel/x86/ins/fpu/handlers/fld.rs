//! FLD - Load floating point value

use crate::architecture::intel::x86::ins::core::ModRm;
use crate::vm::{Vm, VmError};

use super::{mem_address, read_f32, read_f64};

/// FLD m64real (DD /0)
pub(super) fn fld_m64(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = read_f64(vm, addr)?;
    if std::env::var("PE_VM_TRACE_FPU").is_ok() {
        eprintln!(
            "[pe_vm] FLD m64 addr=0x{addr:08X} value={value} eip=0x{:08X}",
            vm.eip()
        );
    }
    vm.fpu_push(value)
}

/// FLD m32real (D9 /0)
pub(super) fn fld_m32(vm: &mut Vm, modrm: &ModRm, segment: u32) -> Result<(), VmError> {
    let addr = mem_address(vm, modrm, segment)?;
    let value = read_f32(vm, addr)?;
    if std::env::var("PE_VM_TRACE_FPU").is_ok() {
        eprintln!(
            "[pe_vm] FLD m32 addr=0x{addr:08X} value={value} eip=0x{:08X}",
            vm.eip()
        );
    }
    vm.fpu_push(value)
}

/// FLD ST(i) (D9 C0+i)
pub(super) fn fld_st(vm: &mut Vm, modrm: &ModRm) -> Result<(), VmError> {
    let idx = modrm.rm as usize;
    let value = vm.fpu_st(idx)?;
    vm.fpu_push(value)
}
