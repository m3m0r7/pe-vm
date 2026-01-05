//! Kernel32 dynamic GetProcAddress stubs.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::{Vm, REG_EDX};

const APPMODEL_ERROR_NO_PACKAGE: u32 = 15_700;

pub fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall("FlsAlloc", crate::vm::stdcall_args(1), fls_alloc);
    vm.register_import_any_stdcall("FlsFree", crate::vm::stdcall_args(1), fls_free);
    vm.register_import_any_stdcall("FlsGetValue", crate::vm::stdcall_args(1), fls_get_value);
    vm.register_import_any_stdcall("FlsSetValue", crate::vm::stdcall_args(2), fls_set_value);
    vm.register_import_any_stdcall("InitializeCriticalSectionEx", crate::vm::stdcall_args(3), initialize_critical_section_ex);
    vm.register_import_any_stdcall("CreateEventExW", crate::vm::stdcall_args(4), create_event_ex_w);
    vm.register_import_any_stdcall("CreateSemaphoreExW", crate::vm::stdcall_args(6), create_semaphore_ex_w);
    vm.register_import_any_stdcall("SetThreadStackGuarantee", crate::vm::stdcall_args(1), set_thread_stack_guarantee);
    vm.register_import_any_stdcall("CreateThreadpoolTimer", crate::vm::stdcall_args(3), create_threadpool_timer);
    vm.register_import_any_stdcall("SetThreadpoolTimer", crate::vm::stdcall_args(4), set_threadpool_timer);
    vm.register_import_any_stdcall(
        "WaitForThreadpoolTimerCallbacks",
        crate::vm::stdcall_args(2),
        wait_for_threadpool_timer_callbacks,
    );
    vm.register_import_any_stdcall("CloseThreadpoolTimer", crate::vm::stdcall_args(1), close_threadpool_timer);
    vm.register_import_any_stdcall("CreateThreadpoolWait", crate::vm::stdcall_args(3), create_threadpool_wait);
    vm.register_import_any_stdcall("SetThreadpoolWait", crate::vm::stdcall_args(3), set_threadpool_wait);
    vm.register_import_any_stdcall("CloseThreadpoolWait", crate::vm::stdcall_args(1), close_threadpool_wait);
    vm.register_import_any_stdcall("FlushProcessWriteBuffers", crate::vm::stdcall_args(0), flush_process_write_buffers);
    vm.register_import_any_stdcall(
        "FreeLibraryWhenCallbackReturns",
        crate::vm::stdcall_args(2),
        free_library_when_callback_returns,
    );
    vm.register_import_any_stdcall("GetCurrentProcessorNumber", crate::vm::stdcall_args(0), get_current_processor_number);
    vm.register_import_any_stdcall(
        "GetLogicalProcessorInformation",
        crate::vm::stdcall_args(2),
        get_logical_processor_information,
    );
    vm.register_import_any_stdcall("CreateSymbolicLinkW", crate::vm::stdcall_args(3), create_symbolic_link_w);
    vm.register_import_any_stdcall("SetDefaultDllDirectories", crate::vm::stdcall_args(1), set_default_dll_directories);
    vm.register_import_any_stdcall("EnumSystemLocalesEx", crate::vm::stdcall_args(4), enum_system_locales_ex);
    vm.register_import_any_stdcall("CompareStringEx", crate::vm::stdcall_args(9), compare_string_ex);
    vm.register_import_any_stdcall("GetDateFormatEx", crate::vm::stdcall_args(7), get_date_format_ex);
    vm.register_import_any_stdcall("GetLocaleInfoEx", crate::vm::stdcall_args(4), get_locale_info_ex);
    vm.register_import_any_stdcall("GetTimeFormatEx", crate::vm::stdcall_args(7), get_time_format_ex);
    vm.register_import_any_stdcall("GetUserDefaultLocaleName", crate::vm::stdcall_args(2), get_user_default_locale_name);
    vm.register_import_any_stdcall("IsValidLocaleName", crate::vm::stdcall_args(1), is_valid_locale_name);
    vm.register_import_any_stdcall("LCMapStringEx", crate::vm::stdcall_args(9), lc_map_string_ex);
    vm.register_import_any_stdcall("GetCurrentPackageId", crate::vm::stdcall_args(2), get_current_package_id);
    vm.register_import_any_stdcall("GetTickCount64", crate::vm::stdcall_args(0), get_tick_count64);
    vm.register_import_any_stdcall("GetVersionExA", crate::vm::stdcall_args(1), get_version_ex_a);
    vm.register_import_any_stdcall("GetVersionExW", crate::vm::stdcall_args(1), get_version_ex_w);
    vm.register_import_any_stdcall(
        "RtlGetNtVersionNumbers",
        crate::vm::stdcall_args(3),
        rtl_get_nt_version_numbers,
    );
    vm.register_import_any_stdcall(
        "GetFileInformationByHandleExW",
        crate::vm::stdcall_args(4),
        get_file_information_by_handle_ex_w,
    );
    vm.register_import_any_stdcall(
        "SetFileInformationByHandleW",
        crate::vm::stdcall_args(4),
        set_file_information_by_handle_w,
    );
}

