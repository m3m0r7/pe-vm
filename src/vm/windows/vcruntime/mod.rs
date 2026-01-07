//! VCRUNTIME stub registration.

pub const DLL_NAME: &str = "VCRUNTIME140.dll";

mod memory;
mod runtime;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    runtime::register(vm);
    memory::register(vm);
}
