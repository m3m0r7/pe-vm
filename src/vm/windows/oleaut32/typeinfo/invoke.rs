//! ITypeInfo::Invoke implementation.

use crate::vm::{ComOutParam, Value, Vm, VmError};

use crate::vm::windows::oleaut32::typelib;

use super::helpers::resolve_typeinfo_info;
use super::super::constants::{
    DISP_E_BADPARAMCOUNT, DISP_E_MEMBERNOTFOUND, DISP_E_TYPEMISMATCH, E_NOTIMPL, PARAMFLAG_FIN,
    PARAMFLAG_FOUT, PARAMFLAG_FRETVAL, S_OK, VARIANT_SIZE, VT_BSTR, VT_BYREF, VT_EMPTY, VT_HRESULT,
    VT_I4, VT_INT, VT_UI4, VT_UINT, VT_USERDEFINED, VT_VARIANT, VT_VOID,
};
use super::super::variant::write_variant_u32;

pub(super) fn typeinfo_invoke(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    let Some(info) = typelib::get_typeinfo(info_id) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    vm.set_last_com_out_params(Vec::new());
    let mut slots = [0u32; 9];
    for (idx, slot) in slots.iter_mut().enumerate() {
        *slot = vm.read_u32(stack_ptr.wrapping_add((idx * 4) as u32)).unwrap_or(0);
    }
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        let mut line = format!("[pe_vm] ITypeInfo::Invoke stack thiscall={thiscall}");
        for (idx, value) in slots.iter().enumerate() {
            line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
        }
        let ecx = vm.reg32(crate::vm::REG_ECX);
        line.push_str(&format!(" ecx=0x{ecx:08X}"));
        eprintln!("{line}");
    }

    #[derive(Clone, Copy)]
    struct InvokeArgs {
        instance: u32,
        memid: u32,
        flags: u16,
        disp_params: u32,
        result_ptr: u32,
        arg_err: u32,
        layout: &'static str,
    }

    let base = if thiscall { 1 } else { 2 };
    let normal = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "normal",
    };
    let no_flags = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "no_flags",
    };
    let swapped_no_flags = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "swapped_no_flags",
    };
    let swapped_normal = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "swapped_normal",
    };

    let mut selected = normal;
    for candidate in [normal, no_flags, swapped_no_flags, swapped_normal] {
        if info.funcs.iter().any(|func| func.memid == candidate.memid) {
            selected = candidate;
            break;
        }
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke layout={} memid=0x{:08X} flags=0x{:04X} disp=0x{:08X}",
            selected.layout,
            selected.memid,
            selected.flags,
            selected.disp_params
        );
    }

    let instance = selected.instance;
    let memid = selected.memid;
    let flags = selected.flags;
    let disp_params = selected.disp_params;
    let result_ptr = selected.result_ptr;
    let arg_err = selected.arg_err;
    let Some(func) = info.funcs.iter().find(|func| func.memid == memid) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke func memid=0x{:08X} params={} vtable=0x{:04X} ret_vt=0x{:04X}",
            func.memid,
            func.params.len(),
            func.vtable_offset,
            func.ret_vt
        );
        eprintln!("[pe_vm] ITypeInfo::Invoke callconv=0x{:X}", func.callconv);
        let vt_list = func
            .params
            .iter()
            .map(|param| format!("0x{:04X}", param.vt))
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!("[pe_vm] ITypeInfo::Invoke param_vt=[{vt_list}]");
    }
    if flags != 0 && func.invkind != 0 && (flags & func.invkind) == 0 {
        return DISP_E_MEMBERNOTFOUND;
    }

    let mut instance = instance;
    let mut disp_params = disp_params;
    if !valid_vtable(vm, instance, func.vtable_offset)
        && valid_vtable(vm, disp_params, func.vtable_offset)
    {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] ITypeInfo::Invoke swapped instance/disp_params");
        }
        std::mem::swap(&mut instance, &mut disp_params);
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() && disp_params != 0 {
        let rgvarg = vm.read_u32(disp_params).unwrap_or(0);
        let cargs = vm.read_u32(disp_params + 8).unwrap_or(0);
        let named = vm.read_u32(disp_params + 12).unwrap_or(0);
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke disp rgvarg=0x{rgvarg:08X} cargs={cargs} named={named}"
        );
    }

    let args_ptr = vm.read_u32(disp_params).unwrap_or(0);
    let arg_count = vm.read_u32(disp_params + 8).unwrap_or(0) as usize;
    let mut input_positions = vec![None; func.params.len()];
    let mut input_count = 0usize;
    for (index, param) in func.params.iter().enumerate() {
        let flags = param.flags;
        if (flags & PARAMFLAG_FRETVAL) != 0 || is_out_only(flags) {
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
            return DISP_E_BADPARAMCOUNT;
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
        let out_only = is_out_only(flags);
        let base_vt = param.vt & !VT_BYREF;
        let record_out = is_retval
            || out_only
            || ((flags & PARAMFLAG_FOUT) != 0
                && ((param.vt & VT_BYREF) != 0
                    || base_vt == VT_USERDEFINED
                    || base_vt == VT_VARIANT
                    || base_vt == VT_BSTR));
        if positional_fallback {
            if index < arg_count {
                let arg_index = arg_count.saturating_sub(1).saturating_sub(index);
                let var_ptr = args_ptr.wrapping_add((arg_index * VARIANT_SIZE) as u32);
                let value = match read_variant_arg(vm, var_ptr, param.vt) {
                    Ok(value) => value,
                    Err(_) => {
                        if arg_err != 0 {
                            let _ = vm.write_u32(arg_err, arg_index as u32);
                        }
                        return DISP_E_TYPEMISMATCH;
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
                continue;
            }
            let value = match alloc_out_arg(vm, param.vt) {
                Ok(value) => value,
                Err(_) => return DISP_E_TYPEMISMATCH,
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
                Err(_) => return DISP_E_TYPEMISMATCH,
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
                Err(_) => return DISP_E_TYPEMISMATCH,
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
                return DISP_E_TYPEMISMATCH;
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
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
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
            eprintln!(
                "[pe_vm] ITypeInfo::Invoke bstr[{index}] ptr=0x{ptr:08X} utf16={preview}"
            );
        }
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke call instance=0x{instance:08X} vtable_off=0x{:04X}",
            func.vtable_offset
        );
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke instance_in_vm={}",
            vm.contains_addr(instance)
        );
        let instance_vtable = vm.read_u32(instance).unwrap_or(0);
        let dispatch = vm.dispatch_instance().unwrap_or(0);
        let dispatch_vtable = if dispatch == 0 {
            0
        } else {
            vm.read_u32(dispatch).unwrap_or(0)
        };
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable instance=0x{instance_vtable:08X} dispatch=0x{dispatch_vtable:08X}"
        );
    }
    let mut instance_ptr = instance;
    if let Some(dispatch) = vm.dispatch_instance() {
        if valid_vtable(vm, dispatch, func.vtable_offset) {
            instance_ptr = dispatch;
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!(
                    "[pe_vm] ITypeInfo::Invoke dispatch_instance=0x{instance_ptr:08X}"
                );
            }
        }
    }
    if !valid_vtable(vm, instance_ptr, func.vtable_offset) {
        return E_NOTIMPL;
    }
    let entry = match vtable_entry(vm, instance_ptr, func.vtable_offset) {
        Ok(value) => value,
        Err(_) => {
            let fallback = vm.reg32(crate::vm::REG_ECX);
            if fallback != 0 && fallback != instance_ptr {
                instance_ptr = fallback;
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    eprintln!(
                        "[pe_vm] ITypeInfo::Invoke fallback instance=0x{instance_ptr:08X}"
                    );
                }
                vtable_entry(vm, instance_ptr, func.vtable_offset).unwrap_or(0)
            } else {
                0
            }
        }
    };
    if entry == 0 {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] ITypeInfo::Invoke vtable lookup failed");
        }
        return E_NOTIMPL;
    }
    let thiscall_entry = detect_thiscall(vm, entry);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] ITypeInfo::Invoke entry thiscall={thiscall_entry}");
    }
    let result = if thiscall_entry {
        vm.execute_at_with_stack_with_ecx(entry, instance_ptr, &values)
    } else {
        let mut call_args = Vec::with_capacity(values.len() + 1);
        call_args.push(Value::U32(instance_ptr));
        call_args.extend(values.iter().cloned());
        vm.execute_at_with_stack(entry, &call_args)
    };
    let result = match result {
        Ok(value) => value,
        Err(err) => {
            if std::env::var("PE_VM_TRACE_COM").is_ok() {
                eprintln!("[pe_vm] ITypeInfo::Invoke call failed: {err}");
            }
            return E_NOTIMPL;
        }
    };
    trace_out_params(vm, &out_params);
    vm.set_last_com_out_params(out_params);
    let retval_value = retval_param.and_then(|(index, vt)| {
        let Some(Value::U32(ptr)) = values.get(index) else {
            return None;
        };
        read_retval_value(vm, *ptr, vt).ok()
    });
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke returned eax=0x{result:08X} ret_vt=0x{:04X}",
            func.ret_vt
        );
    }
    if result_ptr != 0 && func.ret_vt != VT_VOID && func.ret_vt != VT_EMPTY {
        let result_write = if func.ret_vt == VT_HRESULT {
            if let Some((vt, value)) = retval_value {
                write_variant_value(vm, result_ptr, vt, value)
            } else {
                write_variant_value(vm, result_ptr, VT_I4, result)
            }
        } else {
            write_variant_value(vm, result_ptr, func.ret_vt, result)
        };
        if result_write.is_err() {
            return DISP_E_TYPEMISMATCH;
        }
    }
    if arg_err != 0 {
        let _ = vm.write_u32(arg_err, 0);
    }
    if func.ret_vt == VT_HRESULT {
        return result;
    }
    S_OK
}

