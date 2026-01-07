//! ADVAPI32 registry stubs.

mod api;
mod constants;
mod helpers;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    api::register(vm);
}
