//! In-memory IStream implementation for CreateStreamOnHGlobal.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::vm::{Vm, VmError};
use crate::vm_args;

use super::{E_INVALIDARG, E_NOTIMPL, S_OK};

const STREAM_SEEK_SET: u32 = 0;
const STREAM_SEEK_CUR: u32 = 1;
const STREAM_SEEK_END: u32 = 2;

const STGTY_STREAM: u32 = 2;

const STREAM_METHODS: [&str; 14] = [
    "pe_vm.stream.QueryInterface",
    "pe_vm.stream.AddRef",
    "pe_vm.stream.Release",
    "pe_vm.stream.Read",
    "pe_vm.stream.Write",
    "pe_vm.stream.Seek",
    "pe_vm.stream.SetSize",
    "pe_vm.stream.CopyTo",
    "pe_vm.stream.Commit",
    "pe_vm.stream.Revert",
    "pe_vm.stream.LockRegion",
    "pe_vm.stream.UnlockRegion",
    "pe_vm.stream.Stat",
    "pe_vm.stream.Clone",
];

#[derive(Debug, Clone)]
struct HGlobalStream {
    data: Vec<u8>,
    pos: usize,
    ref_count: u32,
    delete_on_release: bool,
    hglobal: u32,
    capacity: usize,
}

fn stream_store() -> &'static Mutex<HashMap<u32, HGlobalStream>> {
    static STORE: OnceLock<Mutex<HashMap<u32, HGlobalStream>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn stream_this(vm: &Vm, stack_ptr: u32) -> u32 {
    let ecx = vm.regs.ecx;
    if ecx != 0 {
        ecx
    } else {
        vm.read_u32(stack_ptr + 4).unwrap_or(0)
    }
}

pub(super) fn register_stream_imports(vm: &mut Vm) {
    vm.register_import_any_stdcall(STREAM_METHODS[0], 8, stream_query_interface);
    vm.register_import_any_stdcall(STREAM_METHODS[1], 0, stream_add_ref);
    vm.register_import_any_stdcall(STREAM_METHODS[2], 0, stream_release);
    vm.register_import_any_stdcall(STREAM_METHODS[3], 12, stream_read);
    vm.register_import_any_stdcall(STREAM_METHODS[4], 12, stream_write);
    vm.register_import_any_stdcall(STREAM_METHODS[5], 16, stream_seek);
    vm.register_import_any_stdcall(STREAM_METHODS[6], 8, stream_set_size);
    vm.register_import_any_stdcall(STREAM_METHODS[7], 20, stream_copy_to);
    vm.register_import_any_stdcall(STREAM_METHODS[8], 4, stream_commit);
    vm.register_import_any_stdcall(STREAM_METHODS[9], 0, stream_revert);
    vm.register_import_any_stdcall(STREAM_METHODS[10], 20, stream_lock_region);
    vm.register_import_any_stdcall(STREAM_METHODS[11], 20, stream_unlock_region);
    vm.register_import_any_stdcall(STREAM_METHODS[12], 8, stream_stat);
    vm.register_import_any_stdcall(STREAM_METHODS[13], 4, stream_clone);
}

pub(super) fn create_stream_on_hglobal(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hglobal, delete_on_release, out_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let vtable_ptr = match build_stream_vtable(vm) {
        Ok(ptr) => ptr,
        Err(_) => return E_INVALIDARG,
    };
    let obj_ptr = match vm.alloc_bytes(&vtable_ptr.to_le_bytes(), 4) {
        Ok(ptr) => ptr,
        Err(_) => return E_INVALIDARG,
    };
    let mut data = Vec::new();
    let mut capacity = 0usize;
    if hglobal != 0 {
        capacity = vm.heap_size(hglobal).unwrap_or(0);
        data.reserve(capacity);
        for idx in 0..capacity {
            data.push(vm.read_u8(hglobal.wrapping_add(idx as u32)).unwrap_or(0));
        }
    }
    let state = HGlobalStream {
        data,
        pos: 0,
        ref_count: 1,
        delete_on_release: delete_on_release != 0,
        hglobal,
        capacity,
    };
    stream_store()
        .lock()
        .expect("stream store")
        .insert(obj_ptr, state);
    let _ = vm.write_u32(out_ptr, obj_ptr);
    S_OK
}

fn build_stream_vtable(vm: &mut Vm) -> Result<u32, VmError> {
    let mut bytes = Vec::with_capacity(STREAM_METHODS.len() * 4);
    for name in STREAM_METHODS.iter() {
        let entry = vm
            .resolve_dynamic_import(name)
            .ok_or(VmError::InvalidConfig("missing stream import"))?;
        bytes.extend_from_slice(&entry.to_le_bytes());
    }
    vm.alloc_bytes(&bytes, 4)
}

fn stream_query_interface(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_iid_ptr, out_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let this = stream_this(vm, stack_ptr);
    if this == 0 {
        return E_INVALIDARG;
    }
    if let Some(state) = stream_store()
        .lock()
        .expect("stream store")
        .get_mut(&this)
    {
        state.ref_count = state.ref_count.saturating_add(1);
    }
    let _ = vm.write_u32(out_ptr, this);
    S_OK
}

fn stream_add_ref(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return 0;
    };
    state.ref_count = state.ref_count.saturating_add(1);
    state.ref_count
}

fn stream_release(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return 0;
    };
    if state.ref_count > 0 {
        state.ref_count -= 1;
    }
    let remaining = state.ref_count;
    if remaining == 0 {
        if state.delete_on_release && state.hglobal != 0 {
            let _ = vm.heap_free(state.hglobal);
        }
        store.remove(&this);
    }
    remaining
}

