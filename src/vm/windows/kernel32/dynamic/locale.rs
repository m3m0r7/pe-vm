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
    let (left_ptr, left_len, right_ptr, right_len) = vm_args!(vm, stack_ptr; u32, i32, u32, i32);
    if left_ptr == 0 || right_ptr == 0 {
        return 0;
    }
    let left = read_utf16(vm, left_ptr, left_len);
    let right = read_utf16(vm, right_ptr, right_len);
    let left = String::from_utf16_lossy(&left);
    let right = String::from_utf16_lossy(&right);
    match left.cmp(&right) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    }
}

fn get_date_format_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (time_ptr, out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:04}-{:02}-{:02}", parts.year, parts.month, parts.day);
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_locale_info_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, usize);
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, "en-US")
}

fn get_time_format_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (time_ptr, out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, u32, usize);
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:02}:{:02}:{:02}", parts.hour, parts.minute, parts.second);
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_user_default_locale_name(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (out_ptr, out_len) = vm_args!(vm, stack_ptr; u32, usize);
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, "en-US")
}

fn is_valid_locale_name(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn lc_map_string_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (flags, src_ptr, src_len, dst_ptr, dst_len) =
        vm_args!(vm, stack_ptr; u32, u32, i32, u32, usize);
    if src_ptr == 0 {
        return 0;
    }
    let mut text = String::from_utf16_lossy(&read_utf16(vm, src_ptr, src_len));
    if flags & 0x0000_0100 != 0 {
        text = text.to_ascii_lowercase();
    } else if flags & 0x0000_0200 != 0 {
        text = text.to_ascii_uppercase();
    }
    if dst_ptr == 0 || dst_len == 0 {
        return (text.len() + 1) as u32;
    }
    write_utf16(vm, dst_ptr, dst_len, &text)
}

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
    SystemTimeParts {
        year: 1970,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
    }
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
