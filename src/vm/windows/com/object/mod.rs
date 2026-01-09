//! COM object wrapper with dispatch helpers.

mod variant;
mod vtable;

use std::sync::Arc;

use crate::vm::windows::oleaut32;
use crate::vm::windows::oleaut32::typelib::{FuncDesc, TypeLib};
use crate::vm::{Value, Vm, VmError};

use super::{ComArg, ComValue, DispatchTable};

use vtable::detect_thiscall;
pub(crate) use vtable::vtable_fn;

const DISPATCH_METHOD: u16 = 0x1;
const DISPATCH_PROPERTYGET: u16 = 0x2;
const DISPATCH_PROPERTYPUT: u16 = 0x4;
const DISPATCH_PROPERTYPUTREF: u16 = 0x8;
const DISPID_PROPERTYPUT: i32 = -3;
pub(super) const VT_EMPTY: u16 = 0;
pub(super) const VT_NULL: u16 = 1;
pub(super) const VT_I4: u16 = 3;
pub(super) const VT_I1: u16 = 16;
pub(super) const VT_BSTR: u16 = 8;
pub(super) const VT_ERROR: u16 = 10;
pub(super) const VT_UI4: u16 = 19;
pub(super) const VT_INT: u16 = 22;
pub(super) const VT_UINT: u16 = 23;
pub(super) const VT_VARIANT: u16 = 12;
pub(super) const VT_BYREF: u16 = 0x4000;
pub(super) const VT_USERDEFINED: u16 = 0x1D;
pub(super) const DISP_E_TYPEMISMATCH: u32 = 0x80020005;
pub(super) const DISP_E_PARAMNOTFOUND: u32 = 0x80020004;
pub(super) const PARAMFLAG_FIN: u32 = 0x1;
pub(super) const PARAMFLAG_FOUT: u32 = 0x2;
pub(super) const PARAMFLAG_FRETVAL: u32 = 0x8;
pub(super) const VARIANT_SIZE: usize = 16;

// Backend for COM dispatch calls.
enum ComBackend {
    Dispatch(Arc<DispatchTable>),
    InProc(InProcObject),
}

// Represents an instantiated COM object with a dispatch backend.
pub struct ComObject {
    clsid: String,
    dll_path: String,
    host_path: String,
    backend: ComBackend,
}

impl ComObject {
    pub(crate) fn new_dispatch(
        clsid: String,
        dll_path: String,
        host_path: String,
        dispatch: Arc<DispatchTable>,
    ) -> Self {
        Self {
            clsid,
            dll_path,
            host_path,
            backend: ComBackend::Dispatch(dispatch),
        }
    }

    pub(crate) fn new_inproc(
        clsid: String,
        dll_path: String,
        host_path: String,
        inproc: InProcObject,
    ) -> Self {
        Self {
            clsid,
            dll_path,
            host_path,
            backend: ComBackend::InProc(inproc),
        }
    }

    pub fn clsid(&self) -> &str {
        &self.clsid
    }

    pub fn dll_path(&self) -> &str {
        &self.dll_path
    }

    pub fn host_path(&self) -> &str {
        &self.host_path
    }

    pub fn instance_ptr(&self) -> Option<u32> {
        match &self.backend {
            ComBackend::InProc(inproc) => Some(inproc.dispatch_ptr()),
            ComBackend::Dispatch(_) => None,
        }
    }

    pub fn write_instance_bytes(
        &self,
        vm: &mut Vm,
        offset: u32,
        bytes: &[u8],
    ) -> Result<(), VmError> {
        let Some(base) = self.instance_ptr() else {
            return Err(VmError::InvalidConfig("instance pointer unavailable"));
        };
        vm.write_bytes(base.wrapping_add(offset), bytes)
    }