fn stream_read(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (buf_ptr, cb, pcb_read) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return E_INVALIDARG;
    };
    let available = state.data.len().saturating_sub(state.pos);
    let to_read = (cb as usize).min(available);
    if buf_ptr != 0 && to_read > 0 {
        let _ = vm.write_bytes(buf_ptr, &state.data[state.pos..state.pos + to_read]);
    }
    state.pos = state.pos.saturating_add(to_read);
    if pcb_read != 0 {
        let _ = vm.write_u32(pcb_read, to_read as u32);
    }
    S_OK
}

fn stream_write(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (buf_ptr, cb, pcb_written) = vm_args!(vm, stack_ptr; u32, u32, u32);
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return E_INVALIDARG;
    };
    let count = cb as usize;
    let mut bytes = Vec::with_capacity(count);
    for idx in 0..count {
        bytes.push(vm.read_u8(buf_ptr.wrapping_add(idx as u32)).unwrap_or(0));
    }
    let end = state.pos.saturating_add(count);
    if end > state.data.len() {
        state.data.resize(end, 0);
    }
    if !bytes.is_empty() {
        state.data[state.pos..end].copy_from_slice(&bytes);
    }
    state.pos = end;
    if state.hglobal != 0 {
        if end > state.capacity {
            let new_ptr = vm.heap_realloc(state.hglobal, end);
            if new_ptr != 0 {
                state.hglobal = new_ptr;
                state.capacity = end;
            }
        }
        if state.capacity >= end {
            let _ = vm.write_bytes(state.hglobal, &state.data);
        } else if state.capacity > 0 {
            let _ = vm.write_bytes(state.hglobal, &state.data[..state.capacity]);
        }
    }
    if pcb_written != 0 {
        let _ = vm.write_u32(pcb_written, count as u32);
    }
    S_OK
}

fn stream_seek(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let low = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let high = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let origin = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let new_pos_ptr = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return E_INVALIDARG;
    };
    let move_val = ((high as i64) << 32) | (low as i64 & 0xFFFF_FFFF);
    let base = match origin {
        STREAM_SEEK_SET => 0i64,
        STREAM_SEEK_CUR => state.pos as i64,
        STREAM_SEEK_END => state.data.len() as i64,
        _ => state.pos as i64,
    };
    let mut new_pos = base.saturating_add(move_val);
    if new_pos < 0 {
        new_pos = 0;
    }
    state.pos = new_pos as usize;
    if new_pos_ptr != 0 {
        let _ = vm.write_u32(new_pos_ptr, new_pos as u32);
        let _ = vm.write_u32(new_pos_ptr + 4, (new_pos >> 32) as u32);
    }
    S_OK
}

fn stream_set_size(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let low = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let high = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let this = stream_this(vm, stack_ptr);
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get_mut(&this) else {
        return E_INVALIDARG;
    };
    let new_size = ((high as u64) << 32) | (low as u64);
    state.data.resize(new_size as usize, 0);
    if state.pos > state.data.len() {
        state.pos = state.data.len();
    }
    if state.hglobal != 0 && state.data.len() > state.capacity {
        let new_ptr = vm.heap_realloc(state.hglobal, state.data.len());
        if new_ptr != 0 {
            state.hglobal = new_ptr;
            state.capacity = state.data.len();
        }
    }
    if state.hglobal != 0 {
        if state.capacity >= state.data.len() {
            let _ = vm.write_bytes(state.hglobal, &state.data);
        } else if state.capacity > 0 {
            let _ = vm.write_bytes(state.hglobal, &state.data[..state.capacity]);
        }
    }
    S_OK
}

fn stream_copy_to(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn stream_commit(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    S_OK
}

fn stream_revert(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn stream_lock_region(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn stream_unlock_region(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    E_NOTIMPL
}

fn stream_stat(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (stat_ptr, _flags) = vm_args!(vm, stack_ptr; u32, u32);
    let this = stream_this(vm, stack_ptr);
    if stat_ptr == 0 {
        return E_INVALIDARG;
    }
    let store = stream_store().lock().expect("stream store");
    let Some(state) = store.get(&this) else {
        return E_INVALIDARG;
    };
    let _ = vm.write_u32(stat_ptr, 0);
    let _ = vm.write_u32(stat_ptr + 4, STGTY_STREAM);
    let _ = vm.write_u32(stat_ptr + 8, state.data.len() as u32);
    let _ = vm.write_u32(stat_ptr + 12, (state.data.len() as u64 >> 32) as u32);
    for offset in (16..72).step_by(4) {
        let _ = vm.write_u32(stat_ptr + offset, 0);
    }
    S_OK
}

fn stream_clone(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (out_ptr,) = vm_args!(vm, stack_ptr; u32);
    let this = stream_this(vm, stack_ptr);
    if out_ptr == 0 {
        return E_INVALIDARG;
    }
    let mut store = stream_store().lock().expect("stream store");
    let Some(state) = store.get(&this) else {
        return E_INVALIDARG;
    };
    let vtable_ptr = match build_stream_vtable(vm) {
        Ok(ptr) => ptr,
        Err(_) => return E_INVALIDARG,
    };
    let obj_ptr = match vm.alloc_bytes(&vtable_ptr.to_le_bytes(), 4) {
        Ok(ptr) => ptr,
        Err(_) => return E_INVALIDARG,
    };
    let clone = HGlobalStream {
        data: state.data.clone(),
        pos: state.pos,
        ref_count: 1,
        delete_on_release: state.delete_on_release,
        hglobal: state.hglobal,
        capacity: state.capacity,
    };
    store.insert(obj_ptr, clone);
    let _ = vm.write_u32(out_ptr, obj_ptr);
    S_OK
}
