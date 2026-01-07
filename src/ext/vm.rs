//! VM setup and memory helpers for the C ABI.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::vm::windows;
use crate::vm::{Architecture, Os, Vm, VmConfig};

use super::error::{alloc_string, clear_last_error, set_last_error};
use super::handles::{vm_from_ptr, vm_from_ptr_mut, VmHandle};

#[no_mangle]
pub extern "C" fn pevm_vm_create(os: u32, arch: u32) -> *mut VmHandle {
    clear_last_error();
    let os = match os {
        0 => Os::Windows,
        1 => Os::Unix,
        2 => Os::Mac,
        _ => {
            set_last_error("invalid os value");
            return std::ptr::null_mut();
        }
    };
    let arch = match arch {
        0 => Architecture::X86,
        1 => Architecture::X86_64,
        _ => {
            set_last_error("invalid architecture value");
            return std::ptr::null_mut();
        }
    };
    let vm = match Vm::new(VmConfig::new().os(os).architecture(arch)) {
        Ok(vm) => vm,
        Err(err) => {
            set_last_error(format!("failed to create VM: {err}"));
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(VmHandle { vm }))
}

/// # Safety
/// `handle` must be returned by `pevm_vm_create` and not freed yet.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_close(handle: *mut VmHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// # Safety
/// `handle`, `guest`, and `host` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_set_path_mapping(
    handle: *mut VmHandle,
    guest: *const c_char,
    host: *const c_char,
) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    if guest.is_null() || host.is_null() {
        set_last_error("path mapping strings are null");
        return false;
    }
    let guest = match CStr::from_ptr(guest).to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("guest path is not valid UTF-8");
            return false;
        }
    };
    let host = match CStr::from_ptr(host).to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("host path is not valid UTF-8");
            return false;
        }
    };
    vm.insert_path_mapping(guest.to_string(), host.to_string());
    true
}

/// # Safety
/// `handle` and `path` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_set_registry_from_reg(
    handle: *mut VmHandle,
    path: *const c_char,
) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    if path.is_null() {
        set_last_error("registry path is null");
        return false;
    }
    let path = match CStr::from_ptr(path).to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("registry path is not valid UTF-8");
            return false;
        }
    };
    let registry = match windows::registry::load_from_registry(path) {
        Ok(registry) => registry,
        Err(err) => {
            set_last_error(format!("failed to load registry: {err}"));
            return false;
        }
    };
    if let Err(err) = vm.set_registry(registry) {
        set_last_error(format!("failed to apply registry: {err}"));
        return false;
    }
    true
}

/// # Safety
/// `handle` and `path` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_set_registry_from_yml(
    handle: *mut VmHandle,
    path: *const c_char,
) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    if path.is_null() {
        set_last_error("registry path is null");
        return false;
    }
    let path = match CStr::from_ptr(path).to_str() {
        Ok(value) => value,
        Err(_) => {
            set_last_error("registry path is not valid UTF-8");
            return false;
        }
    };
    let registry = match windows::registry::load_from_yml(path) {
        Ok(registry) => registry,
        Err(err) => {
            set_last_error(format!("failed to load registry: {err}"));
            return false;
        }
    };
    if let Err(err) = vm.set_registry(registry) {
        set_last_error(format!("failed to apply registry: {err}"));
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn pevm_vm_last_com_out_param_count(handle: *const VmHandle) -> usize {
    vm_from_ptr(handle)
        .map(|vm| vm.last_com_out_params().len())
        .unwrap_or(0)
}

/// # Safety
/// `handle` and output pointers must be valid if non-null.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_last_com_out_param_info(
    handle: *const VmHandle,
    pos: usize,
    out_index: *mut usize,
    out_vt: *mut u16,
    out_flags: *mut u32,
    out_ptr: *mut u32,
) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    let Some(param) = vm.last_com_out_params().get(pos) else {
        set_last_error("out param index is out of range");
        return false;
    };
    if !out_index.is_null() {
        *out_index = param.index;
    }
    if !out_vt.is_null() {
        *out_vt = param.vt;
    }
    if !out_flags.is_null() {
        *out_flags = param.flags;
    }
    if !out_ptr.is_null() {
        *out_ptr = param.ptr;
    }
    true
}

