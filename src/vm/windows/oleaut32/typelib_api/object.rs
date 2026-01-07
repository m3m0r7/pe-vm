use crate::vm::{Vm, VmError};

use super::super::constants::OleMethod;
use super::super::typeinfo::TYPEINFO_METHODS;
use super::super::typelib;
use super::TYPELIB_METHODS;

pub(super) fn alloc_typelib(vm: &mut Vm, lib: typelib::TypeLib) -> Result<u32, VmError> {
    register_typelib_thunks(vm);
    register_typeinfo_thunks(vm);
    let lib_id = typelib::store_typelib(lib);
    let vtable = build_vtable(vm, TYPELIB_METHODS)?;
    build_object(vm, vtable, &[lib_id])
}

pub(super) fn build_typeinfo_object(vm: &mut Vm, typeinfo_id: u32) -> Result<u32, VmError> {
    let vtable = build_vtable(vm, TYPEINFO_METHODS)?;
    build_object(vm, vtable, &[typeinfo_id])
}

fn build_vtable(vm: &mut Vm, methods: &[OleMethod]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(methods.len() * 4);
    for &(name, _, _) in methods {
        let entry = vm
            .resolve_dynamic_import(name)
            .ok_or(VmError::InvalidConfig("missing import"))?;
        bytes.extend_from_slice(&entry.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn build_object(vm: &mut Vm, vtable_ptr: u32, extras: &[u32]) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity((1 + extras.len()) * 4);
    bytes.extend_from_slice(&vtable_ptr.to_le_bytes());
    for extra in extras {
        bytes.extend_from_slice(&extra.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn register_typelib_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPELIB_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn register_typeinfo_thunks(vm: &mut Vm) {
    for &(name, args, func) in TYPEINFO_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}
