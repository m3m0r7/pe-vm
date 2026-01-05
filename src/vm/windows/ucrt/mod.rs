//! UCRT stub registration.

mod env;
mod init;
mod onexit;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    onexit::register(vm);
    env::register(vm);
    init::register(vm);
}
