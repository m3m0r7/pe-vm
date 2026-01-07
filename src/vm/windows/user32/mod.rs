//! User32 stub registration.

pub const DLL_NAME: &str = "USER32.dll";

mod class;
mod cursor;
mod message_box;
mod misc;
mod window;

use crate::vm::Vm;

pub(crate) use message_box::message_box_a;

pub fn register(vm: &mut Vm) {
    class::register(vm);
    cursor::register(vm);
    message_box::register(vm);
    window::register(vm);
    misc::register(vm);
}
