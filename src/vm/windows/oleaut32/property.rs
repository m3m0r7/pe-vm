//! Property frame stubs.

use crate::vm::Vm;

use super::constants::E_NOTIMPL;

// OleCreatePropertyFrame(...)
pub(super) fn ole_create_property_frame(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

// OleCreateFontIndirect(...)
pub(super) fn ole_create_font_indirect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}
