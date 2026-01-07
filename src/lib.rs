//! Crate entry point and public re-exports.

mod api;
mod architecture;
pub mod ext;
mod pe;
pub mod settings;
mod vm;

pub use api::{Pe, SymbolExecutor};
pub use pe::{
    BoundForwarderRef, BoundImportDescriptor, BoundImportDirectory, ClrDirectory, DataDirectory,
    DebugDirectory, DebugDirectoryEntry, DelayImportDescriptor, DelayImportDirectory,
    DelayImportSymbol, DosHeader, ExportDirectory, ExportSymbol, FileHeader, IatDirectory,
    ImportDescriptor, ImportDirectory, ImportSymbol, LoadConfigDirectory32, OptionalHeader32,
    PeDirectories, PeFile, PeImage, PeParseError, RelocationBlock, RelocationDirectory,
    RelocationEntry, ResourceData, ResourceDirectory, ResourceId, ResourceNode, SectionHeader,
    SecurityDirectory, TlsDirectory,
};
pub use vm::windows;
pub use vm::{
    host_create_thread, host_message_box_a, host_printf, Architecture, ComOutParam, ExecuteOptions,
    HostCall, MessageBoxMode, Os, PathMapping, Renderer, SandboxConfig, Value, Vm, VmConfig,
    VmError,
};
