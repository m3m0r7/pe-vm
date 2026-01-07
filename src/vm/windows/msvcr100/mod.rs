//! MSVCR100.dll stub registration.
//!
//! This module provides stub implementations for the Microsoft Visual C++ 2010 Runtime Library.
//! Functions are organized into categories for maintainability.

pub const DLL_NAME: &str = "MSVCR100.dll";

mod concurrency;
mod crt;
mod exception;
mod io;
mod locale;
mod math;
mod memory;
mod process;
mod stdio;
mod stdlib;
mod string;
mod time;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    concurrency::register(vm);
    crt::register(vm);
    exception::register(vm);
    io::register(vm);
    locale::register(vm);
    math::register(vm);
    memory::register(vm);
    process::register(vm);
    stdio::register(vm);
    stdlib::register(vm);
    string::register(vm);
    time::register(vm);
}
