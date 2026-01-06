//! User32 windowing and input stubs.

mod constants;
mod dialog;
mod helpers;
mod input;
mod message;
mod monitor;
mod paint;
mod rect;
mod window_ops;

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    window_ops::register(vm);
    dialog::register(vm);
    message::register(vm);
    input::register(vm);
    monitor::register(vm);
    rect::register(vm);
    paint::register(vm);
}
