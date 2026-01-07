//! x86 sub/sbb/cmp/neg instruction handlers.

mod cmp;
mod dec;
mod neg;
mod sbb;
mod sub_ops;

pub(crate) use cmp::*;
pub(crate) use dec::*;
pub(crate) use neg::*;
pub(crate) use sbb::*;
pub(crate) use sub_ops::*;
