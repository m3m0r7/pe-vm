//! Shared helpers for ITypeInfo methods.

use crate::vm::Vm;

use crate::vm::windows::oleaut32::typelib;

fn is_typeinfo_object(vm: &mut Vm, ptr: u32) -> bool {
    if ptr == 0 {
        return false;
    }
    let vtable_ptr = vm.read_u32(ptr).unwrap_or(0);
    if !vm.contains_addr(vtable_ptr) {
        return false;
    }
    let entry = vm.read_u32(vtable_ptr).unwrap_or(0);
    let Some(expected) = vm.resolve_dynamic_import("pe_vm.typeinfo.QueryInterface") else {
        return false;
    };
    entry == expected
}

pub(super) fn resolve_typeinfo_this(vm: &mut Vm, stack_ptr: u32) -> Option<(u32, bool)> {
    let stack_this = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if stack_this != 0 && is_typeinfo_object(vm, stack_this) {
        return Some((stack_this, false));
    }
    let ecx = vm.reg32(crate::vm::REG_ECX);
    if ecx != 0 && is_typeinfo_object(vm, ecx) {
        return Some((ecx, true));
    }
    None
}

pub(super) fn resolve_typeinfo_info(vm: &mut Vm, stack_ptr: u32) -> Option<(u32, u32, bool)> {
    let (this, thiscall) = resolve_typeinfo_this(vm, stack_ptr)?;
    let info_id = vm.read_u32(this.wrapping_add(4)).ok()?;
    if typelib::get_typeinfo(info_id).is_some() {
        return Some((this, info_id, thiscall));
    }
    None
}
