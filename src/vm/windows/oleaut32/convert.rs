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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_var_ui4_from_str_null_out() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        vm.write_u32(stack + 16, 0).unwrap(); // null output
        let result = var_ui4_from_str(&mut vm, stack);
        assert_eq!(result, E_INVALIDARG);
    }

    #[test]
    fn test_var_ui4_from_str_valid_number() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let out_ptr = vm.heap_start as u32;
        // Create BSTR with "42"
        let bstr = alloc_bstr(&mut vm, "42").unwrap();
        vm.write_u32(stack + 4, bstr).unwrap();
        vm.write_u32(stack + 16, out_ptr).unwrap();
        let result = var_ui4_from_str(&mut vm, stack);
        assert_eq!(result, S_OK);
        assert_eq!(vm.read_u32(out_ptr).unwrap(), 42);
    }

    #[test]
    fn test_var_ui4_from_str_invalid_number() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let out_ptr = vm.heap_start as u32;
        // Create BSTR with "abc"
        let bstr = alloc_bstr(&mut vm, "abc").unwrap();
        vm.write_u32(stack + 4, bstr).unwrap();
        vm.write_u32(stack + 16, out_ptr).unwrap();
        let result = var_ui4_from_str(&mut vm, stack);
        assert_eq!(result, DISP_E_TYPEMISMATCH);
    }

    #[test]
    fn test_var_bstr_cat_null_out() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm.write_u32(stack + 12, 0).unwrap(); // null output
        let result = var_bstr_cat(&mut vm, stack);
        assert_eq!(result, E_INVALIDARG);
    }

    #[test]
    fn test_var_bstr_cat_success() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        // Create BSTRs
        let left = alloc_bstr(&mut vm, "Hello").unwrap();
        let right = alloc_bstr(&mut vm, "World").unwrap();
        vm.write_u32(stack + 4, left).unwrap();
        vm.write_u32(stack + 8, right).unwrap();
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = var_bstr_cat(&mut vm, stack);
        assert_eq!(result, S_OK);
        // Read the result BSTR
        let result_ptr = vm.read_u32(out_ptr).unwrap();
        let result_str = read_bstr(&vm, result_ptr).unwrap();
        assert_eq!(result_str, "HelloWorld");
    }

    #[test]
    fn test_var_bstr_cat_null_inputs() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        // Both inputs are null (0)
        vm.write_u32(stack + 4, 0).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = var_bstr_cat(&mut vm, stack);
        assert_eq!(result, S_OK);
        // Result should be empty string
        let result_ptr = vm.read_u32(out_ptr).unwrap();
        let result_str = read_bstr(&vm, result_ptr).unwrap();
        assert_eq!(result_str, "");
    }
}
