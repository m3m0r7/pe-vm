use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use crate::architecture::intel::x86::X86Executor;
use crate::pe::{PeFile, ResourceDirectory};

use super::{windows, ComOutParam, MessageBoxMode, VmConfig, VmError};

// OS-specific state stored in the VM without exposing platform details.
pub(crate) enum OsState {
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
pub(crate) struct Flags {
    pub(crate) cf: bool,
    pub(crate) zf: bool,
    pub(crate) sf: bool,
    pub(crate) of: bool,
    pub(crate) df: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct FpuState {
    pub(crate) stack: [f64; 8],
    pub(crate) valid: [bool; 8],
    pub(crate) top: u8,
    pub(crate) control_word: u16,
    #[allow(dead_code)]
    pub(crate) status_word: u16,
    #[allow(dead_code)]
    pub(crate) tag_word: u16,
}

impl Default for FpuState {
    fn default() -> Self {
        Self {
            stack: [0.0; 8],
            valid: [false; 8],
            top: 0,
            control_word: 0x037F,
            status_word: 0,
            tag_word: 0xFFFF,
        }
    }
}

impl FpuState {
    fn st_index(&self, index: usize) -> usize {
        (self.top as usize + index) & 7
    }

    fn top_index(&self) -> usize {
        self.top as usize
    }
}

#[derive(Clone, Copy)]
pub(crate) struct HostFunction {
    pub(crate) func: HostCall,
    pub(crate) stack_cleanup: u32,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub(crate) struct AtlStringMgr {
    pub(crate) vtable: u32,
    pub(crate) object: u32,
    pub(crate) nil_data: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct PendingThread {
    pub(crate) entry: u32,
    pub(crate) param: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadedModule {
    pub(crate) name: String,
    pub(crate) guest_path: String,
    pub(crate) host_path: String,
    pub(crate) base: u32,
    #[allow(dead_code)]
    pub(crate) size: u32,
    pub(crate) pe: PeFile,
}

pub struct Vm {
    pub(super) config: VmConfig,
    pub(super) os_state: OsState,
    pub(crate) base: u32,
    pub(crate) memory: Vec<u8>,
    pub(crate) regs: Registers,
    // Minimal SSE state for XMM register operations.
    pub(super) xmm: [[u8; 16]; 8],
    pub(super) flags: Flags,
    pub(crate) stack_top: u32,
    pub(super) stack_depth: u32,
    pub(super) heap_start: usize,
    pub(super) heap_end: usize,
    pub(super) heap_cursor: usize,
    pub(super) heap_allocs: HashMap<u32, usize>,
    pub(super) fs_base: u32,
    pub(super) gs_base: u32,
    pub(super) env: BTreeMap<String, String>,
    pub(super) string_overlays: HashMap<u32, String>,
    pub(super) image_path: Option<String>,
    pub(super) resource_dir: Option<ResourceDirectory>,
    pub(super) resource_sizes: HashMap<u32, u32>,
    pub(super) dispatch_instance: Option<u32>,
    pub(super) last_com_out_params: Vec<ComOutParam>,
    pub(super) last_error: u32,
    pub(super) main_module: Option<u32>,
    pub(super) modules: Vec<LoadedModule>,
    pub(super) registry_handles: HashMap<u32, String>,
    pub(super) registry_next_handle: u32,
    pub(super) file_handles: HashMap<u32, FileHandle>,
    pub(super) file_next_handle: u32,
    pub(super) virtual_files: HashMap<String, Vec<u8>>,
    pub(super) tls_values: HashMap<u32, u32>,
    pub(super) tls_next_index: u32,
    pub(super) unhandled_exception_filter: u32,
    pub(super) message_box_mode: MessageBoxMode,
    pub(super) onexit_tables: BTreeMap<u32, Vec<u32>>,
    pub(super) default_onexit_table: u32,
    pub(super) imports_by_name: HashMap<String, HostFunction>,
    pub(super) imports_by_any: HashMap<String, HostFunction>,
    pub(super) imports_by_ordinal: HashMap<String, HostFunction>,
    pub(super) imports_by_iat: HashMap<u32, HostFunction>,
    pub(super) imports_by_iat_name: HashMap<u32, String>,
    pub(super) dynamic_imports: HashMap<String, u32>,
    pub(super) dynamic_import_next: u32,
    pub(super) atl_string_mgr: Option<AtlStringMgr>,
    pub(super) pending_threads: Vec<PendingThread>,
    pub(super) next_thread_handle: u32,
    pub(super) stdout: Arc<Mutex<Vec<u8>>>,
    pub(super) executor: X86Executor,
    pub(super) fpu: FpuState,
    // File mapping support
    pub(super) file_mappings: HashMap<u32, FileMapping>,
    pub(super) file_mapping_next_handle: u32,
    pub(super) mapped_views: HashMap<u32, MappedView>,
}

#[derive(Debug, Clone)]
pub(crate) struct FileHandle {
    pub(crate) path: String,
    pub(crate) cursor: usize,
    pub(crate) readable: bool,
    pub(crate) writable: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct FileMapping {
    pub(crate) file_handle: u32,
    pub(crate) size: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct MappedView {
    pub(crate) mapping_handle: u32,
    pub(crate) base_addr: u32,
    pub(crate) size: usize,
}

impl Vm {
    pub(crate) fn fpu_push(&mut self, value: f64) -> Result<(), VmError> {
        let new_top = (self.fpu.top.wrapping_sub(1)) & 7;
        let idx = new_top as usize;
        if self.fpu.valid[idx] {
            return Err(VmError::FpuStackOverflow);
        }
        self.fpu.top = new_top;
        self.fpu.stack[idx] = value;
        self.fpu.valid[idx] = true;
        Ok(())
    }

    pub(crate) fn fpu_pop(&mut self) -> Result<f64, VmError> {
        let idx = self.fpu.top_index();
        if !self.fpu.valid[idx] {
            return Err(VmError::FpuStackUnderflow);
        }
        let value = self.fpu.stack[idx];
        self.fpu.valid[idx] = false;
        self.fpu.top = (self.fpu.top + 1) & 7;
        Ok(value)
    }

    pub fn fpu_st(&self, index: usize) -> Result<f64, VmError> {
        if index >= 8 {
            return Err(VmError::UnsupportedInstruction(0));
        }
        let idx = self.fpu.st_index(index);
        if !self.fpu.valid[idx] {
            return Err(VmError::FpuStackUnderflow);
        }
        Ok(self.fpu.stack[idx])
    }

    pub(crate) fn fpu_set_st(&mut self, index: usize, value: f64) -> Result<(), VmError> {
        if index >= 8 {
            return Err(VmError::UnsupportedInstruction(0));
        }
        let idx = self.fpu.st_index(index);
        if !self.fpu.valid[idx] {
            return Err(VmError::FpuStackUnderflow);
        }
        self.fpu.stack[idx] = value;
        Ok(())
    }

    pub(crate) fn fpu_control(&self) -> u16 {
        self.fpu.control_word
    }

    pub(crate) fn fpu_set_control(&mut self, value: u16) {
        self.fpu.control_word = value;
    }

    #[allow(dead_code)]
    pub(crate) fn fpu_status(&self) -> u16 {
        self.fpu.status_word
    }

    #[allow(dead_code)]
    pub(crate) fn fpu_set_status(&mut self, value: u16) {
        self.fpu.status_word = value;
    }

    pub(crate) fn fpu_reset(&mut self) {
        self.fpu = FpuState::default();
    }
}
