//! Kernel32 time-related stubs.

mod filetime;
mod format;
mod helpers;
mod perf;
mod timezone;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    filetime::register(vm);
    format::register(vm);
    timezone::register(vm);
    perf::register(vm);
}
