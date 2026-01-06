//! VARIANT helpers.

use crate::vm::{Vm, VmError};

use super::bstr::read_bstr;
use super::constants::{
    DISP_E_TYPEMISMATCH, E_INVALIDARG, S_OK, VARIANT_SIZE, VT_BSTR, VT_EMPTY, VT_I4, VT_UI4,
};

// VariantInit(ptr)
pub(super) fn variant_init(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return E_INVALIDARG;
    }
    let _ = vm.write_bytes(ptr, &[0u8; VARIANT_SIZE]);
    S_OK
}

// VariantClear(ptr)
pub(super) fn variant_clear(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if ptr == 0 {
        return E_INVALIDARG;
    }
    let _ = vm.write_bytes(ptr, &[0u8; VARIANT_SIZE]);
    S_OK
}

// VariantChangeType(dest, src, flags, vt)
pub(super) fn variant_change_type(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dest = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let vt = vm.read_u32(stack_ptr + 16).unwrap_or(0) as u16;
    if dest == 0 || src == 0 {
        return E_INVALIDARG;
    }
    let src_vt = vm.read_u16(src).unwrap_or(VT_EMPTY);
    let result = match (src_vt, vt) {
        (VT_I4, VT_I4) | (VT_UI4, VT_UI4) => {
            let value = vm.read_u32(src + 8).unwrap_or(0);
            write_variant_u32(vm, dest, vt, value)
        }
        (VT_BSTR, VT_I4) | (VT_BSTR, VT_UI4) => {
            let ptr = vm.read_u32(src + 8).unwrap_or(0);
            let text = read_bstr(vm, ptr).unwrap_or_default();
            let parsed = text.trim().parse::<u32>().unwrap_or(0);
            write_variant_u32(vm, dest, vt, parsed)
        }
        _ => Err(VmError::InvalidConfig("variant change type unsupported")),
    };
    match result {
        Ok(()) => S_OK,
        Err(_) => DISP_E_TYPEMISMATCH,
    }
}

// Write a VARIANT with a 32-bit value.
pub(super) fn write_variant_u32(vm: &mut Vm, dest: u32, vt: u16, value: u32) -> Result<(), VmError> {
    vm.write_u16(dest, vt)?;
    vm.write_u16(dest + 2, 0)?;
    vm.write_u16(dest + 4, 0)?;
    vm.write_u16(dest + 6, 0)?;
    vm.write_u32(dest + 8, value)?;
    vm.write_u32(dest + 12, 0)?;
    Ok(())
}
