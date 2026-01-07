//! PE header and directory data structures.

mod clr;
mod debug;
mod headers;
mod import;
mod load_config;
mod reloc;
mod resource;
mod security;
mod tls;

pub use clr::*;
pub use debug::*;
pub use headers::*;
pub use import::*;
pub use load_config::*;
pub use reloc::*;
pub use resource::*;
pub use security::*;
pub use tls::*;

#[derive(Debug, Clone, Default)]
pub struct PeDirectories {
    pub export: Option<ExportDirectory>,
    pub import: Option<ImportDirectory>,
    pub resource: Option<ResourceDirectory>,
    pub exception: Option<Vec<u8>>,
    pub security: Option<SecurityDirectory>,
    pub reloc: Option<RelocationDirectory>,
    pub debug: Option<DebugDirectory>,
    pub architecture: Option<Vec<u8>>,
    pub global_ptr: Option<u32>,
    pub tls: Option<TlsDirectory>,
    pub load_config: Option<LoadConfigDirectory32>,
    pub bound_import: Option<BoundImportDirectory>,
    pub iat: Option<IatDirectory>,
    pub delay_import: Option<DelayImportDirectory>,
    pub clr: Option<ClrDirectory>,
}
