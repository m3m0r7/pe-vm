//! ADVAPI32 stubs.

pub const DLL_NAME: &str = "ADVAPI32.dll";

mod registry;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    registry::register(vm);
}
