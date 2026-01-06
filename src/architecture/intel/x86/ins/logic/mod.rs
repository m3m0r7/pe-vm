//! x86 logic and test instruction handlers.

mod and;
mod not;
mod or;
mod test;
mod xor;

pub(crate) use and::*;
pub(crate) use not::*;
pub(crate) use or::*;
pub(crate) use test::*;
pub(crate) use xor::*;
