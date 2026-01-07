//! ITypeInfo::Invoke implementation.

mod args;
mod dispatch;
mod layout;
mod variants;

use crate::vm::windows::oleaut32::typelib;
use crate::vm::{Value, Vm};

use super::super::constants::{
    DISP_E_MEMBERNOTFOUND, DISP_E_TYPEMISMATCH, E_NOTIMPL, PARAMFLAG_FRETVAL, S_OK, VT_EMPTY,
    VT_HRESULT, VT_I4, VT_VOID,
};
use super::helpers::resolve_typeinfo_info;

use args::{build_invoke_values, trace_values};
use dispatch::{detect_thiscall, valid_vtable, vtable_entry};
use layout::{read_stack_slots, select_invoke_args};
use variants::{read_retval_value, trace_out_params, write_variant_value};

pub(super) fn typeinfo_invoke(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let Some((_this, info_id, thiscall)) = resolve_typeinfo_info(vm, stack_ptr) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    let Some(info) = typelib::get_typeinfo(info_id) else {
        return DISP_E_MEMBERNOTFOUND;
    };
    vm.set_last_com_out_params(Vec::new());
    let slots = read_stack_slots(vm, stack_ptr);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        let mut line = format!("[pe_vm] ITypeInfo::Invoke stack thiscall={thiscall}");
        for (idx, value) in slots.iter().enumerate() {
            line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
        }
        let ecx = vm.reg32(crate::vm::REG_ECX);
        line.push_str(&format!(" ecx=0x{ecx:08X}"));
        eprintln!("{line}");
    }

    let selected = select_invoke_args(&info, &slots, thiscall);
    let instance = selected.instance;
    let memid = selected.memid;
    let flags = selected.flags;
    let disp_params = selected.disp_params;
    let result_ptr = selected.result_ptr;
    let arg_err = selected.arg_err;
    let mut arg_count = if disp_params == 0 {
        0usize
    } else {
        vm.read_u32(disp_params + 8).unwrap_or(0) as usize
    };
    let max_expected = info
        .funcs
        .iter()
        .filter(|func| func.memid == memid)
        .map(expected_inputs)
        .max()
        .unwrap_or(0);
    if arg_count > max_expected {
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            eprintln!("[pe_vm] ITypeInfo::Invoke cargs clamped from {arg_count} to {max_expected}");
        }
        arg_count = max_expected;
    }
    let Some(func) = select_invoke_func(&info, memid, flags, arg_count) else {
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

    let invoke_values = match build_invoke_values(vm, func, disp_params, arg_err) {
        Ok(value) => value,
        Err(code) => return code,
    };
    trace_values(vm, func, &invoke_values.values);

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke call instance=0x{instance:08X} vtable_off=0x{:04X}",
            func.vtable_offset
        );
        let instance_vtable = vm.read_u32(instance).unwrap_or(0);
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke instance_candidate=0x{instance:08X} vtable=0x{instance_vtable:08X} valid={}",
            valid_vtable(vm, instance, func.vtable_offset)
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
    if !valid_vtable(vm, instance_ptr, func.vtable_offset) {
        if let Some(dispatch) = vm.dispatch_instance() {
            if valid_vtable(vm, dispatch, func.vtable_offset) {
                instance_ptr = dispatch;
                if std::env::var("PE_VM_TRACE_COM").is_ok() {
                    eprintln!("[pe_vm] ITypeInfo::Invoke dispatch_instance=0x{instance_ptr:08X}");
                }
            }
        }
    }
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] ITypeInfo::Invoke instance_ptr=0x{instance_ptr:08X}");
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
                    eprintln!("[pe_vm] ITypeInfo::Invoke fallback instance=0x{instance_ptr:08X}");
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
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] ITypeInfo::Invoke entry=0x{entry:08X}");
    }
    let thiscall_entry = detect_thiscall(vm, entry);
    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!("[pe_vm] ITypeInfo::Invoke entry thiscall={thiscall_entry}");
        let mut bytes = Vec::new();
        for i in 0..64u32 {
            bytes.push(vm.read_u8(entry.wrapping_add(i)).unwrap_or(0));
        }
        let preview = bytes
            .iter()
            .map(|byte| format!("{byte:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[pe_vm] ITypeInfo::Invoke entry bytes: {preview}");
    }
    let result = if thiscall_entry {
        vm.execute_at_with_stack_with_ecx(entry, instance_ptr, &invoke_values.values)
    } else {
        let mut call_args = Vec::with_capacity(invoke_values.values.len() + 1);
        call_args.push(Value::U32(instance_ptr));
        call_args.extend(invoke_values.values.iter().cloned());
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
    trace_out_params(vm, &invoke_values.out_params);
    vm.set_last_com_out_params(invoke_values.out_params);
    let retval_value = invoke_values.retval_param.and_then(|(index, vt)| {
        let Some(Value::U32(ptr)) = invoke_values.values.get(index) else {
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

fn select_invoke_func(
    info: &typelib::TypeInfoData,
    memid: u32,
    flags: u16,
    arg_count: usize,
) -> Option<&typelib::FuncDesc> {
    let candidates: Vec<&typelib::FuncDesc> = info
        .funcs
        .iter()
        .filter(|func| func.memid == memid)
        .collect();
    if candidates.is_empty() {
        return None;
    }
    if flags != 0 {
        if let Some(best) = select_best_func(
            candidates
                .iter()
                .copied()
                .filter(|func| func.invkind == 0 || (flags & func.invkind) != 0),
            arg_count,
        ) {
            return Some(best);
        }
    }
    select_best_func(candidates.iter().copied(), arg_count)
}

fn select_best_func<'a, I>(funcs: I, arg_count: usize) -> Option<&'a typelib::FuncDesc>
where
    I: Iterator<Item = &'a typelib::FuncDesc>,
{
    let mut best: Option<&'a typelib::FuncDesc> = None;
    let mut best_diff = usize::MAX;
    let mut best_total = 0usize;
    for func in funcs {
        let expected_inputs = expected_inputs(func);
        if arg_count > expected_inputs {
            continue;
        }
        let diff = expected_inputs.saturating_sub(arg_count);
        let total = func.params.len();
        if diff < best_diff || (diff == best_diff && total > best_total) {
            best = Some(func);
            best_diff = diff;
            best_total = total;
        }
    }
    best
}

fn expected_inputs(func: &typelib::FuncDesc) -> usize {
    // Count parameters that could be inputs.
    // FRETVAL params are normally not inputs, but some TypeLibs incorrectly mark input params
    // with FRETVAL. If a method already has a return type (not VOID/HRESULT), treat all
    // non-FRETVAL params and single FRETVAL params as potential inputs.
    let has_explicit_return = func.ret_vt != VT_VOID && func.ret_vt != VT_EMPTY;
    let only_fretval = func.params.len() == 1
        && func.params.iter().all(|p| (p.flags & PARAMFLAG_FRETVAL) != 0);

    let count = func.params
        .iter()
        .filter(|param| {
            let is_fretval = (param.flags & PARAMFLAG_FRETVAL) != 0;
            // If it's not FRETVAL, count it as input
            if !is_fretval {
                return true;
            }
            // If there's an explicit return type and only one FRETVAL param,
            // the FRETVAL might be incorrectly applied - count it as input
            if has_explicit_return && only_fretval {
                return true;
            }
            false
        })
        .count();
    count
}
