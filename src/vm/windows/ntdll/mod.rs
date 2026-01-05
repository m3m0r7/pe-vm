//! NTDLL stub registration.

mod peb;
mod slist;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    slist::register(vm);
    peb::register(vm);
}
