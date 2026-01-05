//! COM object wrapper with dispatch helpers.

use std::sync::Arc;

use crate::vm::{Value, Vm, VmError};
use crate::vm::windows::oleaut32;

use super::{ComArg, ComValue, DispatchTable};

const DISPATCH_METHOD: u16 = 0x1;
const VT_EMPTY: u16 = 0;
const VT_I4: u16 = 3;
const VT_BSTR: u16 = 8;
const VT_UI4: u16 = 19;

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
        match &self.backend {
            ComBackend::Dispatch(dispatch) => dispatch.invoke(vm, dispid, args),
            ComBackend::InProc(inproc) => inproc.invoke(vm, dispid, args, DISPATCH_METHOD),
        }
    }

    pub fn invoke_i4(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<i32, VmError> {
        match self.invoke(vm, dispid, args)? {
            ComValue::I4(value) => Ok(value),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }

    pub fn invoke_bstr(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<String, VmError> {
        match self.invoke(vm, dispid, args)? {
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
        match self.invoke(vm, dispid, args)? {
            ComValue::Void => Ok(()),
            _ => Err(VmError::InvalidConfig("dispatch return type mismatch")),
        }
    }
}

// In-proc COM object that calls IDispatch::Invoke inside the VM.
pub(crate) struct InProcObject {
    i_dispatch: u32,
}

impl InProcObject {
    pub(crate) fn new(i_dispatch: u32) -> Self {
        Self { i_dispatch }
    }

    fn invoke(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
        flags: u16,
    ) -> Result<ComValue, VmError> {
        let invoke_ptr = vtable_fn(vm, self.i_dispatch, 6)?;
        let args_ptr = build_variant_array(vm, args)?;
        let disp_params_ptr = build_disp_params(vm, args_ptr, args.len())?;
        let riid_ptr = vm.alloc_bytes(&[0u8; 16], 4)?;

        let result_ptr = if flags == DISPATCH_METHOD {
            vm.alloc_bytes(&[0u8; 16], 4)?
        } else {
            0
        };

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

        let hr = vm.execute_at_with_stack(invoke_ptr, &values)?;
        if hr != 0 {
            return Err(VmError::Com(hr));
        }
        if result_ptr == 0 {
            return Ok(ComValue::Void);
        }
        read_variant(vm, result_ptr)
    }
}

// Read a vtable entry from a COM interface pointer.
pub(crate) fn vtable_fn(vm: &Vm, obj_ptr: u32, index: u32) -> Result<u32, VmError> {
    let vtable_ptr = vm.read_u32(obj_ptr)?;
    if !vm.contains_addr(vtable_ptr) {
        return Err(VmError::MemoryOutOfRange);
    }
    let entry = vtable_ptr.wrapping_add(index * 4);
    vm.read_u32(entry)
}

// Build a VARIANT array in right-to-left order.
fn build_variant_array(vm: &mut Vm, args: &[ComArg]) -> Result<u32, VmError> {
    if args.is_empty() {
        return Ok(0);
    }
    let total = args.len() * 16;
    let base = vm.alloc_bytes(&vec![0u8; total], 4)?;
    for (index, arg) in args.iter().rev().enumerate() {
        write_variant(vm, base + (index as u32) * 16, arg)?;
    }
    Ok(base)
}

// Build a DISPPARAMS structure for Invoke.
fn build_disp_params(vm: &mut Vm, args_ptr: u32, arg_count: usize) -> Result<u32, VmError> {
    let base = vm.alloc_bytes(&[0u8; 16], 4)?;
    vm.write_u32(base, args_ptr)?;
    vm.write_u32(base + 4, 0)?;
    vm.write_u32(base + 8, arg_count as u32)?;
    vm.write_u32(base + 12, 0)?;
    Ok(base)
}

// Write a VARIANT from a COM argument.
fn write_variant(vm: &mut Vm, addr: u32, arg: &ComArg) -> Result<(), VmError> {
    vm.write_u16(addr, VT_EMPTY)?;
    vm.write_u16(addr + 2, 0)?;
    vm.write_u16(addr + 4, 0)?;
    vm.write_u16(addr + 6, 0)?;
    vm.write_u32(addr + 8, 0)?;
    vm.write_u32(addr + 12, 0)?;
    match arg {
        ComArg::I4(value) => {
            vm.write_u16(addr, VT_I4)?;
            vm.write_u32(addr + 8, *value as u32)?;
        }
        ComArg::U32(value) => {
            vm.write_u16(addr, VT_UI4)?;
            vm.write_u32(addr + 8, *value)?;
        }
        ComArg::BStr(value) => {
            let bstr = oleaut32::alloc_bstr(vm, value)?;
            vm.write_u16(addr, VT_BSTR)?;
            vm.write_u32(addr + 8, bstr)?;
        }
    }
    Ok(())
}

// Read a VARIANT into a COM value.
fn read_variant(vm: &Vm, addr: u32) -> Result<ComValue, VmError> {
    let vt = vm.read_u16(addr)?;
    match vt {
        VT_EMPTY => Ok(ComValue::Void),
        VT_I4 => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_UI4 => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_BSTR => {
            let ptr = vm.read_u32(addr + 8)?;
            let value = oleaut32::read_bstr(vm, ptr)?;
            Ok(ComValue::BStr(value))
        }
        _ => Err(VmError::InvalidConfig("unsupported variant return type")),
    }
}
