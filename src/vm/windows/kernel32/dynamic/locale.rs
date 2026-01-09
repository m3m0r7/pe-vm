use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "EnumSystemLocalesEx",
        crate::vm::stdcall_args(4),
        enum_system_locales_ex,
    );
    vm.register_import_any_stdcall(
        "CompareStringEx",
        crate::vm::stdcall_args(9),
        compare_string_ex,
    );
    vm.register_import_any_stdcall(
        "GetDateFormatEx",
        crate::vm::stdcall_args(7),
        get_date_format_ex,
    );
    vm.register_import_any_stdcall(
        "GetLocaleInfoEx",
        crate::vm::stdcall_args(4),
        get_locale_info_ex,
    );
    vm.register_import_any_stdcall(
        "GetTimeFormatEx",
        crate::vm::stdcall_args(7),
        get_time_format_ex,
    );
    vm.register_import_any_stdcall(
        "GetUserDefaultLocaleName",
        crate::vm::stdcall_args(2),
        get_user_default_locale_name,
    );
    vm.register_import_any_stdcall(
        "IsValidLocaleName",
        crate::vm::stdcall_args(1),
        is_valid_locale_name,
    );
    vm.register_import_any_stdcall(
        "LCMapStringEx",
        crate::vm::stdcall_args(9),
        lc_map_string_ex,
    );
}

fn enum_system_locales_ex(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn compare_string_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale_ptr, flags, left_ptr, left_len, right_ptr, right_len, _, _, _) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, i32, u32, u32, u32);
    if left_ptr == 0 || right_ptr == 0 {
        return 0;
    }
    let mut left = String::from_utf16_lossy(&read_utf16(vm, left_ptr, left_len));
    let mut right = String::from_utf16_lossy(&read_utf16(vm, right_ptr, right_len));
    if flags & 0x0000_0001 != 0 {
        left = left.to_ascii_lowercase();
        right = right.to_ascii_lowercase();
    }
    match left.cmp(&right) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    }
}

fn get_date_format_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale_ptr, _flags, time_ptr, _format_ptr, out_ptr, out_len, _calendar_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, usize, u32);
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:04}-{:02}-{:02}", parts.year, parts.month, parts.day);
    let required = text.encode_utf16().count() + 1;
    if out_ptr == 0 || out_len == 0 {
        return required as u32;
    }
    if out_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_locale_info_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale_ptr, _lc_type, out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, u32, u32, usize);
    let value = locale_info_value(_lc_type);
    let required = value.encode_utf16().count() + 1;
    if out_ptr == 0 || out_len == 0 {
        return required as u32;
    }
    if out_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, value)
}

fn get_time_format_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale_ptr, _flags, time_ptr, _format_ptr, out_ptr, out_len, _calendar_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, usize, u32);
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:02}:{:02}:{:02}", parts.hour, parts.minute, parts.second);
    let required = text.encode_utf16().count() + 1;
    if out_ptr == 0 || out_len == 0 {
        return required as u32;
    }
    if out_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_user_default_locale_name(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, usize);
    let locale = default_locale_name();
    let required = locale.encode_utf16().count() + 1;
    if out_ptr == 0 || out_len == 0 {
        return required as u32;
    }
    if out_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, locale)
}

fn is_valid_locale_name(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn lc_map_string_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_locale_ptr, flags, src_ptr, src_len, dst_ptr, dst_len, _, _, _) =
        vm_args!(vm, stack_ptr; u32, u32, u32, i32, u32, usize, u32, u32, u32);
    if src_ptr == 0 {
        return 0;
    }
    let mut text = String::from_utf16_lossy(&read_utf16(vm, src_ptr, src_len));
    if flags & 0x0000_0100 != 0 {
        text = text.to_ascii_lowercase();
    } else if flags & 0x0000_0200 != 0 {
        text = text.to_ascii_uppercase();
    }
    if (flags & 0x0000_0400) != 0 {
        let mut bytes = Vec::new();
        for unit in text.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        bytes.push(0);
        let required = bytes.len();
        if dst_ptr == 0 || dst_len == 0 {
            return required as u32;
        }
        if dst_len < required {
            vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        let _ = vm.write_bytes(dst_ptr, &bytes);
        return required as u32;
    }
    let required = text.encode_utf16().count() + 1;
    if dst_ptr == 0 || dst_len == 0 {
        return required as u32;
    }
    if dst_len < required {
        vm.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_utf16(vm, dst_ptr, dst_len, &text)
}

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

const LOCALE_SNAME: u32 = 0x0000005C;
const LOCALE_SISO639LANGNAME: u32 = 0x00000059;
const LOCALE_SISO3166CTRYNAME: u32 = 0x0000005A;
const LOCALE_IDEFAULTANSICODEPAGE: u32 = 0x00001004;
const LOCALE_IDEFAULTCODEPAGE: u32 = 0x0000000B;

#[derive(Clone, Copy)]
struct SystemTimeParts {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

fn now_parts() -> SystemTimeParts {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    parts_from_unix_seconds(duration.as_secs() as i64)
}

fn read_system_time(vm: &Vm, addr: u32) -> SystemTimeParts {
    let year = vm.read_u16(addr).unwrap_or(0) as i32;
    let month = vm.read_u16(addr + 2).unwrap_or(0) as u32;
    let day = vm.read_u16(addr + 6).unwrap_or(0) as u32;
    let hour = vm.read_u16(addr + 8).unwrap_or(0) as u32;
    let minute = vm.read_u16(addr + 10).unwrap_or(0) as u32;
    let second = vm.read_u16(addr + 12).unwrap_or(0) as u32;
    SystemTimeParts {
        year,
        month,
        day,
        hour,
        minute,
        second,
    }
}

fn read_utf16(vm: &Vm, addr: u32, len: i32) -> Vec<u16> {
    let mut out = Vec::new();
    let mut idx = 0u32;
    while len < 0 || (idx as i32) < len {
        if let Ok(value) = vm.read_u16(addr + idx * 2) {
            if value == 0 {
                break;
            }
            out.push(value);
        } else {
            break;
        }
        idx += 1;
    }
    out
}

fn write_utf16(vm: &mut Vm, addr: u32, max_len: usize, text: &str) -> u32 {
    let mut utf16: Vec<u16> = text.encode_utf16().collect();
    if utf16.len() >= max_len {
        utf16.truncate(max_len.saturating_sub(1));
    }
    for (i, unit) in utf16.iter().enumerate() {
        let _ = vm.write_u16(addr + (i as u32) * 2, *unit);
    }
    let _ = vm.write_u16(addr + (utf16.len() as u32) * 2, 0);
    (utf16.len() + 1) as u32
}

fn default_locale_name() -> &'static str {
    "ja-JP"
}

fn locale_info_value(lc_type: u32) -> &'static str {
    match lc_type {
        LOCALE_SNAME | LOCALE_SISO639LANGNAME | LOCALE_SISO3166CTRYNAME => default_locale_name(),
        LOCALE_IDEFAULTANSICODEPAGE | LOCALE_IDEFAULTCODEPAGE => "932",
        _ => default_locale_name(),
    }
}

fn parts_from_unix_seconds(seconds: i64) -> SystemTimeParts {
    let days = seconds.div_euclid(86_400);
    let secs_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = (secs_of_day / 3_600) as u32;
    let minute = ((secs_of_day % 3_600) / 60) as u32;
    let second = (secs_of_day % 60) as u32;
    SystemTimeParts {
        year,
        month,
        day,
        hour,
        minute,
        second,
    }
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (mp + if mp < 10 { 3 } else { -9 }) as i32;
    let year = y + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day)
}
