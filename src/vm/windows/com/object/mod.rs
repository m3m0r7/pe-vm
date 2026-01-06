//! COM object wrapper with dispatch helpers.

mod variant;
mod vtable;

use std::sync::Arc;

use crate::vm::{Value, Vm, VmError};
use crate::vm::windows::oleaut32;
use crate::vm::windows::oleaut32::typelib::{FuncDesc, TypeLib};

use super::{ComArg, ComValue, DispatchTable};

pub(crate) use vtable::vtable_fn;
use vtable::detect_thiscall;

const DISPATCH_METHOD: u16 = 0x1;
const DISPATCH_PROPERTYGET: u16 = 0x2;
pub(super) const VT_EMPTY: u16 = 0;
pub(super) const VT_I4: u16 = 3;
pub(super) const VT_BSTR: u16 = 8;
pub(super) const VT_UI4: u16 = 19;
pub(super) const VT_INT: u16 = 22;
pub(super) const VT_UINT: u16 = 23;
pub(super) const VT_VARIANT: u16 = 12;
pub(super) const VT_BYREF: u16 = 0x4000;
pub(super) const VT_USERDEFINED: u16 = 0x1D;
pub(super) const DISP_E_TYPEMISMATCH: u32 = 0x80020005;
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

    pub fn invoke(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<ComValue, VmError> {
        self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)
    }

    pub fn invoke_i4(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<i32, VmError> {
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD | DISPATCH_PROPERTYGET)? {
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
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD | DISPATCH_PROPERTYGET)? {
            ComValue::BStr(value) => Ok(value),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn invoke_void(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<(), VmError> {
        match self.invoke_with_flags(vm, dispid, args, DISPATCH_METHOD)? {
            ComValue::Void => Ok(()),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    fn invoke_with_flags(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
        flags: u16,
    ) -> Result<ComValue, VmError> {
        match &self.backend {
            ComBackend::Dispatch(dispatch) => dispatch.invoke(vm, dispid, args),
            ComBackend::InProc(inproc) => inproc.invoke(vm, dispid, args, flags),
        }
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

        let (args_ptr, arg_count, out_params) = match self.find_typelib_func(dispid, flags) {
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
            None => (variant::build_variant_array(vm, args)?, args.len(), Vec::new()),
        };
        if !out_params.is_empty() {
            vm.set_last_com_out_params(out_params);
        }

        let disp_params_ptr = variant::build_disp_params(vm, args_ptr, arg_count)?;
        let riid_ptr = vm.alloc_bytes(&[0u8; 16], 4)?;

        let result_ptr = if flags == DISPATCH_METHOD {
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
                Value::U32(flags as u32),
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
                Value::U32(flags as u32),
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
            eprintln!(
                "[pe_vm] IDispatch::Invoke result vt=0x{vt:04X} raw=0x{raw:08X}"
            );
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

    fn find_typelib_func(&self, dispid: u32, flags: u16) -> Option<&FuncDesc> {
        let lib = self.typelib.as_ref()?;
        for info in &lib.typeinfos {
            for func in &info.funcs {
                if func.memid != dispid {
                    continue;
                }
                if func.invkind != 0 && (flags & func.invkind) == 0 {
                    continue;
                }
                return Some(func);
            }
        }
        None
    }
}
