//! VM configuration and core types.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::architecture::intel::x86::X86Executor;
use crate::pe::PeParseError;

mod core;
mod host;
mod registers;
pub mod test_support;
pub mod windows;

pub use host::{host_create_thread, host_message_box_a, host_printf};
pub(crate) use registers::*;

#[derive(Debug, Clone, Copy)]
pub enum Os {
    Windows,
    Unix,
    Mac,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
}

pub type PathMapping = BTreeMap<String, String>;

pub(crate) const fn stdcall_args(args: u32) -> u32 {
    args * 4
}

#[derive(Debug, Clone)]
pub struct VmConfig {
    os: Os,
    architecture: Architecture,
    properties: Option<windows::registry::Registry>,
    paths: PathMapping,
    font_path: Option<String>,
    execution_limit: u64,
}

impl VmConfig {
    pub fn new() -> Self {
        Self {
            os: Os::Windows,
            architecture: Architecture::X86,
            properties: None,
            paths: PathMapping::new(),
            font_path: None,
            execution_limit: 1_000_000,
        }
    }

    pub fn os(self, os: Os) -> Self {
        let mut config = self;
        config.os = os;
        config
    }

    pub fn architecture(self, architecture: Architecture) -> Self {
        let mut config = self;
        config.architecture = architecture;
        config
    }

    pub fn properties(self, properties: windows::registry::Registry) -> Self {
        let mut config = self;
        config.properties = Some(properties);
        config
    }

    pub fn paths(self, paths: PathMapping) -> Self {
        let mut config = self;
        config.paths = paths;
        config
    }

    pub fn font_path(self, path: impl Into<String>) -> Self {
        let mut config = self;
        config.font_path = Some(path.into());
        config
    }

    pub fn font_path_opt(&self) -> Option<&str> {
        self.font_path.as_deref()
    }

    pub fn execution_limit(self, limit: u64) -> Self {
        let mut config = self;
        config.execution_limit = limit;
        config
    }

    pub fn execution_limit_value(&self) -> u64 {
        self.execution_limit
    }
}

impl Default for VmConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Env(BTreeMap<String, String>),
    U32(u32),
    U64(u64),
    String(String),
}

// Captures COM out parameters for the most recent IDispatch/ITypeInfo call.
#[derive(Debug, Clone)]
pub struct ComOutParam {
    pub index: usize,
    pub vt: u16,
    pub flags: u32,
    pub ptr: u32,
}

#[derive(Debug, Default, Clone)]
pub struct ExecuteOptions {
    env: Option<BTreeMap<String, String>>,
}

impl ExecuteOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn env(self, env: BTreeMap<String, String>) -> Self {
        let mut options = self;
        options.env = Some(env);
        options
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageBoxMode {
    Stdout,
    #[default]
    Dialog,
    Silent,
}

// OS-specific state stored in the VM without exposing platform details.
enum OsState {
    Windows(windows::WindowsState),
    Unix,
    Mac,
}

pub type HostCall = fn(&mut Vm, u32) -> u32;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Registers {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub eip: u32,
}

#[derive(Debug, Default, Clone, Copy)]
struct Flags {
    cf: bool,
    zf: bool,
    sf: bool,
    of: bool,
}

#[derive(Clone, Copy)]
struct HostFunction {
    func: HostCall,
    stack_cleanup: u32,
}

#[derive(Debug)]
pub enum VmError {
    Io(std::io::Error),
    Pe(PeParseError),
    MemoryOutOfRange,
    OutOfMemory,
    DivideError,
    UnsupportedInstruction(u8),
    ExecutionLimit,
    MissingExport(String),
    NoImage,
    InvalidConfig(&'static str),
    Com(u32),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Io(err) => write!(f, "io error: {err}"),
            VmError::Pe(err) => write!(f, "pe error: {err}"),
            VmError::MemoryOutOfRange => write!(f, "memory out of range"),
            VmError::OutOfMemory => write!(f, "out of memory"),
            VmError::DivideError => write!(f, "divide error"),
            VmError::UnsupportedInstruction(op) => write!(f, "unsupported instruction 0x{op:02X}"),
            VmError::ExecutionLimit => write!(f, "execution limit reached"),
            VmError::MissingExport(name) => write!(f, "missing export: {name}"),
            VmError::NoImage => write!(f, "no image loaded"),
            VmError::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            VmError::Com(code) => write!(f, "com error: 0x{code:08X}"),
        }
    }
}

impl std::error::Error for VmError {}

impl From<std::io::Error> for VmError {
    fn from(err: std::io::Error) -> Self {
        VmError::Io(err)
    }
}

impl From<PeParseError> for VmError {
    fn from(err: PeParseError) -> Self {
        VmError::Pe(err)
    }
}

pub struct Vm {
    config: VmConfig,
    os_state: OsState,
    base: u32,
    memory: Vec<u8>,
    regs: Registers,
    // Minimal SSE state for XMM register operations.
    xmm: [[u8; 16]; 8],
    flags: Flags,
    stack_top: u32,
    stack_depth: u32,
    heap_start: usize,
    heap_end: usize,
    heap_cursor: usize,
    heap_allocs: HashMap<u32, usize>,
    fs_base: u32,
    gs_base: u32,
    env: BTreeMap<String, String>,
    image_path: Option<String>,
    dispatch_instance: Option<u32>,
    last_com_out_params: Vec<ComOutParam>,
    last_error: u32,
    registry_handles: HashMap<u32, String>,
    registry_next_handle: u32,
    file_handles: HashMap<u32, FileHandle>,
    file_next_handle: u32,
    virtual_files: HashMap<String, Vec<u8>>,
    tls_values: HashMap<u32, u32>,
    tls_next_index: u32,
    unhandled_exception_filter: u32,
    message_box_mode: MessageBoxMode,
    onexit_tables: BTreeMap<u32, Vec<u32>>,
    default_onexit_table: u32,
    imports_by_name: HashMap<String, HostFunction>,
    imports_by_any: HashMap<String, HostFunction>,
    imports_by_ordinal: HashMap<String, HostFunction>,
    imports_by_iat: HashMap<u32, HostFunction>,
    imports_by_iat_name: HashMap<u32, String>,
    dynamic_imports: HashMap<String, u32>,
    dynamic_import_next: u32,
    stdout: Arc<Mutex<Vec<u8>>>,
    executor: X86Executor,
}

#[derive(Debug, Clone)]
struct FileHandle {
    path: String,
    cursor: usize,
    readable: bool,
    writable: bool,
}
