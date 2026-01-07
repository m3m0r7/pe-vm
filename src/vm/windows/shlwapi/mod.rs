//! SHLWAPI stubs.

pub const DLL_NAME: &str = "SHLWAPI.dll";

mod path;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    path::register(vm);
}
