use crate::vm::{HostFunction, Vm, VmError};
use crate::vm::state::AtlStringMgr;

type AtlMethod = (&'static str, u32, fn(&mut Vm, u32) -> u32);

const ATL_STRING_METHODS: &[AtlMethod] = &[
    ("pe_vm.atl.Allocate", 2, atl_allocate),
    ("pe_vm.atl.Free", 1, atl_free),
    ("pe_vm.atl.Reallocate", 3, atl_reallocate),
    ("pe_vm.atl.GetNilString", 0, atl_get_nil),
    ("pe_vm.atl.Clone", 1, atl_clone),
];

#[derive(Clone, Copy, Debug)]
enum AtlStringMethod {
    Allocate,
    Free,
    Reallocate,
    GetNil,
    Clone,
}

impl AtlStringMethod {
    fn from_mem_addr(mem_addr: u32) -> Option<Self> {
        match mem_addr {
            0 => Some(Self::Allocate),
            4 => Some(Self::Free),
            8 => Some(Self::Reallocate),
            12 => Some(Self::GetNil),
            16 => Some(Self::Clone),
            _ => None,
        }
    }

    fn needs_pdata(self) -> bool {
        matches!(self, Self::Free | Self::Reallocate | Self::Clone)
    }
}

impl Vm {
    pub(crate) fn ensure_atl_string_mgr(&mut self) -> Result<AtlStringMgr, VmError> {
        if let Some(mgr) = self.atl_string_mgr {
            return Ok(mgr);
        }
        register_atl_string_mgr_thunks(self);
        let vtable = build_vtable(self)?;
        let object = build_object(self, vtable)?;
        let nil_data = build_nil_string(self, object)?;
        let mgr = AtlStringMgr {
            vtable,
            object,
            nil_data,
        };
        self.atl_string_mgr = Some(mgr);
        Ok(mgr)
    }

    pub(crate) fn try_handle_atl_string_mgr_call(
        &mut self,
        mem_addr: u32,
        next: u32,
    ) -> Result<bool, VmError> {
        let Some(method) = AtlStringMethod::from_mem_addr(mem_addr) else {
            return Ok(false);
        };
        let mgr = self.ensure_atl_string_mgr()?;
        if method.needs_pdata() {
            let p_data = self.read_u32(self.regs.esp).unwrap_or(0);
            if p_data != 0 && self.contains_addr(p_data) {
                let _ = self.write_u32(p_data, mgr.object);
            }
        }
        self.regs.ecx = mgr.object;
        let host = atl_method_host(method);
        self.call_host(host, next)?;
        Ok(true)
    }
}

fn register_atl_string_mgr_thunks(vm: &mut Vm) {
    for &(name, args, func) in ATL_STRING_METHODS {
        vm.register_import_any_stdcall(name, crate::vm::stdcall_args(args), func);
    }
}

fn build_vtable(vm: &mut Vm) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(ATL_STRING_METHODS.len() * 4);
    for &(name, _, _) in ATL_STRING_METHODS {
        let entry = vm
            .resolve_dynamic_import(name)
            .ok_or(VmError::InvalidConfig("missing import"))?;
        bytes.extend_from_slice(&entry.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn build_object(vm: &mut Vm, vtable_ptr: u32) -> Result<u32, VmError> {
    vm.alloc_bytes(&vtable_ptr.to_le_bytes(), 4)
}

fn build_nil_string(vm: &mut Vm, mgr_ptr: u32) -> Result<u32, VmError> {
    let bytes = build_cstring_bytes(mgr_ptr, 0, 0, 2, -1, &[])
        .ok_or(VmError::OutOfMemory)?;
    vm.alloc_bytes(&bytes, 4)
}

fn atl_method_host(method: AtlStringMethod) -> HostFunction {
    match method {
        AtlStringMethod::Allocate => HostFunction {
            func: atl_allocate,
            stack_cleanup: crate::vm::stdcall_args(2),
        },
        AtlStringMethod::Free => HostFunction {
            func: atl_free,
            stack_cleanup: crate::vm::stdcall_args(1),
        },
        AtlStringMethod::Reallocate => HostFunction {
            func: atl_reallocate,
            stack_cleanup: crate::vm::stdcall_args(3),
        },
        AtlStringMethod::GetNil => HostFunction {
            func: atl_get_nil,
            stack_cleanup: crate::vm::stdcall_args(0),
        },
        AtlStringMethod::Clone => HostFunction {
            func: atl_clone,
            stack_cleanup: crate::vm::stdcall_args(1),
        },
    }
}

fn atl_allocate(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (n_chars, n_bytes) = crate::vm_args!(vm, stack_ptr; u32, u32);
    if std::env::var("PE_VM_TRACE_ATL").is_ok() {
        eprintln!("[pe_vm] ATL Allocate n_chars={n_chars} n_bytes={n_bytes}");
    }
    let mgr = match vm.ensure_atl_string_mgr() {
        Ok(mgr) => mgr,
        Err(_) => return 0,
    };
    let data_len = 0u32;
    let Some(bytes) = build_cstring_bytes(mgr.object, data_len, n_chars, n_bytes, 1, &[]) else {
        return 0;
    };
    vm.alloc_bytes(&bytes, 4).unwrap_or(0)
}

fn atl_free(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn atl_reallocate(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (p_data, n_chars, n_bytes) = crate::vm_args!(vm, stack_ptr; u32, u32, u32);
    let mgr = match vm.ensure_atl_string_mgr() {
        Ok(mgr) => mgr,
        Err(_) => return 0,
    };
    let char_size = n_bytes.max(1);
    let mut old_len = 0u32;
    let mut old_alloc = 0u32;
    let mut data = Vec::new();
    if p_data != 0 && vm.contains_addr(p_data) {
        old_len = vm.read_u32(p_data.wrapping_add(4)).unwrap_or(0);
        old_alloc = vm.read_u32(p_data.wrapping_add(8)).unwrap_or(0);
        let copy_chars = old_len.min(n_chars);
        if let Some(byte_len) = (copy_chars as usize).checked_mul(char_size as usize) {
            if byte_len > 0 {
                data = read_vm_bytes(vm, p_data.wrapping_add(16), byte_len);
            }
        } else {
            return 0;
        }
    }
    let data_len = old_len.min(n_chars);
    if std::env::var("PE_VM_TRACE_ATL").is_ok() {
        eprintln!(
            "[pe_vm] ATL Reallocate p_data=0x{p_data:08X} n_chars={n_chars} n_bytes={n_bytes} old_len={old_len} old_alloc={old_alloc}"
        );
    }
    let Some(bytes) = build_cstring_bytes(mgr.object, data_len, n_chars, char_size, 1, &data)
    else {
        return 0;
    };
    vm.alloc_bytes(&bytes, 4).unwrap_or(0)
}

fn atl_get_nil(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    match vm.ensure_atl_string_mgr() {
        Ok(mgr) => mgr.nil_data,
        Err(_) => 0,
    }
}

fn atl_clone(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (p_data,) = crate::vm_args!(vm, stack_ptr; u32);
    let mgr = match vm.ensure_atl_string_mgr() {
        Ok(mgr) => mgr,
        Err(_) => return 0,
    };
    if p_data == 0 {
        return mgr.nil_data;
    }
    if vm.contains_addr(p_data) {
        let refs = vm.read_u32(p_data.wrapping_add(12)).unwrap_or(0) as i32;
        if refs >= 0 {
            let updated = refs.saturating_add(1) as u32;
            let _ = vm.write_u32(p_data.wrapping_add(12), updated);
        }
    }
    p_data
}

fn build_cstring_bytes(
    mgr_ptr: u32,
    data_len: u32,
    alloc_len: u32,
    char_size: u32,
    refs: i32,
    data: &[u8],
) -> Option<Vec<u8>> {
    let char_size = char_size.max(1) as usize;
    let alloc_len = alloc_len as usize;
    let data_len = data_len.min(alloc_len as u32) as usize;
    let data_bytes = data_len.checked_mul(char_size)?;
    let total_data_bytes = alloc_len.checked_add(1)?.checked_mul(char_size)?;
    let total_size = 16usize.checked_add(total_data_bytes)?;
    let mut bytes = vec![0u8; total_size];
    bytes[0..4].copy_from_slice(&mgr_ptr.to_le_bytes());
    bytes[4..8].copy_from_slice(&(data_len as u32).to_le_bytes());
    bytes[8..12].copy_from_slice(&(alloc_len as u32).to_le_bytes());
    bytes[12..16].copy_from_slice(&refs.to_le_bytes());
    let copy_len = data_bytes.min(data.len());
    if copy_len > 0 {
        bytes[16..16 + copy_len].copy_from_slice(&data[..copy_len]);
    }
    Some(bytes)
}

fn read_vm_bytes(vm: &Vm, ptr: u32, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(vm.read_u8(ptr.wrapping_add(i as u32)).unwrap_or(0));
    }
    out
}