    pub fn invoke(&self, vm: &mut Vm, dispid: u32, args: &[ComArg]) -> Result<ComValue, VmError> {
        self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)
    }

    pub fn invoke_i4(&self, vm: &mut Vm, dispid: u32, args: &[ComArg]) -> Result<i32, VmError> {
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)? {
            ComValue::I4(value) => Ok(value),
            ComValue::Void => Ok(0),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn invoke_bstr(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<String, VmError> {
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)? {
            ComValue::BStr(value) => Ok(value),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn get_property_bstr(&self, vm: &mut Vm, dispid: u32) -> Result<String, VmError> {
        match self.invoke_with_flags(vm, dispid, &[], DISPATCH_PROPERTYGET)? {
            ComValue::BStr(value) => Ok(value),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn invoke_void(&self, vm: &mut Vm, dispid: u32, args: &[ComArg]) -> Result<(), VmError> {
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)? {
            ComValue::Void => Ok(()),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn set_property_bstr(&self, vm: &mut Vm, dispid: u32, value: &str) -> Result<i32, VmError> {
        match self.invoke_with_flags(
            vm,
            dispid,
            &[ComArg::BStr(value.to_string())],
            DISPATCH_PROPERTYPUT,
        )? {
            ComValue::I4(value) => Ok(value),
            ComValue::Void => Ok(0),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn get_dispids(&self, vm: &mut Vm, names: &[&str]) -> Result<Vec<i32>, VmError> {
        match &self.backend {
            ComBackend::Dispatch(_) => Err(VmError::InvalidConfig(
                "GetIDsOfNames requires in-proc COM object",
            )),
            ComBackend::InProc(inproc) => inproc.get_dispids(vm, names),
        }
    }

    pub fn get_dispid(&self, vm: &mut Vm, name: &str) -> Result<i32, VmError> {
        let mut dispids = self.get_dispids(vm, &[name])?;
        dispids
            .pop()
            .ok_or(VmError::InvalidConfig("GetIDsOfNames returned no dispid"))
    }

    fn invoke_with_flags(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
        flags: u16,
    ) -> Result<ComValue, VmError> {
        let result = match &self.backend {
            ComBackend::Dispatch(dispatch) => dispatch.invoke(vm, dispid, args),
            ComBackend::InProc(inproc) => inproc.invoke(vm, dispid, args, flags),
        };
        // Drain queued threads after COM calls to emulate asynchronous work.
        let _ = vm.run_pending_threads();
        result
    }
}

// In-proc COM object that calls IDispatch::Invoke inside the VM.
pub(crate) struct InProcObject {
    i_dispatch: u32,
    typelib: Option<Arc<TypeLib>>,
}

impl InProcObject {
    pub(crate) fn new(i_dispatch: u32, typelib: Option<TypeLib>) -> Self {
        Self {
            i_dispatch,
            typelib: typelib.map(Arc::new),
        }
    }

    pub(crate) fn dispatch_ptr(&self) -> u32 {
        self.i_dispatch
    }

    fn invoke(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
        flags: u16,
    ) -> Result<ComValue, VmError> {
        let invoke_ptr = vtable_fn(vm, self.i_dispatch, 6)?;
        // Reset COM out parameters for this call.
        vm.clear_last_com_out_params();

        let mut call_flags = flags;
        let mut func = self.find_typelib_func(dispid, flags, args.len());
        if func.is_none() {
            if let Some(fallback) = self.find_typelib_func_any(dispid, args.len()) {
                if fallback.invkind != 0 {
                    call_flags = fallback.invkind;
                }
                func = Some(fallback);
            }
        }

        let (args_ptr, arg_count, out_params) = match func {
            Some(func) => {
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    eprintln!(
                        "[pe_vm] IDispatch::Invoke using typelib params memid=0x{dispid:08X} args={} total={}",
                        args.len(),
                        func.params.len()
                    );
                }
                variant::build_variant_array_typed(vm, args, func)?
            }
            None => (
                variant::build_variant_array(vm, args)?,
                args.len(),
                Vec::new(),
            ),
        };
        if !out_params.is_empty() {
            vm.set_last_com_out_params(out_params);
        }

        let named_args_storage = [DISPID_PROPERTYPUT];
        let named_args = if (call_flags & (DISPATCH_PROPERTYPUT | DISPATCH_PROPERTYPUTREF)) != 0 {
            Some(&named_args_storage[..])
        } else {
            None
        };
        let disp_params_ptr = variant::build_disp_params(vm, args_ptr, arg_count, named_args)?;
        let riid_ptr = vm.alloc_bytes(&[0u8; 16], 4)?;

        let wants_result =
            (call_flags & DISPATCH_METHOD) != 0 || (call_flags & DISPATCH_PROPERTYGET) != 0;
        let result_ptr = if wants_result {
            vm.alloc_bytes(&[0u8; 16], 4)?
        } else {
            0
        };

        let invoke_thiscall = detect_thiscall(vm, invoke_ptr);
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!(
                "[pe_vm] IDispatch::Invoke entry=0x{invoke_ptr:08X} thiscall={invoke_thiscall}"
            );
        }
        let prev_dispatch = vm.set_dispatch_instance(Some(self.i_dispatch));
        let hr = if invoke_thiscall {
            let values = [
                Value::U32(dispid),
                Value::U32(riid_ptr),
                Value::U32(0),
                Value::U32(call_flags as u32),
                Value::U32(disp_params_ptr),
                Value::U32(result_ptr),
                Value::U32(0),
                Value::U32(0),
            ];
            vm.execute_at_with_stack_with_ecx(invoke_ptr, self.i_dispatch, &values)?
        } else {
            let values = [
                Value::U32(self.i_dispatch),
                Value::U32(dispid),
                Value::U32(riid_ptr),
                Value::U32(0),
                Value::U32(call_flags as u32),
                Value::U32(disp_params_ptr),
                Value::U32(result_ptr),
                Value::U32(0),
                Value::U32(0),
            ];
            vm.execute_at_with_stack(invoke_ptr, &values)?
        };
        vm.set_dispatch_instance(prev_dispatch);
        if hr != 0 {
            return Err(VmError::Com(hr));
        }
        if result_ptr == 0 {
            return Ok(ComValue::Void);
        }
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            let vt = vm.read_u16(result_ptr).unwrap_or(0);
            let raw = vm.read_u32(result_ptr + 8).unwrap_or(0);
            // Trace the raw VARIANT payload to debug COM return types.
            eprintln!("[pe_vm] IDispatch::Invoke result vt=0x{vt:04X} raw=0x{raw:08X}");
            if vt == VT_BSTR {
                if let Ok(text) = oleaut32::read_bstr(vm, raw) {
                    if !text.is_empty() {
                        eprintln!("[pe_vm] IDispatch::Invoke BSTR={text}");
                    }
                }
            }
        }
        variant::read_variant(vm, result_ptr)
    }

    fn get_dispids(&self, vm: &mut Vm, names: &[&str]) -> Result<Vec<i32>, VmError> {
        if names.is_empty() {
            return Ok(Vec::new());
        }
        let get_ids_ptr = vtable_fn(vm, self.i_dispatch, 5)?;
        let riid_ptr = vm.alloc_bytes(&[0u8; 16], 4)?;
        let mut name_ptrs = Vec::with_capacity(names.len());
        for name in names {
            let mut units: Vec<u16> = name.encode_utf16().collect();
            units.push(0);
            let mut bytes = Vec::with_capacity(units.len() * 2);
            for unit in units {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
            let ptr = vm.alloc_bytes(&bytes, 2)?;
            name_ptrs.push(ptr);
        }
        let mut name_bytes = Vec::with_capacity(name_ptrs.len() * 4);
        for ptr in &name_ptrs {
            name_bytes.extend_from_slice(&ptr.to_le_bytes());
        }
        let names_ptr = vm.alloc_bytes(&name_bytes, 4)?;
        let dispid_ptr = vm.alloc_bytes(&vec![0u8; names.len() * 4], 4)?;
        let lcid = 0u32;
        let get_ids_thiscall = detect_thiscall(vm, get_ids_ptr);
        let hr = if get_ids_thiscall {
            let values = [
                Value::U32(riid_ptr),
                Value::U32(names_ptr),
                Value::U32(names.len() as u32),
                Value::U32(lcid),
                Value::U32(dispid_ptr),
            ];
            vm.execute_at_with_stack_with_ecx(get_ids_ptr, self.i_dispatch, &values)?
        } else {
            let values = [
                Value::U32(self.i_dispatch),
                Value::U32(riid_ptr),
                Value::U32(names_ptr),
                Value::U32(names.len() as u32),
                Value::U32(lcid),
                Value::U32(dispid_ptr),
            ];
            vm.execute_at_with_stack(get_ids_ptr, &values)?
        };
        if hr != 0 {
            return Err(VmError::Com(hr));
        }
        let mut dispids = Vec::with_capacity(names.len());
        for index in 0..names.len() {
            let value = vm.read_u32(dispid_ptr + (index as u32) * 4)?;
            dispids.push(value as i32);
        }
        Ok(dispids)
    }

    fn find_typelib_func(&self, dispid: u32, flags: u16, args_len: usize) -> Option<&FuncDesc> {
        let lib = self.typelib.as_ref()?;
        let mut best: Option<&FuncDesc> = None;
        let mut best_diff = usize::MAX;
        let mut best_total = 0usize;
        for info in &lib.typeinfos {
            for func in &info.funcs {
                if func.memid != dispid {
                    continue;
                }
                if func.invkind != 0 && (flags & func.invkind) == 0 {
                    continue;
                }
                let expected_inputs = expected_inputs_for_func(func);
                if args_len > expected_inputs {
                    continue;
                }
                let diff = expected_inputs.saturating_sub(args_len);
                let total = func.params.len();
                if diff < best_diff || (diff == best_diff && total > best_total) {
                    best = Some(func);
                    best_diff = diff;
                    best_total = total;
                }
            }
        }
        best
    }

    fn find_typelib_func_any(&self, dispid: u32, args_len: usize) -> Option<&FuncDesc> {
        let lib = self.typelib.as_ref()?;
        let mut best: Option<&FuncDesc> = None;
        let mut best_diff = usize::MAX;
        let mut best_total = 0usize;
        for info in &lib.typeinfos {
            for func in &info.funcs {
                if func.memid != dispid {
                    continue;
                }
                let expected_inputs = expected_inputs_for_func(func);
                if args_len > expected_inputs {
                    continue;
                }
                let diff = expected_inputs.saturating_sub(args_len);
                let total = func.params.len();
                if diff < best_diff || (diff == best_diff && total > best_total) {
                    best = Some(func);
                    best_diff = diff;
                    best_total = total;
                }
            }
        }
        best
    }
}

fn expected_inputs_for_func(func: &FuncDesc) -> usize {
    // Count parameters that could be inputs.
    // FRETVAL params are normally not inputs, but some TypeLibs incorrectly mark input params
    // with FRETVAL. If a method already has a return type (not VOID/EMPTY), treat single
    // FRETVAL params as potential inputs.
    let has_explicit_return = func.ret_vt != VT_EMPTY && func.ret_vt != 0;
    let only_fretval = func.params.len() == 1
        && func
            .params
            .iter()
            .all(|p| (p.flags & PARAMFLAG_FRETVAL) != 0);

    func.params
        .iter()
        .filter(|param| {
            let is_fretval = (param.flags & PARAMFLAG_FRETVAL) != 0;
            if !is_fretval {
                return !is_out_only(param.flags);
            }
            // If there's an explicit return type and only one FRETVAL param,
            // treat it as a potential input
            if has_explicit_return && only_fretval {
                return true;
            }
            false
        })
        .count()
}

fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}
