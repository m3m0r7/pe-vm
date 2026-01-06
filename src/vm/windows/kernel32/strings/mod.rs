//! Kernel32 ANSI/Unicode string helpers.

mod ansi;
mod classify;
mod codepage;
mod convert;
mod helpers;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    codepage::register(vm);
    classify::register(vm);
    ansi::register(vm);
    convert::register(vm);
}
