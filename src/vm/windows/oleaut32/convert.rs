//! Conversion helpers used by automation calls.

use crate::vm::Vm;

use super::bstr::{alloc_bstr, read_bstr};
use super::constants::{DISP_E_TYPEMISMATCH, E_INVALIDARG, S_OK};

// VarUI4FromStr(...)
pub(super) fn var_ui4_from_str(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let str_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let text = read_bstr(vm, str_ptr).unwrap_or_default();
    match text.trim().parse::<u32>() {
        Ok(value) => {
            let _ = vm.write_u32(out_ptr, value);
            S_OK
        }
        Err(_) => DISP_E_TYPEMISMATCH,
    }
}

// VarBstrCat(...)
pub(super) fn var_bstr_cat(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let right_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let left = read_bstr(vm, left_ptr).unwrap_or_default();
    let right = read_bstr(vm, right_ptr).unwrap_or_default();
    let combined = format!("{left}{right}");
    let bstr = alloc_bstr(vm, &combined).unwrap_or(0);
    let _ = vm.write_u32(out_ptr, bstr);
    S_OK
}
