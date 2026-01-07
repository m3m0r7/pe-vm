//! Kernel32 stub registration.

pub const DLL_NAME: &str = "KERNEL32.dll";

mod exceptions;
mod error;
mod env;
mod console;
mod dynamic;
mod memory;
mod module;
mod file;
mod interlocked;
mod pointer;
mod process;
mod slist;
pub mod strings;
mod sync;
mod thread;
mod time;
mod tls;

use crate::vm::Vm;

pub(crate) use thread::create_thread;

pub fn register(vm: &mut Vm) {
    console::register(vm);
    dynamic::register(vm);
    error::register(vm);
    env::register(vm);
    file::register(vm);
    interlocked::register(vm);
    memory::register(vm);
    module::register(vm);
    pointer::register(vm);
    thread::register(vm);
    process::register(vm);
    slist::register(vm);
    strings::register(vm);
    sync::register(vm);
    time::register(vm);
    tls::register(vm);
    exceptions::register(vm);
}
