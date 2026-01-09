//! VARIANT array and argument helpers for COM dispatch.

use crate::vm::windows::oleaut32;
use crate::vm::windows::oleaut32::typelib::FuncDesc;
use crate::vm::{ComOutParam, Vm, VmError};

use super::{ComArg, ComValue};
use super::{
    DISP_E_PARAMNOTFOUND, DISP_E_TYPEMISMATCH, PARAMFLAG_FIN, PARAMFLAG_FOUT, PARAMFLAG_FRETVAL,
    VARIANT_SIZE, VT_BSTR, VT_BYREF, VT_EMPTY, VT_ERROR, VT_I1, VT_I4, VT_INT, VT_NULL, VT_UI4,
    VT_UINT, VT_USERDEFINED, VT_VARIANT,
};

// Build a VARIANT array in right-to-left order.
pub(super) fn build_variant_array(vm: &mut Vm, args: &[ComArg]) -> Result<u32, VmError> {
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
pub(super) fn build_variant_array_typed(
    vm: &mut Vm,
    args: &[ComArg],
    func: &FuncDesc,
) -> Result<(u32, usize, Vec<ComOutParam>), VmError> {
    if func.params.is_empty() {
        return Ok((0, 0, Vec::new()));
    }

    // Detect if TypeLib incorrectly marks input params as FRETVAL.
    // If method has explicit return type and only one FRETVAL param AND we have args, treat it as input.
    // If caller passed no args, the FRETVAL param is a true output (not an input).
    let has_explicit_return = func.ret_vt != VT_EMPTY && func.ret_vt != 0;
    let only_fretval = func.params.len() == 1
        && func
            .params
            .iter()
            .all(|p| (p.flags & PARAMFLAG_FRETVAL) != 0);
    let fretval_is_input = has_explicit_return && only_fretval && !args.is_empty();

    let expected_inputs = func
        .params
        .iter()
        .filter(|param| {
            let is_fretval = (param.flags & PARAMFLAG_FRETVAL) != 0;
            if !is_fretval {
                return !is_out_only(param.flags);
            }
            fretval_is_input
        })
        .count();
    if args.len() > expected_inputs {
        let base = build_variant_array(vm, args)?;
        return Ok((base, args.len(), Vec::new()));
    }

    let mut params = Vec::with_capacity(func.params.len());
    let mut out_params = Vec::new();
    let mut input_iter = args.iter();
    for (index, param) in func.params.iter().enumerate() {
        let is_retval = (param.flags & PARAMFLAG_FRETVAL) != 0;
        // Skip FRETVAL params unless they should be treated as inputs
        if is_retval && !fretval_is_input {
            continue;
        }
        let out_only = is_out_only(param.flags);
        let in_out = (param.flags & PARAMFLAG_FOUT) != 0 && (param.flags & PARAMFLAG_FIN) != 0;
        let arg = if out_only { None } else { input_iter.next() };
        let force_out = out_only || in_out;
        let (vt, value, out_ptr) = build_param_variant(vm, param.vt, arg, force_out)?;
        if let Some(ptr) = out_ptr {
            out_params.push(ComOutParam {
                index,
                vt,
                flags: param.flags,
                ptr,
            });
        }
        params.push(ParamValue { vt, value });
    }

    if params.is_empty() {
        return Ok((0, 0, out_params));
    }

    let total = params.len() * VARIANT_SIZE;
    let base = vm.alloc_bytes(&vec![0u8; total], 4)?;
    for (index, param) in params.iter().rev().enumerate() {
        write_variant_raw(
            vm,
            base + (index as u32) * VARIANT_SIZE as u32,
            param.vt,
            param.value,
        )?;
    }
    Ok((base, params.len(), out_params))
}

// Build a DISPPARAMS structure for Invoke.
pub(super) fn build_disp_params(
    vm: &mut Vm,
    args_ptr: u32,
    arg_count: usize,
    named_args: Option<&[i32]>,
) -> Result<u32, VmError> {
    let base = vm.alloc_bytes(&[0u8; 16], 4)?;
    vm.write_u32(base, args_ptr)?;
    if let Some(args) = named_args {
        let mut raw = Vec::with_capacity(args.len() * 4);
        for arg in args {
            raw.extend_from_slice(&(*arg as u32).to_le_bytes());
        }
        let named_ptr = vm.alloc_bytes(&raw, 4)?;
        vm.write_u32(base + 4, named_ptr)?;
        vm.write_u32(base + 12, args.len() as u32)?;
    } else {
        vm.write_u32(base + 4, 0)?;
        vm.write_u32(base + 12, 0)?;
    }
    vm.write_u32(base + 8, arg_count as u32)?;
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
        ComArg::Ansi(_) => {
            return Err(VmError::Com(DISP_E_TYPEMISMATCH));
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
        if base_vt == VT_I1 {
            if let ComArg::Ansi(text) = arg {
                let mut bytes = text.as_bytes().to_vec();
                bytes.push(0);
                let data_ptr = vm.alloc_bytes(&bytes, 1)?;
                if (vt & VT_BYREF) != 0 {
                    out_ptr = Some(data_ptr);
                    data_ptr
                } else {
                    bytes[0] as u32
                }
            } else {
                return Err(VmError::Com(DISP_E_TYPEMISMATCH));
            }
        } else {
            let base_value = match (base_vt, arg) {
                (VT_I4 | VT_INT | VT_USERDEFINED | VT_NULL, ComArg::I4(value)) => *value as u32,
                (VT_I4 | VT_INT | VT_USERDEFINED | VT_NULL, ComArg::U32(value)) => *value,
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
        }
    } else if !force_out {
        return Ok((VT_ERROR, DISP_E_PARAMNOTFOUND, None));
    } else if (vt & VT_BYREF) != 0 || base_vt == VT_VARIANT || base_vt == VT_USERDEFINED {
        let ptr = alloc_param_buffer(vm, vt)?;
        if base_vt == VT_BSTR {
            let empty = oleaut32::alloc_bstr(vm, "")?;
            write_base_value(vm, base_vt, ptr, empty)?;
        }
        out_ptr = Some(ptr);
        ptr
    } else {
        0
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
        VT_I1 | VT_I4 | VT_INT | VT_UI4 | VT_UINT | VT_BSTR | VT_NULL => vm.write_u32(ptr, value),
        _ => Err(VmError::Com(DISP_E_TYPEMISMATCH)),
    }
}

fn alloc_param_buffer(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    let size = match base {
        VT_I1 | VT_I4 | VT_UI4 | VT_BSTR | VT_NULL | VT_INT | VT_UINT => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    vm.alloc_bytes(&vec![0u8; size], 4)
}

fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}

// Read a VARIANT into a COM value.
pub(super) fn read_variant(vm: &Vm, addr: u32) -> Result<ComValue, VmError> {
    let vt = vm.read_u16(addr)?;
    match vt {
        VT_EMPTY => Ok(ComValue::Void),
        VT_I4 => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_UI4 | VT_INT | VT_UINT => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_ERROR => Ok(ComValue::I4(vm.read_u32(addr + 8)? as i32)),
        VT_BSTR => {
            let ptr = vm.read_u32(addr + 8)?;
            let value = oleaut32::read_bstr(vm, ptr)?;
            Ok(ComValue::BStr(value))
        }
        _ => Err(VmError::InvalidConfig("unsupported variant return type")),
    }
}
