//! User32 stub registration.

mod message_box;

use crate::vm::Vm;

pub(crate) use message_box::message_box_a;

pub fn register(vm: &mut Vm) {
    message_box::register(vm);
}