fn vtable_entry(vm: &Vm, instance: u32, offset: u16) -> Result<u32, VmError> {
    let vtable_ptr = vm.read_u32(instance)?;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable_ptr=0x{vtable_ptr:08X} in_vm={}",
            vm.contains_addr(vtable_ptr)
        );
    }
    if !vm.contains_addr(vtable_ptr) {
        return Err(VmError::MemoryOutOfRange);
    }
    let entry = vtable_ptr.wrapping_add(offset as u32);
    let value = vm.read_u32(entry)?;
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke vtable_entry=0x{entry:08X} fn=0x{value:08X} in_vm={}",
            vm.contains_addr(value)
        );
    }
    Ok(value)
}

fn valid_vtable(vm: &Vm, instance: u32, offset: u16) -> bool {
    let vtable_ptr = vm.read_u32(instance).unwrap_or(0);
    if vtable_ptr == 0 || !vm.contains_addr(vtable_ptr) {
        return false;
    }
    let entry = vm.read_u32(vtable_ptr.wrapping_add(offset as u32)).unwrap_or(0);
    entry != 0 && vm.contains_addr(entry)
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

fn read_variant_arg(vm: &Vm, var_ptr: u32, expected_vt: u16) -> Result<u32, VmError> {
    if var_ptr == 0 {
        return Err(VmError::InvalidConfig("variant pointer is null"));
    }
    let actual_vt = vm.read_u16(var_ptr)?;
    let value = vm.read_u32(var_ptr + 8)?;
    let expected_base = expected_vt & !VT_BYREF;
    let actual_base = actual_vt & !VT_BYREF;

    if (expected_vt & VT_BYREF) != 0 {
        if (actual_vt & VT_BYREF) == 0 {
            return Err(VmError::InvalidConfig("expected byref variant"));
        }
        return Ok(value);
    }

    if (actual_vt & VT_BYREF) != 0 {
        if value == 0 {
            return Err(VmError::InvalidConfig("null byref pointer"));
        }
        return match actual_base {
            VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED => vm.read_u32(value),
            VT_BSTR => vm.read_u32(value),
            _ => Err(VmError::InvalidConfig("unsupported byref variant")),
        };
    }

    let expected_int = matches!(
        expected_base,
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED
    );
    let actual_int = matches!(
        actual_base,
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED
    );
    if expected_int && actual_int {
        return Ok(value);
    }
    if expected_base == VT_BSTR && actual_base == VT_BSTR {
        return Ok(value);
    }
    Err(VmError::InvalidConfig("variant type mismatch"))
}

fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}

