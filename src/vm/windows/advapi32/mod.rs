//! ADVAPI32 stubs.

mod registry;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    registry::register(vm);
}
