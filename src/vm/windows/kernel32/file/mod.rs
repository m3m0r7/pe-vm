//! Kernel32 file-system related stubs.

mod constants;
mod create;
mod find;
mod handle;
mod helpers;
mod mapping;
mod path;
mod pointer;
mod query;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    handle::register(vm);
    create::register(vm);
    find::register(vm);
    mapping::register(vm);
    query::register(vm);
    path::register(vm);
    pointer::register(vm);
}
