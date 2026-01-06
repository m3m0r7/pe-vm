//! VM configuration and core types.

mod config;
mod core;
mod error;
mod host;
mod registers;
mod state;
mod types;

pub mod test_support;
pub mod windows;

pub use config::*;
pub use error::VmError;
pub use host::{host_create_thread, host_message_box_a, host_printf};
pub use state::{HostCall, Vm};
pub use types::{ComOutParam, ExecuteOptions, Value};

pub(crate) use registers::*;
pub(crate) use state::{FileHandle, Flags, HostFunction, OsState, PendingThread, Registers};
