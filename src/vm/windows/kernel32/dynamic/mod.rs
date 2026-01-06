//! Kernel32 dynamic GetProcAddress stubs.

mod fileinfo;
mod locale;
mod package;
mod process;
mod sync;
mod threadpool;
mod time;
mod tls;
mod version;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    tls::register(vm);
    sync::register(vm);
    threadpool::register(vm);
    process::register(vm);
    locale::register(vm);
    package::register(vm);
    time::register(vm);
    version::register(vm);
    fileinfo::register(vm);
}
