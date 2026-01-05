//! VCRUNTIME stub registration.

mod memory;
mod runtime;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    runtime::register(vm);
    memory::register(vm);
}
