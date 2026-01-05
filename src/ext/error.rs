//! C ABI error and string helpers.

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

static LAST_ERROR: OnceLock<Mutex<Option<String>>> = OnceLock::new();

pub(crate) fn set_last_error(message: impl Into<String>) {
    let slot = LAST_ERROR.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = slot.lock() {
        *guard = Some(message.into());
    }
}

pub(crate) fn clear_last_error() {
    let slot = LAST_ERROR.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = slot.lock() {
        *guard = None;
    }
}

pub(crate) fn take_last_error() -> Option<String> {
    let slot = LAST_ERROR.get_or_init(|| Mutex::new(None));
    slot.lock().ok().and_then(|mut guard| guard.take())
}

pub(crate) fn alloc_string(value: &str) -> *mut c_char {
    CString::new(value)
        .ok()
        .map_or(std::ptr::null_mut(), CString::into_raw)
}

#[no_mangle]
pub extern "C" fn pevm_last_error() -> *mut c_char {
    match take_last_error() {
        Some(message) => alloc_string(&message),
        None => std::ptr::null_mut(),
    }
}

/// # Safety
/// `ptr` must be allocated by a `pevm_*` API and not freed already.
#[no_mangle]
pub unsafe extern "C" fn pevm_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}
