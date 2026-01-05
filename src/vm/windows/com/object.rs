//! COM object wrapper with dispatch helpers.

use std::sync::Arc;

use crate::vm::{ComOutParam, Value, Vm, VmError};
use crate::vm::windows::oleaut32;
use crate::vm::windows::oleaut32::typelib::{FuncDesc, TypeLib};

use super::{ComArg, ComValue, DispatchTable};

const DISPATCH_METHOD: u16 = 0x1;
const DISPATCH_PROPERTYGET: u16 = 0x2;
const VT_EMPTY: u16 = 0;
const VT_I4: u16 = 3;
const VT_BSTR: u16 = 8;
const VT_UI4: u16 = 19;
const VT_INT: u16 = 22;
const VT_UINT: u16 = 23;
const VT_VARIANT: u16 = 12;
const VT_BYREF: u16 = 0x4000;
const VT_USERDEFINED: u16 = 0x1D;
const DISP_E_TYPEMISMATCH: u32 = 0x80020005;
const DISP_E_BADPARAMCOUNT: u32 = 0x8002000E;
const PARAMFLAG_FIN: u32 = 0x1;
const PARAMFLAG_FOUT: u32 = 0x2;
const PARAMFLAG_FRETVAL: u32 = 0x8;
const VARIANT_SIZE: usize = 16;

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
                build_variant_array_typed(vm, args, func)?
            }
            None => (build_variant_array(vm, args)?, args.len(), Vec::new()),
        };
        if !out_params.is_empty() {
            vm.set_last_com_out_params(out_params);
        }

        let disp_params_ptr = build_disp_params(vm, args_ptr, arg_count)?;
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
        read_variant(vm, result_ptr)
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

// Read a vtable entry from a COM interface pointer.
pub(crate) fn vtable_fn(vm: &Vm, obj_ptr: u32, index: u32) -> Result<u32, VmError> {
    let vtable_ptr = vm.read_u32(obj_ptr)?;
    if !vm.contains_addr(vtable_ptr) {
        return Err(VmError::MemoryOutOfRange);
    }
    let entry = vtable_ptr.wrapping_add(index * 4);
    vm.read_u32(entry)
}

fn detect_thiscall(vm: &Vm, entry: u32) -> bool {
    let mut bytes = [0u8; 96];
    for (idx, slot) in bytes.iter_mut().enumerate() {
        *slot = vm.read_u8(entry.wrapping_add(idx as u32)).unwrap_or(0);
    }

    for idx in 0..bytes.len().saturating_sub(3) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x44 && bytes[idx + 2] == 0x24 && bytes[idx + 3] == 0x04 {
            return false;
        }
    }
    for idx in 0..bytes.len().saturating_sub(2) {
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x45 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x75 && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x4D && bytes[idx + 2] == 0x08 {
            return false;
        }
        if bytes[idx] == 0x8B && bytes[idx + 1] == 0x55 && bytes[idx + 2] == 0x08 {
            return false;
        }
    }

    for idx in 0..bytes.len().saturating_sub(1) {
        let opcode = bytes[idx];
        if !matches!(opcode, 0x8B | 0x89 | 0x8A | 0x8D) {
            continue;
        }
        let modrm = bytes[idx + 1];
        let mod_bits = modrm & 0xC0;
        let rm = modrm & 0x07;
        if mod_bits != 0xC0 && rm == 0x01 {
            return true;
        }
    }
    false
}

// Build a VARIANT array in right-to-left order.
fn build_variant_array(vm: &mut Vm, args: &[ComArg]) -> Result<u32, VmError> {
    if args.is_empty() {
        return Ok(0);
    }
    let total = args.len() * VARIANT_SIZE;
    let base = vm.alloc_bytes(&vec![0u8; total], 4)?;
    for (index, arg) in args.iter().rev().enumerate() {
        write_variant(vm, base + (index as u32) * VARIANT_SIZE as u32, arg)?;
    }
    Ok(base)
}

