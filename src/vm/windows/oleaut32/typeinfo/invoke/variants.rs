use crate::vm::{ComOutParam, Vm, VmError};

use super::super::super::constants::{
    PARAMFLAG_FIN, PARAMFLAG_FOUT, VARIANT_SIZE, VT_ARRAY, VT_BSTR, VT_BYREF, VT_INT, VT_I4,
    VT_NULL, VT_UI1, VT_UI4, VT_UINT, VT_USERDEFINED, VT_VARIANT,
};
use super::super::super::variant::write_variant_u32;

pub(super) fn read_variant_arg(
    vm: &Vm,
    var_ptr: u32,
    expected_vt: u16,
) -> Result<u32, VmError> {
    if var_ptr == 0 {
        return Err(VmError::InvalidConfig("variant pointer is null"));
    }
    let actual_vt = vm.read_u16(var_ptr)?;
    let value = vm.read_u32(var_ptr + 8)?;
    let expected_base = expected_vt & !(VT_BYREF | VT_ARRAY);
    let actual_base = actual_vt & !(VT_BYREF | VT_ARRAY);
    let expected_array = (expected_vt & VT_ARRAY) != 0;
    let actual_array = (actual_vt & VT_ARRAY) != 0;

    if expected_array {
        if !actual_array {
            return Err(VmError::InvalidConfig("expected array variant"));
        }
        return Ok(value);
    }

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
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED | VT_NULL
    );
    let actual_int = matches!(
        actual_base,
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_USERDEFINED | VT_NULL
    );
    if expected_int && actual_int {
        return Ok(value);
    }
    if expected_base == VT_BSTR && actual_base == VT_BSTR {
        return Ok(value);
    }
    Err(VmError::InvalidConfig("variant type mismatch"))
}

pub(super) fn is_out_only(flags: u32) -> bool {
    (flags & PARAMFLAG_FOUT) != 0 && (flags & PARAMFLAG_FIN) == 0
}

pub(super) fn alloc_out_arg(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    if (vt & VT_ARRAY) != 0 {
        return vm.alloc_bytes(&vec![0u8; 4], 4);
    }
    let base = vt & !(VT_BYREF | VT_ARRAY);
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_UI1 | VT_BSTR | VT_NULL => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    vm.alloc_bytes(&vec![0u8; size], 4)
}

pub(super) fn default_arg_for_vt(vm: &mut Vm, vt: u16) -> Result<u32, VmError> {
    if (vt & VT_ARRAY) != 0 {
        return vm.alloc_bytes(&vec![0u8; 4], 4);
    }
    let base = vt & !(VT_BYREF | VT_ARRAY);
    if (vt & VT_BYREF) == 0 && base != VT_USERDEFINED && base != VT_VARIANT {
        return Ok(0);
    }
    let size = match base {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_UI1 | VT_BSTR | VT_NULL => 4,
        VT_VARIANT | VT_USERDEFINED => VARIANT_SIZE,
        _ => 4,
    };
    let buffer = vec![0u8; size];
    vm.alloc_bytes(&buffer, 4)
}

fn normalize_retval_vt(vt: u16) -> u16 {
    let base = vt & !VT_BYREF;
    if base == VT_USERDEFINED || base == VT_NULL {
        return VT_I4;
    }
    base
}

pub(super) fn read_retval_value(vm: &Vm, ptr: u32, vt: u16) -> Result<(u16, u32), VmError> {
    if ptr == 0 {
        return Err(VmError::InvalidConfig("retval pointer is null"));
    }
    let value = vm.read_u32(ptr)?;
    Ok((normalize_retval_vt(vt), value))
}

pub(super) fn write_variant_value(
    vm: &mut Vm,
    dest: u32,
    vt: u16,
    value: u32,
) -> Result<(), VmError> {
    if (vt & VT_ARRAY) != 0 {
        vm.write_u16(dest, vt)?;
        vm.write_u16(dest + 2, 0)?;
        vm.write_u16(dest + 4, 0)?;
        vm.write_u16(dest + 6, 0)?;
        vm.write_u32(dest + 8, value)?;
        vm.write_u32(dest + 12, 0)?;
        return Ok(());
    }
    let base_vt = vt & !VT_BYREF;
    match base_vt {
        VT_I4 | VT_UI4 | VT_INT | VT_UINT | VT_UI1 | VT_BSTR | VT_NULL => {
            write_variant_u32(vm, dest, base_vt, value)
        }
        _ => Err(VmError::InvalidConfig("unsupported variant type")),
    }
}

pub(super) fn trace_out_params(vm: &Vm, params: &[ComOutParam]) {
    if std::env::var("PE_VM_TRACE_COM").is_err() {
        return;
    }
    for param in params {
        let base = param.vt & !VT_BYREF;
        let rendered = if (param.vt & VT_ARRAY) != 0 {
            let array_ptr = if (param.vt & VT_BYREF) != 0 {
                vm.read_u32(param.ptr).unwrap_or(0)
            } else {
                param.ptr
            };
            format!("safearray=0x{array_ptr:08X}")
        } else {
            match base {
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
