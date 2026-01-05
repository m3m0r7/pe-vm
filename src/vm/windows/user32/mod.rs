//! User32 stub registration.

mod message_box;
mod cursor;
mod class;
mod window;
mod misc;

use crate::vm::Vm;

pub(crate) use message_box::message_box_a;

pub fn register(vm: &mut Vm) {
    class::register(vm);
    cursor::register(vm);
    message_box::register(vm);
    window::register(vm);
    misc::register(vm);
}
