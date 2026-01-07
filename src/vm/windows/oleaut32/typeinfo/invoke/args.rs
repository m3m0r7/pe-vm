use crate::vm::windows::oleaut32::typelib::FuncDesc;
use crate::vm::{ComOutParam, Value, Vm};

use super::super::super::constants::{
    DISP_E_BADPARAMCOUNT, DISP_E_TYPEMISMATCH, PARAMFLAG_FOUT, PARAMFLAG_FRETVAL, VARIANT_SIZE,
    VT_ARRAY, VT_BSTR, VT_BYREF, VT_NULL, VT_USERDEFINED, VT_VARIANT,
};

use super::variants::{alloc_out_arg, default_arg_for_vt, is_out_only, read_variant_arg};

pub(super) struct InvokeValues {
    pub(super) values: Vec<Value>,
    pub(super) out_params: Vec<ComOutParam>,
    pub(super) retval_param: Option<(usize, u16)>,
}

pub(super) fn build_invoke_values(
    vm: &mut Vm,
    func: &FuncDesc,
    disp_params: u32,
    arg_err: u32,
) -> Result<InvokeValues, u32> {
    let args_ptr = vm.read_u32(disp_params).unwrap_or(0);
    let arg_count = vm.read_u32(disp_params + 8).unwrap_or(0) as usize;
    let mut input_positions = vec![None; func.params.len()];
    let mut input_count = 0usize;
    for (index, param) in func.params.iter().enumerate() {
        let flags = param.flags;
        if (flags & PARAMFLAG_FRETVAL) != 0 || is_out_param(param.vt, flags) {
            input_positions[index] = None;
        } else {
            input_positions[index] = Some(input_count);
            input_count += 1;
        }
    }
    let mut positional_fallback = false;
    if arg_count > input_count {
        if arg_count > func.params.len() {
            if arg_err != 0 {
                let _ = vm.write_u32(arg_err, arg_count as u32);
            }
            return Err(DISP_E_BADPARAMCOUNT);
        }
        positional_fallback = true;
    }

    let mut values = Vec::with_capacity(func.params.len() + 1);
    let mut out_params = Vec::new();
    let provided = arg_count.min(input_count);
    let mut retval_param: Option<(usize, u16)> = None;
    for (index, param) in func.params.iter().enumerate() {
        let flags = param.flags;
        let is_retval = (flags & PARAMFLAG_FRETVAL) != 0;
        let out_only = is_out_param(param.vt, flags);
        let base_vt = param.vt & !VT_BYREF;
        let record_out = is_retval
            || out_only
            || ((flags & PARAMFLAG_FOUT) != 0
                && ((param.vt & VT_BYREF) != 0
                    || (param.vt & VT_ARRAY) != 0
                    || base_vt == VT_USERDEFINED
                    || base_vt == VT_VARIANT
                    || base_vt == VT_BSTR));
        if positional_fallback {
            // For out-only params (including FRETVAL), always allocate our own buffer
            // rather than reading from caller-provided VARIANTs which may not match.
            if out_only {
                let value = match alloc_out_arg(vm, param.vt) {
                    Ok(value) => value,
                    Err(_) => return Err(DISP_E_TYPEMISMATCH),
                };
                if is_retval && retval_param.is_none() {
                    retval_param = Some((index, param.vt));
                }
                values.push(Value::U32(value));
                if record_out {
                    out_params.push(ComOutParam {
                        index,
                        vt: param.vt,
                        flags,
                        ptr: value,
                    });
                }
                continue;
            }
            if index < arg_count {
                let arg_index = arg_count.saturating_sub(1).saturating_sub(index);
                let var_ptr = args_ptr.wrapping_add((arg_index * VARIANT_SIZE) as u32);
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    let actual_vt = vm.read_u16(var_ptr).unwrap_or(0);
                    let actual_val = vm.read_u32(var_ptr + 8).unwrap_or(0);
                    eprintln!("[pe_vm] build_invoke_values positional idx={index} arg_index={arg_index} var_ptr=0x{var_ptr:08X} actual_vt=0x{actual_vt:04X} val=0x{actual_val:08X} expected_vt=0x{:04X}", param.vt);
                }
                let value = match read_variant_arg(vm, var_ptr, param.vt) {
                    Ok(value) => value,
                    Err(_) => {
                        if arg_err != 0 {
                            let _ = vm.write_u32(arg_err, arg_index as u32);
                        }
                        return Err(DISP_E_TYPEMISMATCH);
                    }
                };
                values.push(Value::U32(value));
                if is_retval && retval_param.is_none() {
                    retval_param = Some((index, param.vt));
                }
                if record_out {
                    out_params.push(ComOutParam {
                        index,
                        vt: param.vt,
                        flags,
                        ptr: value,
                    });
                }
                continue;
            }
            let value = match alloc_out_arg(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return Err(DISP_E_TYPEMISMATCH),
            };
            if is_retval && retval_param.is_none() {
                retval_param = Some((index, param.vt));
            }
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        }
        let Some(position) = input_positions[index] else {
            let value = match alloc_out_arg(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return Err(DISP_E_TYPEMISMATCH),
            };
            if is_retval && retval_param.is_none() {
                retval_param = Some((index, param.vt));
            }
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        };
        if position >= provided {
            let value = match default_arg_for_vt(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return Err(DISP_E_TYPEMISMATCH),
            };
            values.push(Value::U32(value));
            if record_out {
                out_params.push(ComOutParam {
                    index,
                    vt: param.vt,
                    flags,
                    ptr: value,
                });
            }
            continue;
        }
        let arg_index = provided.saturating_sub(1).saturating_sub(position);
        let var_ptr = args_ptr.wrapping_add((arg_index * VARIANT_SIZE) as u32);
        let value = match read_variant_arg(vm, var_ptr, param.vt) {
            Ok(value) => value,
            Err(_) => {
                if arg_err != 0 {
                    let _ = vm.write_u32(arg_err, arg_index as u32);
                }
                return Err(DISP_E_TYPEMISMATCH);
            }
        };
        values.push(Value::U32(value));
        if record_out {
            out_params.push(ComOutParam {
                index,
                vt: param.vt,
                flags,
                ptr: value,
            });
        }
    }

    Ok(InvokeValues {
        values,
        out_params,
        retval_param,
    })
}