// Build a VARIANT array using typelib parameter metadata to fill out-params.
fn build_variant_array_typed(
    vm: &mut Vm,
    args: &[ComArg],
    func: &FuncDesc,
) -> Result<(u32, usize, Vec<ComOutParam>), VmError> {
    if args.is_empty() {
        return Ok((0, 0, Vec::new()));
    }

    let mut input_vts = Vec::new();
    for param in &func.params {
        if (param.flags & PARAMFLAG_FRETVAL) != 0 || is_out_only(param.flags) {
            continue;
        }
        input_vts.push(param.vt);
    }
    if args.len() > input_vts.len() {
        let base = build_variant_array(vm, args)?;
        return Ok((base, args.len(), Vec::new()));
    }

    let mut params = Vec::with_capacity(args.len());
    for (arg, vt) in args.iter().zip(input_vts.iter()) {
        let (vt, value, _out_ptr) = build_param_variant(vm, *vt, Some(arg), false)?;
        params.push(ParamValue { vt, value });
    }

    let total = params.len() * VARIANT_SIZE;
    let base = vm.alloc_bytes(&vec![0u8; total], 4)?;
    for (index, param) in params.iter().rev().enumerate() {
        write_variant_raw(vm, base + (index as u32) * VARIANT_SIZE as u32, param.vt, param.value)?;
    }
    Ok((base, params.len(), Vec::new()))
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

struct ParamValue {
    vt: u16,
    value: u32,
}

fn build_param_variant(
    vm: &mut Vm,
    vt: u16,
    arg: Option<&ComArg>,
    force_out: bool,
) -> Result<(u16, u32, Option<u32>), VmError> {
    let base_vt = vt & !VT_BYREF;
    let mut out_ptr = None;
    let value = if let Some(arg) = arg {
        let base_value = match (base_vt, arg) {
            (VT_I4 | VT_INT | VT_USERDEFINED, ComArg::I4(value)) => *value as u32,
            (VT_I4 | VT_INT | VT_USERDEFINED, ComArg::U32(value)) => *value,
            (VT_UI4 | VT_UINT, ComArg::I4(value)) => *value as u32,
            (VT_UI4 | VT_UINT, ComArg::U32(value)) => *value,
            (VT_BSTR, ComArg::BStr(text)) => oleaut32::alloc_bstr(vm, text)?,
            _ => return Err(VmError::Com(DISP_E_TYPEMISMATCH)),
        };
        if (vt & VT_BYREF) != 0 {
            let ptr = alloc_param_buffer(vm, vt)?;
            write_base_value(vm, base_vt, ptr, base_value)?;
            out_ptr = Some(ptr);
            ptr
        } else {
            base_value
        }
    } else {
        if force_out || (vt & VT_BYREF) != 0 || base_vt == VT_VARIANT || base_vt == VT_USERDEFINED
        {
            let ptr = alloc_param_buffer(vm, vt)?;
            out_ptr = Some(ptr);
            ptr
        } else {
            0
        }
    };
    Ok((vt, value, out_ptr))
}

fn write_variant_raw(vm: &mut Vm, addr: u32, vt: u16, value: u32) -> Result<(), VmError> {
    vm.write_u16(addr, vt)?;
    vm.write_u16(addr + 2, 0)?;
    vm.write_u16(addr + 4, 0)?;
    vm.write_u16(addr + 6, 0)?;
    vm.write_u32(addr + 8, value)?;
    vm.write_u32(addr + 12, 0)?;
    Ok(())
}

fn write_base_value(vm: &mut Vm, base_vt: u16, ptr: u32, value: u32) -> Result<(), VmError> {
    match base_vt {
        VT_I4 | VT_UI4 | VT_BSTR => vm.write_u32(ptr, value),
        _ => Err(VmError::Com(DISP_E_TYPEMISMATCH)),
    }
}

fn alloc_param_buffer(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    let size = match base {
        VT_I4 | VT_UI4 | VT_BSTR => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    vm.alloc_bytes(&vec![0u8; size], 4)
}

fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}

// Read a VARIANT into a COM value.
fn read_variant(vm: &Vm, addr: u32) -> Result<ComValue, VmError> {
    let vt = vm.read_u16(addr)?;
    match vt {
        VT_EMPTY => Ok(ComValue::Void),
        VT_I4 => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_UI4 | VT_INT | VT_UINT => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_BSTR => {
            let ptr = vm.read_u32(addr + 8)?;
            let value = oleaut32::read_bstr(vm, ptr)?;
            Ok(ComValue::BStr(value))
        }
        _ => Err(VmError::InvalidConfig("unsupported variant return type")),
    }
}
