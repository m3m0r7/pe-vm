//! Time conversion stubs.

use crate::vm::Vm;
use crate::vm_args;

const SECS_PER_DAY: f64 = 86_400.0;
const MSECS_PER_DAY: f64 = 86_400_000.0;

// SystemTimeToVariantTime(...)
pub(super) fn system_time_to_variant_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (time_ptr, out) = vm_args!(vm, stack_ptr; u32, u32);
    if time_ptr == 0 || out == 0 {
        return 0;
    }
    let year = vm.read_u16(time_ptr).unwrap_or(0) as i32;
    let month = vm.read_u16(time_ptr + 2).unwrap_or(0) as u32;
    let day = vm.read_u16(time_ptr + 6).unwrap_or(0) as u32;
    let hour = vm.read_u16(time_ptr + 8).unwrap_or(0) as u32;
    let minute = vm.read_u16(time_ptr + 10).unwrap_or(0) as u32;
    let second = vm.read_u16(time_ptr + 12).unwrap_or(0) as u32;
    let millis = vm.read_u16(time_ptr + 14).unwrap_or(0) as u32;

    if !is_valid_date(year, month, day)
        || hour > 23
        || minute > 59
        || second > 59
        || millis > 999
    {
        return 0;
    }

    let base_days = days_from_civil(1899, 12, 30);
    let days = days_from_civil(year, month, day) - base_days;
    let time_secs = (hour * 3600 + minute * 60 + second) as f64 + (millis as f64 / 1000.0);
    let variant = days as f64 + (time_secs / SECS_PER_DAY);
    let _ = vm.write_bytes(out, &variant.to_le_bytes());
    1
}

// VariantTimeToSystemTime(DOUBLE vtime, SYSTEMTIME *lpSystemTime)
// DOUBLE is 8 bytes, pointer is 4 bytes = 12 bytes total
pub(super) fn variant_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Skip 4 bytes for return address, then DOUBLE (vtime) is 8 bytes, then pointer
    if std::env::var("PE_VM_TRACE").is_ok() {
        // Dump more of the stack to understand the layout
        let mut stack_dump = String::new();
        for i in 0..8u32 {
            let val = vm.read_u32(stack_ptr + i * 4).unwrap_or(0xDEADBEEF);
            stack_dump.push_str(&format!(" +0x{:02X}=0x{val:08X}", i * 4));
        }
        let esp = vm.reg32(crate::vm::REG_ESP);
        eprintln!(
            "[pe_vm] VariantTimeToSystemTime: stack_ptr=0x{stack_ptr:08X} esp=0x{esp:08X}{stack_dump}"
        );
    }
    let out = vm.read_u32(stack_ptr + 4 + 8).unwrap_or(0);
    if out == 0 {
        return 0;
    }
    let low = vm.read_u32(stack_ptr + 4).unwrap_or(0) as u64;
    let high = vm.read_u32(stack_ptr + 8).unwrap_or(0) as u64;
    let bits = low | (high << 32);
    let value = f64::from_bits(bits);
    if !value.is_finite() {
        return 0;
    }

    let mut days = value.floor();
    let mut frac = value - days;
    if frac < 0.0 {
        days -= 1.0;
        frac += 1.0;
    }
    let mut total_ms = (frac * MSECS_PER_DAY).round();
    if total_ms >= MSECS_PER_DAY {
        total_ms -= MSECS_PER_DAY;
        days += 1.0;
    }

    let base_days = days_from_civil(1899, 12, 30);
    let absolute_days = base_days + days as i64;
    let (year, month, day) = civil_from_days(absolute_days);

    let mut remaining = total_ms as u32;
    let hour = remaining / 3_600_000;
    remaining %= 3_600_000;
    let minute = remaining / 60_000;
    remaining %= 60_000;
    let second = remaining / 1000;
    let millis = remaining % 1000;

    let dow_base = 6i64;
    let mut dow = (days as i64 + dow_base) % 7;
    if dow < 0 {
        dow += 7;
    }

    let systemtime: [u8; 16] = [
        (year as u16).to_le_bytes()[0],
        (year as u16).to_le_bytes()[1],
        (month as u16).to_le_bytes()[0],
        (month as u16).to_le_bytes()[1],
        (dow as u16).to_le_bytes()[0],
        (dow as u16).to_le_bytes()[1],
        (day as u16).to_le_bytes()[0],
        (day as u16).to_le_bytes()[1],
        (hour as u16).to_le_bytes()[0],
        (hour as u16).to_le_bytes()[1],
        (minute as u16).to_le_bytes()[0],
        (minute as u16).to_le_bytes()[1],
        (second as u16).to_le_bytes()[0],
        (second as u16).to_le_bytes()[1],
        (millis as u16).to_le_bytes()[0],
        (millis as u16).to_le_bytes()[1],
    ];
    let _ = vm.write_bytes(out, &systemtime);
    1
}

fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    if year < 1 || month < 1 || month > 12 {
        return false;
    }
    let dim = days_in_month(year, month);
    day >= 1 && day <= dim
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = month as i32;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + yoe / 400 + doy;
    (era * 146097 + doe - 719468) as i64
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let days = days + 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i32 + (era as i32) * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100 + yoe / 400);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (mp + if mp < 10 { 3 } else { -9 }) as u32;
    let year = y + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_system_time_to_variant_time_null_out() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 8, 0).unwrap(); // null output
        let result = system_time_to_variant_time(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_system_time_to_variant_time_base() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        let out_ptr = vm.heap_start as u32;
        vm.write_u32(stack + 8, out_ptr).unwrap();
        // 1899-12-30 00:00:00 -> 0.0
        let time_ptr = vm.heap_start as u32 + 0x40;
        vm.write_u32(stack + 4, time_ptr).unwrap();
        vm.write_u16(time_ptr, 1899).unwrap();
        vm.write_u16(time_ptr + 2, 12).unwrap();
        vm.write_u16(time_ptr + 6, 30).unwrap();
        let result = system_time_to_variant_time(&mut vm, stack);
        assert_eq!(result, 1);
        let low = vm.read_u32(out_ptr).unwrap() as u64;
        let high = vm.read_u32(out_ptr + 4).unwrap() as u64;
        let value = f64::from_bits(low | (high << 32));
        assert!((value - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_variant_time_to_system_time_null_out() {
        let mut vm = create_test_vm();
        // Stack layout: [ret_addr(4)] [double vtime(8)] [ptr lpSystemTime(4)]
        let stack = vm.stack_top - 16;
        // Write null to lpSystemTime at offset 12 (4 + 8)
        vm.write_u32(stack + 12, 0).unwrap();
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_variant_time_to_system_time_base() {
        let mut vm = create_test_vm();
        // Stack layout: [ret_addr(4)] [double vtime(8)] [ptr lpSystemTime(4)]
        let stack = vm.stack_top - 16;
        let out_ptr = vm.heap_start as u32;
        // vtime = 0.0 -> 1899-12-30
        let value = 0.0f64.to_bits();
        vm.write_u32(stack + 4, value as u32).unwrap();
        vm.write_u32(stack + 8, (value >> 32) as u32).unwrap();
        vm.write_u32(stack + 12, out_ptr).unwrap();
        let result = variant_time_to_system_time(&mut vm, stack);
        assert_eq!(result, 1);
        // Should write SYSTEMTIME struct (wYear=1899, wMonth=12, wDay=30)
        let year = vm.read_u16(out_ptr).unwrap();
        let month = vm.read_u16(out_ptr + 2).unwrap();
        let day = vm.read_u16(out_ptr + 6).unwrap();
        assert_eq!(year, 1899);
        assert_eq!(month, 12);
        assert_eq!(day, 30);
    }
}