fn alloc_out_arg(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    vm.alloc_bytes(&vec![0u8; size], 4)
}

fn default_arg_for_vt(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    let base = vt & !VT_BYREF;
    if (vt & VT_BYREF) == 0 && base != VT_USERDEFINED && base != VT_VARIANT {
        return Ok(0);
    }
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    let buffer = vec![0u8; size];
    vm.alloc_bytes(&buffer, 4)
}

fn normalize_retval_vt(vt: u16) -> u16 {
    let base = vt & !VT_BYREF;
    if base == VT_USERDEFINED {
        return VT_I4;
    }
    base
}

fn read_retval_value(vm: &Vm, ptr: u32, vt: u16) -> Result<(u16, u32), VmError> {
    if ptr == 0 {
        return Err(VmError::InvalidConfig("retval pointer is null"));
    }
    let value = vm.read_u32(ptr)?;
    Ok((normalize_retval_vt(vt), value))
}

fn write_variant_value(vm: &mut Vm, dest: u32, vt: u16, value: u32) -> Result<(), VmError> {
    let base_vt = vt & !VT_BYREF;
    match base_vt {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_BSTR => write_variant_u32(vm, dest, base_vt, value),
        _ => Err(VmError::InvalidConfig("unsupported variant type")),
    }
}

fn trace_out_params(vm: &Vm, params: &[ComOutParam]) {
    if std::env::var("PE_VM_TRACE_COM").is_err() {
        return;
    }
    for param in params {
        let base = param.vt & !VT_BYREF;
        let rendered = match base {
            VT_BSTR => {
                let bstr_ptr = vm.read_u32(param.ptr).unwrap_or(0);
                match vm.read_bstr(bstr_ptr) {
                    Ok(value) => format!("bstr={value:?}"),
                    Err(_) => format!("bstr_ptr=0x{bstr_ptr:08X}"),
                }
            }
            VT_VARIANT => {
                let vt = vm.read_u16(param.ptr).unwrap_or(0);
                let value = vm.read_u32(param.ptr + 8).unwrap_or(0);
                format!("variant vt=0x{vt:04X} value=0x{value:08X}")
            }
            _ => {
                let value = vm.read_u32(param.ptr).unwrap_or(0);
                format!("value=0x{value:08X}")
            }
        };
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke out[{}] vt=0x{:04X} flags=0x{:08X} ptr=0x{:08X} {rendered}",
            param.index,
            param.vt,
            param.flags,
            param.ptr,
        );
    }
}
