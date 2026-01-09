//! Miscellaneous User32 helpers.

use crate::define_stub_fn;
use crate::vm::windows::gdi32;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::{Value, Vm};
use crate::vm_args;

define_stub_fn!(DLL_NAME, enable_window, 1);

// Register smaller helpers that don't warrant their own module.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        DLL_NAME,
        "EnableWindow",
        crate::vm::stdcall_args(2),
        enable_window,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "CharNextA",
        crate::vm::stdcall_args(1),
        char_next_a,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "CharNextW",
        crate::vm::stdcall_args(1),
        char_next_w,
    );
    vm.register_import_stdcall(DLL_NAME, "SetTimer", crate::vm::stdcall_args(4), set_timer);
    vm.register_import_stdcall(
        DLL_NAME,
        "KillTimer",
        crate::vm::stdcall_args(2),
        kill_timer,
    );
    vm.register_import_stdcall(
        DLL_NAME,
        "LoadImageA",
        crate::vm::stdcall_args(6),
        load_image_a,
    );
}

fn char_next_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    let byte = vm.read_u8(ptr).unwrap_or(0);
    if is_shift_jis_lead(byte) {
        ptr.wrapping_add(2)
    } else {
        ptr.wrapping_add(1)
    }
}

fn char_next_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (ptr,) = vm_args!(vm, stack_ptr; u32);
    if ptr == 0 {
        return 0;
    }
    ptr.wrapping_add(2)
}

fn is_shift_jis_lead(byte: u8) -> bool {
    matches!(byte, 0x81..=0x9F | 0xE0..=0xFC)
}

fn set_timer(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (hwnd, timer_id, elapse, callback) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    let resolved_id = if timer_id == 0 { 1 } else { timer_id };

    if callback != 0 {
        let args = [
            Value::U32(hwnd),
            Value::U32(0x0113), // WM_TIMER
            Value::U32(resolved_id),
            Value::U32(elapse),
        ];
        let _ = vm.execute_at_with_stack(callback, &args);
        let _ = vm.execute_at_with_stack(callback, &args);
    }

    resolved_id
}

fn kill_timer(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn load_image_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let (hinst, name_ptr, image_type, cx, cy, _flags) =
        vm_args!(_vm, _stack_ptr; u32, u32, u32, u32, u32, u32);
    if hinst == 0 && name_ptr == 0 {
        return 0;
    }
    // Only handle bitmap-from-file for now.
    if image_type != 0 {
        return 0;
    }
    if name_ptr == 0 {
        return 0;
    }
    let path = read_wide_or_utf16le_str!(_vm, name_ptr);
    let host_path = _vm.map_path(&path);
    let data = match std::fs::read(&host_path) {
        Ok(data) => data,
        Err(_) => return 0,
    };
    let bmp = match parse_bmp(&data) {
        Some(info) => info,
        None => return 0,
    };
    let width = if cx == 0 { bmp.width } else { cx };
    let height = if cy == 0 { bmp.height } else { cy };
    let Some((handle, _)) = gdi32::create_bitmap(_vm, width, height, bmp.bit_count, Some(&bmp.bits))
    else {
        return 0;
    };
    handle
}

struct BmpInfo {
    width: u32,
    height: u32,
    bit_count: u16,
    bits: Vec<u8>,
}

fn parse_bmp(data: &[u8]) -> Option<BmpInfo> {
    if data.len() < 54 {
        return None;
    }
    if &data[0..2] != b"BM" {
        return None;
    }
    let pixel_offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
    let header_size = u32::from_le_bytes([data[14], data[15], data[16], data[17]]) as usize;
    if data.len() < 14 + header_size {
        return None;
    }
    let width_raw = i32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height_raw = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let planes = u16::from_le_bytes([data[26], data[27]]);
    let bit_count = u16::from_le_bytes([data[28], data[29]]);
    let compression = u32::from_le_bytes([data[30], data[31], data[32], data[33]]);
    if planes == 0 || compression != 0 {
        return None;
    }
    let width = width_raw.unsigned_abs();
    let height = height_raw.unsigned_abs();
    if width == 0 || height == 0 {
        return None;
    }
    let stride = ((u32::from(bit_count) * width + 31) / 32).saturating_mul(4);
    let size = stride.saturating_mul(height) as usize;
    if pixel_offset >= data.len() {
        return None;
    }
    let available = data.len().saturating_sub(pixel_offset);
    let copy_len = size.min(available);
    let mut bits = vec![0u8; size];
    bits[..copy_len].copy_from_slice(&data[pixel_offset..pixel_offset + copy_len]);
    Some(BmpInfo {
        width,
        height,
        bit_count,
        bits,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BypassSettings;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut bypass = BypassSettings::new();
        bypass.not_implemented_module = true;
        let mut vm = Vm::new(
            VmConfig::new()
                .architecture(Architecture::X86)
                .bypass(bypass),
        )
        .expect("vm");
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
    fn test_enable_window_returns_one() {
        let mut vm = create_test_vm();
        let result = enable_window(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_char_next_a_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = char_next_a(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_char_next_a_advances_by_one() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let ptr = 0x1000u32;
        vm.write_u32(stack + 4, ptr).unwrap();
        let result = char_next_a(&mut vm, stack);
        assert_eq!(result, 0x1001);
    }

    #[test]
    fn test_char_next_w_null_ptr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, 0).unwrap();
        let result = char_next_w(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_char_next_w_advances_by_two() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        let ptr = 0x1000u32;
        vm.write_u32(stack + 4, ptr).unwrap();
        let result = char_next_w(&mut vm, stack);
        assert_eq!(result, 0x1002);
    }
}