/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_clear_last_com_out_params(handle: *mut VmHandle) {
    if let Some(vm) = vm_from_ptr_mut(handle) {
        vm.clear_last_com_out_params();
    }
}

#[no_mangle]
pub extern "C" fn pevm_vm_read_u8(handle: *const VmHandle, addr: u32) -> u8 {
    clear_last_error();
    let Some(vm) = vm_from_ptr(handle) else {
        set_last_error("vm handle is null");
        return 0;
    };
    match vm.read_u8(addr) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("read_u8 failed: {err}"));
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn pevm_vm_read_u16(handle: *const VmHandle, addr: u32) -> u16 {
    clear_last_error();
    let Some(vm) = vm_from_ptr(handle) else {
        set_last_error("vm handle is null");
        return 0;
    };
    match vm.read_u16(addr) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("read_u16 failed: {err}"));
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn pevm_vm_read_u32(handle: *const VmHandle, addr: u32) -> u32 {
    clear_last_error();
    let Some(vm) = vm_from_ptr(handle) else {
        set_last_error("vm handle is null");
        return 0;
    };
    match vm.read_u32(addr) {
        Ok(value) => value,
        Err(err) => {
            set_last_error(format!("read_u32 failed: {err}"));
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn pevm_vm_read_bstr(handle: *const VmHandle, ptr: u32) -> *mut c_char {
    clear_last_error();
    let Some(vm) = vm_from_ptr(handle) else {
        set_last_error("vm handle is null");
        return std::ptr::null_mut();
    };
    match vm.read_bstr(ptr) {
        Ok(text) => alloc_string(&text),
        Err(err) => {
            set_last_error(format!("read_bstr failed: {err}"));
            std::ptr::null_mut()
        }
    }
}

/// Write an 8-bit value to VM memory.
///
/// # Safety
/// `handle` must be a valid VM handle.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_write_u8(handle: *mut VmHandle, addr: u32, value: u8) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    match vm.write_u8(addr, value) {
        Ok(()) => true,
        Err(err) => {
            set_last_error(format!("write_u8 failed: {err}"));
            false
        }
    }
}

/// Write a 16-bit value to VM memory.
///
/// # Safety
/// `handle` must be a valid VM handle.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_write_u16(handle: *mut VmHandle, addr: u32, value: u16) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    match vm.write_u16(addr, value) {
        Ok(()) => true,
        Err(err) => {
            set_last_error(format!("write_u16 failed: {err}"));
            false
        }
    }
}

/// Write a 32-bit value to VM memory.
///
/// # Safety
/// `handle` must be a valid VM handle.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_write_u32(handle: *mut VmHandle, addr: u32, value: u32) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    match vm.write_u32(addr, value) {
        Ok(()) => true,
        Err(err) => {
            set_last_error(format!("write_u32 failed: {err}"));
            false
        }
    }
}

/// Write a null-terminated C string to VM memory.
///
/// # Safety
/// `handle` and `str_ptr` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn pevm_vm_write_c_string(
    handle: *mut VmHandle,
    addr: u32,
    str_ptr: *const c_char,
) -> bool {
    clear_last_error();
    let Some(vm) = vm_from_ptr_mut(handle) else {
        set_last_error("vm handle is null");
        return false;
    };
    if str_ptr.is_null() {
        set_last_error("string pointer is null");
        return false;
    }
    let cstr = match CStr::from_ptr(str_ptr).to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("string is not valid UTF-8");
            return false;
        }
    };
    let bytes = cstr.as_bytes();
    // Write string bytes followed by null terminator
    for (i, &byte) in bytes.iter().enumerate() {
        if vm.write_u8(addr + i as u32, byte).is_err() {
            set_last_error("write failed during c_string write");
            return false;
        }
    }
    // Write null terminator
    if vm.write_u8(addr + bytes.len() as u32, 0).is_err() {
        set_last_error("write failed for null terminator");
        return false;
    }
    true
}
