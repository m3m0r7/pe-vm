//! Kernel32 time-related stubs.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::Vm;

const WINDOWS_TICK: u64 = 10_000_000;
const SEC_TO_UNIX_EPOCH: u64 = 11_644_473_600;

pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetSystemTimeAsFileTime",
        crate::vm::stdcall_args(1),
        get_system_time_as_filetime,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetLocalTime", crate::vm::stdcall_args(1), get_local_time);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "SystemTimeToFileTime",
        crate::vm::stdcall_args(2),
        system_time_to_filetime,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FileTimeToSystemTime",
        crate::vm::stdcall_args(2),
        file_time_to_system_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "LocalFileTimeToFileTime",
        crate::vm::stdcall_args(2),
        local_file_time_to_file_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "FileTimeToLocalFileTime",
        crate::vm::stdcall_args(2),
        file_time_to_local_file_time,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetTimeZoneInformation",
        crate::vm::stdcall_args(1),
        get_time_zone_information,
    );
    vm.register_import_stdcall("KERNEL32.dll", "GetDateFormatW", crate::vm::stdcall_args(6), get_date_format_w);
    vm.register_import_stdcall("KERNEL32.dll", "GetTimeFormatW", crate::vm::stdcall_args(6), get_time_format_w);
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "QueryPerformanceCounter",
        crate::vm::stdcall_args(1),
        query_performance_counter,
    );
}

fn get_system_time_as_filetime(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if filetime_ptr == 0 {
        return 0;
    }

    let ticks = filetime_now();
    write_filetime(vm, filetime_ptr, ticks);

    0
}

fn get_local_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if out_ptr == 0 {
        return 0;
    }
    let parts = now_parts();
    write_system_time(vm, out_ptr, &parts);
    0
}

fn system_time_to_filetime(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if time_ptr == 0 || filetime_ptr == 0 {
        return 0;
    }
    let parts = read_system_time(vm, time_ptr);
    let ticks = filetime_from_parts(&parts);
    write_filetime(vm, filetime_ptr, ticks);
    1
}

fn file_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if filetime_ptr == 0 || time_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, filetime_ptr);
    let parts = parts_from_filetime(ticks);
    write_system_time(vm, time_ptr, &parts);
    1
}

fn local_file_time_to_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let local_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if local_ptr == 0 || filetime_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, local_ptr);
    write_filetime(vm, filetime_ptr, ticks);
    1
}

fn file_time_to_local_file_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let filetime_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let local_ptr = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if filetime_ptr == 0 || local_ptr == 0 {
        return 0;
    }
    let ticks = read_filetime(vm, filetime_ptr);
    write_filetime(vm, local_ptr, ticks);
    1
}

fn get_time_zone_information(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let _ = vm.write_bytes(info_ptr, &[0u8; 172]);
    0
}

fn get_date_format_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0) as usize;
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:04}-{:02}-{:02}", parts.year, parts.month, parts.day);
    write_utf16(vm, out_ptr, out_len, &text)
}

fn get_time_format_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr.wrapping_add(12)).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr.wrapping_add(20)).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr.wrapping_add(24)).unwrap_or(0) as usize;
    let parts = if time_ptr == 0 {
        now_parts()
    } else {
        read_system_time(vm, time_ptr)
    };
    let text = format!("{:02}:{:02}:{:02}", parts.hour, parts.minute, parts.second);
    write_utf16(vm, out_ptr, out_len, &text)
}

fn query_performance_counter(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let counter_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if counter_ptr == 0 {
        return 0;
    }

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let ticks = duration.as_nanos() as u64;
    let low = ticks as u32;
    let high = (ticks >> 32) as u32;
    let _ = vm.write_u32(counter_ptr, low);
    let _ = vm.write_u32(counter_ptr.wrapping_add(4), high);

    1
}

#[derive(Clone, Copy)]
struct SystemTimeParts {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millis: u32,
    weekday: u32,
}

fn filetime_now() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (duration.as_secs() + SEC_TO_UNIX_EPOCH) * WINDOWS_TICK
        + (duration.subsec_nanos() as u64 / 100)
}

fn now_parts() -> SystemTimeParts {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    parts_from_unix_seconds(duration.as_secs() as i64, duration.subsec_millis())
}

