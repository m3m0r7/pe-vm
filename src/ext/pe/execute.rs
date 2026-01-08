use std::ffi::CStr;
use std::os::raw::c_char;

use crate::vm::{windows, ExecuteOptions, Value, Vm, VmConfig};

use super::super::error::{clear_last_error, set_last_error};
use super::handle::PeHandle;

/// # Safety
/// `handle` and `symbol` must be valid pointers, and `args` must point to
/// `args_len` elements when `args_len > 0`.
#[no_mangle]
pub unsafe extern "C" fn pevm_pe_execute_symbol_u32(
    handle: *const PeHandle,
    symbol: *const c_char,
    args: *const u32,
    args_len: usize,
) -> u32 {
    clear_last_error();
    let handle = match handle_from_ptr(handle) {
        Some(handle) => handle,
        None => {
            set_last_error("handle is null");
            return 0;
        }
    };
    if symbol.is_null() {
        set_last_error("symbol is null");
        return 0;
    }
    let symbol = CStr::from_ptr(symbol);
    let symbol = match symbol.to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("symbol is not valid UTF-8");
            return 0;
        }
    };
    if args_len > 0 && args.is_null() {
        set_last_error("args is null");
        return 0;
    }
    let values = if args_len == 0 {
        Vec::new()
    } else {
        let slice = std::slice::from_raw_parts(args, args_len);
        slice.iter().copied().map(Value::U32).collect()
    };

    let arch = handle.file.architecture();
    let mut vm = match Vm::new(VmConfig::new().architecture(arch)) {
        Ok(vm) => vm,
        Err(err) => {
            set_last_error(format!("failed to create VM: {err}"));
            return 0;
        }
    };
    if let Err(err) = vm.load_image(&handle.file, &handle.image) {
        set_last_error(format!("failed to load image: {err}"));
        return 0;
    }
    windows::register_default(&mut vm);
    if let Err(err) = vm.resolve_imports(&handle.file) {
        set_last_error(format!("failed to resolve imports: {err}"));
        return 0;
    }
    match vm.execute_export_with_values(&handle.file, symbol, &values, ExecuteOptions::new()) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("execution failed: {err}"));
            0
        }
    }
}

fn handle_from_ptr<'a>(ptr: *const PeHandle) -> Option<&'a PeHandle> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_ref() }
    }
}
