//! SHLWAPI stubs.

mod path;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    path::register(vm);
}
