//! SafeArray stubs.

use crate::vm::Vm;

use super::constants::S_OK;

// SafeArrayCreate(...)
pub(super) fn safe_array_create(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

// SafeArrayAccessData(...)
pub(super) fn safe_array_access_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

// SafeArrayUnaccessData(...)
pub(super) fn safe_array_unaccess_data(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}
