//! COM/IDispatch scaffolding and dispatch table support.

mod args;
mod dispatch;
mod object;
mod runtime;

pub use args::{ComArg, ComValue};
pub use dispatch::{DispatchHandle, DispatchTable};
pub use object::ComObject;
pub use runtime::Com;
