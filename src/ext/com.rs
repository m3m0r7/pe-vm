//! COM C ABI helpers for in-proc automation.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::vm::windows;

use super::args::parse_com_args;
use super::error::{alloc_string, clear_last_error, set_last_error};
use super::handles::{
    com_from_ptr, com_object_from_ptr, vm_from_ptr_mut, ComHandle, ComObjectHandle,
};

#[no_mangle]
pub extern "C" fn pevm_com_create() -> *mut ComHandle {
    clear_last_error();
    Box::into_raw(Box::new(ComHandle {
        com: windows::com::Com::new(),
    }))
}

/// # Safety
/// `handle` must be returned by `pevm_com_create` and not freed yet.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_close(handle: *mut ComHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// # Safety
/// `com`, `vm`, and `clsid` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_create_instance_inproc(
    com: *const ComHandle,
    vm: *mut super::handles::VmHandle,
    clsid: *const c_char,
) -> *mut ComObjectHandle {
    clear_last_error();
    let Some(com) = com_from_ptr(com) else {
        set_last_error("com handle is null");
        return std::ptr::null_mut();
    };
    let Some(vm) = vm_from_ptr_mut(vm) else {
        set_last_error("vm handle is null");
        return std::ptr::null_mut();
    };
    if clsid.is_null() {
        set_last_error("clsid is null");
        return std::ptr::null_mut();
    }
    let clsid = match CStr::from_ptr(clsid).to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("clsid is not valid UTF-8");
            return std::ptr::null_mut();
        }
    };
    let obj = match com.create_instance_inproc(vm, clsid) {
        Ok(obj) => obj,
        Err(err) => {
            set_last_error(format!("failed to create COM instance: {err}"));
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(ComObjectHandle { obj }))
}

/// # Safety
/// `handle` must be returned by `pevm_com_create_instance_inproc` and not freed yet.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_object_close(handle: *mut ComObjectHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// # Safety
/// `obj`, `vm`, and `args` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_object_invoke_i4(
    obj: *const ComObjectHandle,
    vm: *mut super::handles::VmHandle,
    dispid: u32,
    args: *const super::args::PevmComArg,
    args_len: usize,
) -> i32 {
    clear_last_error();
    let Some(obj) = com_object_from_ptr(obj) else {
        set_last_error("com object handle is null");
        return 0;
    };
    let Some(vm) = vm_from_ptr_mut(vm) else {
        set_last_error("vm handle is null");
        return 0;
    };
    let args = match parse_com_args(args, args_len) {
        Ok(args) => args,
        Err(err) => {
            set_last_error(err);
            return 0;
        }
    };
    vm.clear_last_com_out_params();
    match obj.invoke_i4(vm, dispid, &args) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("invoke_i4 failed: {err}"));
            0
        }
    }
}

/// # Safety
/// `obj`, `vm`, and `args` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_object_invoke_bstr(
    obj: *const ComObjectHandle,
    vm: *mut super::handles::VmHandle,
    dispid: u32,
    args: *const super::args::PevmComArg,
    args_len: usize,
) -> *mut c_char {
    clear_last_error();
    let Some(obj) = com_object_from_ptr(obj) else {
        set_last_error("com object handle is null");
        return std::ptr::null_mut();
    };
    let Some(vm) = vm_from_ptr_mut(vm) else {
        set_last_error("vm handle is null");
        return std::ptr::null_mut();
    };
    let args = match parse_com_args(args, args_len) {
        Ok(args) => args,
        Err(err) => {
            set_last_error(err);
            return std::ptr::null_mut();
        }
    };
    vm.clear_last_com_out_params();
    match obj.invoke_bstr(vm, dispid, &args) {
        Ok(value) => alloc_string(&value),
        Err(err) => {
            set_last_error(format!("invoke_bstr failed: {err}"));
            std::ptr::null_mut()
        }
    }
}

/// # Safety
/// `obj`, `vm`, and `args` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_com_object_invoke_void(
    obj: *const ComObjectHandle,
    vm: *mut super::handles::VmHandle,
    dispid: u32,
    args: *const super::args::PevmComArg,
    args_len: usize,
) -> bool {
    clear_last_error();
    let Some(obj) = com_object_from_ptr(obj) else {
        set_last_error("com object handle is null");
        return false;
    };
    let Some(vm) = vm_from_ptr_mut(vm) else {
        set_last_error("vm handle is null");
        return false;
    };
    let args = match parse_com_args(args, args_len) {
        Ok(args) => args,
        Err(err) => {
            set_last_error(err);
            return false;
        }
    };
    vm.clear_last_com_out_params();
    match obj.invoke_void(vm, dispid, &args) {
        Ok(()) => true,
        Err(err) => {
            set_last_error(format!("invoke_void failed: {err}"));
            false
        }
    }
}