fn parts_from_unix_seconds(seconds: i64, millis: u32) -> SystemTimeParts {
    let days = seconds.div_euclid(86_400);
    let secs_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = (secs_of_day / 3_600) as u32;
    let minute = ((secs_of_day % 3_600) / 60) as u32;
    let second = (secs_of_day % 60) as u32;
    let weekday = ((days + 4).rem_euclid(7)) as u32;
    SystemTimeParts {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millis,
        weekday,
    }
}

fn filetime_from_parts(parts: &SystemTimeParts) -> u64 {
    let unix_seconds = unix_seconds_from_parts(parts);
    let total_seconds = unix_seconds + SEC_TO_UNIX_EPOCH as i64;
    if total_seconds <= 0 {
        return 0;
    }
    (total_seconds as u64) * WINDOWS_TICK + (parts.millis as u64) * 10_000
}

fn parts_from_filetime(ticks: u64) -> SystemTimeParts {
    let total_seconds = (ticks / WINDOWS_TICK) as i64;
    let unix_seconds = total_seconds - SEC_TO_UNIX_EPOCH as i64;
    let millis = ((ticks % WINDOWS_TICK) / 10_000) as u32;
    parts_from_unix_seconds(unix_seconds, millis)
}

fn unix_seconds_from_parts(parts: &SystemTimeParts) -> i64 {
    let days = days_from_civil(parts.year, parts.month, parts.day);
    days * 86_400
        + parts.hour as i64 * 3_600
        + parts.minute as i64 * 60
        + parts.second as i64
}

fn write_filetime(vm: &mut Vm, ptr: u32, ticks: u64) {
    let low = ticks as u32;
    let high = (ticks >> 32) as u32;
    let _ = vm.write_u32(ptr, low);
    let _ = vm.write_u32(ptr.wrapping_add(4), high);
}

fn read_filetime(vm: &Vm, ptr: u32) -> u64 {
    let low = vm.read_u32(ptr).unwrap_or(0) as u64;
    let high = vm.read_u32(ptr.wrapping_add(4)).unwrap_or(0) as u64;
    (high << 32) | low
}

fn write_system_time(vm: &mut Vm, ptr: u32, parts: &SystemTimeParts) {
    let _ = vm.write_u16(ptr, parts.year as u16);
    let _ = vm.write_u16(ptr.wrapping_add(2), parts.month as u16);
    let _ = vm.write_u16(ptr.wrapping_add(4), parts.weekday as u16);
    let _ = vm.write_u16(ptr.wrapping_add(6), parts.day as u16);
    let _ = vm.write_u16(ptr.wrapping_add(8), parts.hour as u16);
    let _ = vm.write_u16(ptr.wrapping_add(10), parts.minute as u16);
    let _ = vm.write_u16(ptr.wrapping_add(12), parts.second as u16);
    let _ = vm.write_u16(ptr.wrapping_add(14), parts.millis as u16);
}

fn read_system_time(vm: &Vm, ptr: u32) -> SystemTimeParts {
    let year = vm.read_u16(ptr).unwrap_or(1970) as i32;
    let month = vm.read_u16(ptr.wrapping_add(2)).unwrap_or(1) as u32;
    let day = vm.read_u16(ptr.wrapping_add(6)).unwrap_or(1) as u32;
    let hour = vm.read_u16(ptr.wrapping_add(8)).unwrap_or(0) as u32;
    let minute = vm.read_u16(ptr.wrapping_add(10)).unwrap_or(0) as u32;
    let second = vm.read_u16(ptr.wrapping_add(12)).unwrap_or(0) as u32;
    let millis = vm.read_u16(ptr.wrapping_add(14)).unwrap_or(0) as u32;
    let days = days_from_civil(year, month, day);
    let weekday = ((days + 4).rem_euclid(7)) as u32;
    SystemTimeParts {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millis,
        weekday,
    }
}

fn write_utf16(vm: &mut Vm, ptr: u32, len: usize, text: &str) -> u32 {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let needed = utf16.len() + 1;
    if ptr == 0 || len == 0 {
        return needed as u32;
    }
    let write_len = len.saturating_sub(1).min(utf16.len());
    for (idx, unit) in utf16.iter().take(write_len).enumerate() {
        let _ = vm.write_u16(ptr + (idx as u32) * 2, *unit);
    }
    let _ = vm.write_u16(ptr + (write_len as u32) * 2, 0);
    needed as u32
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

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = month as i32 + if month > 2 { -3 } else { 9 };
    let doy = (153 * m + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146_097 + doe - 719_468) as i64
}
