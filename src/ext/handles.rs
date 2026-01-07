//! Opaque handle types for the C ABI.

use crate::vm::windows;
use crate::vm::Vm;

#[repr(C)]
pub struct VmHandle {
    pub(crate) vm: Vm,
}

#[repr(C)]
pub struct ComHandle {
    pub(crate) com: windows::com::Com,
}

#[repr(C)]
pub struct ComObjectHandle {
    pub(crate) obj: windows::com::ComObject,
}

pub(crate) fn vm_from_ptr<'a>(ptr: *const VmHandle) -> Option<&'a Vm> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_ref().map(|handle| &handle.vm) }
    }
}

pub(crate) fn vm_from_ptr_mut<'a>(ptr: *mut VmHandle) -> Option<&'a mut Vm> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_mut().map(|handle| &mut handle.vm) }
    }
}

pub(crate) fn com_from_ptr<'a>(ptr: *const ComHandle) -> Option<&'a windows::com::Com> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_ref().map(|handle| &handle.com) }
    }
}

pub(crate) fn com_object_from_ptr<'a>(
    ptr: *const ComObjectHandle,
) -> Option<&'a windows::com::ComObject> {
    if ptr.is_null() {
        None
    } else {
        unsafe { ptr.as_ref().map(|handle| &handle.obj) }
    }
}
