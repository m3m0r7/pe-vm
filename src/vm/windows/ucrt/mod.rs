//! UCRT stub registration.

pub const DLL_NAME: &str = "ucrtbase.dll";

mod env;
mod init;
mod onexit;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    onexit::register(vm);
    env::register(vm);
    init::register(vm);
}
