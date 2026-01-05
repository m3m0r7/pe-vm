//! COM argument definitions for the C ABI.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::vm::windows;

#[repr(C)]
pub struct PevmComArg {
    pub tag: u32,
    pub value: PevmComArgValue,
}

#[repr(C)]
pub union PevmComArgValue {
    pub i4: i32,
    pub u32_value: u32,
    pub bstr: *const c_char,
}

pub(crate) const COM_ARG_I4: u32 = 0;
pub(crate) const COM_ARG_U32: u32 = 1;
pub(crate) const COM_ARG_BSTR: u32 = 2;

pub(crate) unsafe fn parse_com_args(
    args: *const PevmComArg,
    args_len: usize,
) -> Result<Vec<windows::com::ComArg>, String> {
    if args_len == 0 {
        return Ok(Vec::new());
    }
    if args.is_null() {
        return Err("args is null".to_string());
    }
    let slice = std::slice::from_raw_parts(args, args_len);
    let mut out = Vec::with_capacity(slice.len());
    for (index, arg) in slice.iter().enumerate() {
        match arg.tag {
            COM_ARG_I4 => {
                let value = unsafe { arg.value.i4 };
                out.push(windows::com::ComArg::I4(value));
            }
            COM_ARG_U32 => {
                let value = unsafe { arg.value.u32_value };
                out.push(windows::com::ComArg::U32(value));
            }
            COM_ARG_BSTR => {
                let ptr = unsafe { arg.value.bstr };
                if ptr.is_null() {
                    return Err(format!("arg {index} bstr is null"));
                }
                let text = CStr::from_ptr(ptr)
                    .to_str()
                    .map_err(|_| format!("arg {index} bstr is not valid UTF-8"))?;
                out.push(windows::com::ComArg::BStr(text.to_string()));
            }
            _ => return Err(format!("arg {index} has unknown tag")),
        }
    }
    Ok(out)
}
