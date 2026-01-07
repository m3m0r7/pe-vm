//! NTDLL stub registration.

pub const DLL_NAME: &str = "ntdll.dll";

mod peb;
mod slist;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    slist::register(vm);
    peb::register(vm);
}