fn is_out_param(vt: u16, flags: u32) -> bool {
    if is_out_only(flags) {
        return true;
    }
    if (flags & PARAMFLAG_FOUT) == 0 || (vt & VT_BYREF) != 0 {
        return false;
    }
    let base = vt & !(VT_BYREF | VT_ARRAY);
    matches!(base, VT_USERDEFINED | VT_VARIANT | VT_BSTR | VT_NULL) || (vt & VT_ARRAY) != 0
}

pub(super) fn trace_values(vm: &Vm, func: &FuncDesc, values: &[Value]) {
    if std::env::var("PE_VM_TRACE_COM").is_err() {
        return;
    }
    let rendered = values
        .iter()
        .map(|value| match value {
            Value::U32(v) => format!("0x{v:08X}"),
            Value::U64(v) => format!("0x{v:016X}"),
            Value::String(text) => format!("{text:?}"),
            Value::Env(_) => "<env>".to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ");
    eprintln!("[pe_vm] ITypeInfo::Invoke args=[{rendered}]");
    for (index, param) in func.params.iter().enumerate() {
        if (param.vt & !VT_BYREF) != VT_BSTR {
            continue;
        }
        let Some(Value::U32(ptr)) = values.get(index) else {
            continue;
        };
        let mut preview = String::new();
        for i in 0..8u32 {
            let addr = ptr.wrapping_add(i * 2);
            let value = vm.read_u16(addr).unwrap_or(0);
            if i != 0 {
                preview.push(' ');
            }
            preview.push_str(&format!("{value:04X}"));
        }
        let byte_len = if *ptr >= 4 {
            vm.read_u32(ptr.saturating_sub(4)).unwrap_or(0)
        } else {
            0
        };
        let char_len = byte_len / 2;
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke bstr[{index}] ptr=0x{ptr:08X} len={char_len} utf16={preview}"
        );
    }
}