fn fls_alloc(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.tls_alloc()
}

fn fls_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if vm.tls_free(index) { 1 } else { 0 }
}

fn fls_get_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    vm.tls_get(index)
}

fn fls_set_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let index = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let value = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if vm.tls_set(index, value) { 1 } else { 0 }
}

fn initialize_critical_section_ex(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_event_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_semaphore_ex_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_thread_stack_guarantee(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn create_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn wait_for_threadpool_timer_callbacks(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn close_threadpool_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn create_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn close_threadpool_wait(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn flush_process_write_buffers(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn free_library_when_callback_returns(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_current_processor_number(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_logical_processor_information(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let len_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if len_ptr != 0 {
        let _ = vm.write_u32(len_ptr, 0);
    }
    if buffer_ptr == 0 {
        return 0;
    }
    0
}

fn create_symbolic_link_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn set_default_dll_directories(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn enum_system_locales_ex(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn compare_string_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let left_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let left_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let right_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let right_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as i32;
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
    let time_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
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
    let out_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, "en-US")
}

fn get_time_format_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let time_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let out_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
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
    let out_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let out_len = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    if out_ptr == 0 || out_len == 0 {
        return 0;
    }
    write_utf16(vm, out_ptr, out_len, "en-US")
}

fn is_valid_locale_name(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn lc_map_string_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let flags = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let src_len = vm.read_u32(stack_ptr + 16).unwrap_or(0) as i32;
    let dst_ptr = vm.read_u32(stack_ptr + 20).unwrap_or(0);
    let dst_len = vm.read_u32(stack_ptr + 24).unwrap_or(0) as usize;
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

fn get_current_package_id(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let len_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if len_ptr != 0 {
        let _ = vm.write_u32(len_ptr, 0);
    }
    APPMODEL_ERROR_NO_PACKAGE
}

fn get_tick_count64(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let millis = duration.as_millis() as u64;
    let low = millis as u32;
    let high = (millis >> 32) as u32;
    vm.set_reg32(REG_EDX, high);
    low
}

// Provide a stable OS version for version checks inside DLLs.
fn get_version_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let size = vm.read_u32(info_ptr).unwrap_or(0) as usize;
    write_os_version_a(vm, info_ptr, size);
    1
}

fn get_version_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let size = vm.read_u32(info_ptr).unwrap_or(0) as usize;
    write_os_version_w(vm, info_ptr, size);
    1
}

fn rtl_get_nt_version_numbers(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let major_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let minor_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let build_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if major_ptr != 0 {
        let _ = vm.write_u32(major_ptr, 10);
    }
    if minor_ptr != 0 {
        let _ = vm.write_u32(minor_ptr, 0);
    }
    if build_ptr != 0 {
        let _ = vm.write_u32(build_ptr, 19045);
    }
    0
}

fn write_os_version_a(vm: &mut Vm, base: u32, size: usize) {
    if size < 20 {
        return;
    }
    let _ = vm.write_u32(base + 4, 10);
    let _ = vm.write_u32(base + 8, 0);
    let _ = vm.write_u32(base + 12, 19045);
    let _ = vm.write_u32(base + 16, 2);
    if size >= 20 + 128 {
        for idx in 0..128 {
            let _ = vm.write_u8(base + 20 + idx as u32, 0);
        }
    }
    if size >= 156 {
        let _ = vm.write_u16(base + 148, 0);
        let _ = vm.write_u16(base + 150, 0);
        let _ = vm.write_u16(base + 152, 0);
        let _ = vm.write_u8(base + 154, 1);
        let _ = vm.write_u8(base + 155, 0);
    }
}

fn write_os_version_w(vm: &mut Vm, base: u32, size: usize) {
    if size < 20 {
        return;
    }
    let _ = vm.write_u32(base + 4, 10);
    let _ = vm.write_u32(base + 8, 0);
    let _ = vm.write_u32(base + 12, 19045);
    let _ = vm.write_u32(base + 16, 2);
    if size >= 20 + 256 {
        for idx in 0..128 {
            let _ = vm.write_u16(base + 20 + (idx as u32) * 2, 0);
        }
    }
    if size >= 284 {
        let _ = vm.write_u16(base + 276, 0);
        let _ = vm.write_u16(base + 278, 0);
        let _ = vm.write_u16(base + 280, 0);
        let _ = vm.write_u8(base + 282, 1);
        let _ = vm.write_u8(base + 283, 0);
    }
}

fn get_file_information_by_handle_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 16).unwrap_or(0) as usize;
    if info_ptr == 0 || size == 0 {
        return 0;
    }
    let _ = vm.write_bytes(info_ptr, &vec![0u8; size]);
    1
}

fn set_file_information_by_handle_w(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
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
