//! GDI32 stub implementations for UI-heavy DLLs.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::vm::Vm;

const HANDLE_BASE: u32 = 0x7300_0000;

#[derive(Default)]
struct GdiStore {
    next_handle: u32,
    handles: HashSet<u32>,
}

fn store() -> &'static Mutex<GdiStore> {
    static STORE: OnceLock<Mutex<GdiStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        Mutex::new(GdiStore {
            next_handle: HANDLE_BASE,
            handles: HashSet::new(),
        })
    })
}

fn alloc_handle() -> u32 {
    let mut guard = store().lock().expect("gdi32 store");
    if guard.next_handle == 0 {
        guard.next_handle = HANDLE_BASE;
    }
    let handle = guard.next_handle;
    guard.next_handle = guard.next_handle.wrapping_add(1);
    guard.handles.insert(handle);
    handle
}

fn free_handle(handle: u32) -> bool {
    let mut guard = store().lock().expect("gdi32 store");
    guard.handles.remove(&handle)
}

// Register minimal GDI32 entry points needed by basic GUI flows.
pub fn register(vm: &mut Vm) {
    vm.register_import_stdcall("GDI32.dll", "SaveDC", crate::vm::stdcall_args(1), save_dc);
    vm.register_import_stdcall("GDI32.dll", "RestoreDC", crate::vm::stdcall_args(2), restore_dc);
    vm.register_import_stdcall("GDI32.dll", "SelectObject", crate::vm::stdcall_args(2), select_object);
    vm.register_import_stdcall("GDI32.dll", "SetBkColor", crate::vm::stdcall_args(2), set_bk_color);
    vm.register_import_stdcall("GDI32.dll", "SetTextColor", crate::vm::stdcall_args(2), set_text_color);
    vm.register_import_stdcall("GDI32.dll", "SetTextAlign", crate::vm::stdcall_args(2), set_text_align);
    vm.register_import_stdcall("GDI32.dll", "SetMapMode", crate::vm::stdcall_args(2), set_map_mode);
    vm.register_import_stdcall("GDI32.dll", "SetWindowOrgEx", crate::vm::stdcall_args(4), set_window_org_ex);
    vm.register_import_stdcall("GDI32.dll", "SetWindowExtEx", crate::vm::stdcall_args(4), set_window_ext_ex);
    vm.register_import_stdcall("GDI32.dll", "SetViewportOrgEx", crate::vm::stdcall_args(4), set_viewport_org_ex);
    vm.register_import_stdcall("GDI32.dll", "LPtoDP", crate::vm::stdcall_args(3), lp_to_dp);
    vm.register_import_stdcall("GDI32.dll", "TextOutA", crate::vm::stdcall_args(5), text_out_a);
    vm.register_import_stdcall("GDI32.dll", "GetObjectA", crate::vm::stdcall_args(3), get_object_a);
    vm.register_import_stdcall("GDI32.dll", "BitBlt", crate::vm::stdcall_args(9), bit_blt);
    vm.register_import_stdcall("GDI32.dll", "CloseMetaFile", crate::vm::stdcall_args(1), close_meta_file);
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateCompatibleBitmap",
        crate::vm::stdcall_args(3),
        create_compatible_bitmap,
    );
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateCompatibleDC",
        crate::vm::stdcall_args(1),
        create_compatible_dc,
    );
    vm.register_import_stdcall("GDI32.dll", "CreateDCA", crate::vm::stdcall_args(4), create_dca);
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateFontIndirectA",
        crate::vm::stdcall_args(1),
        create_font_indirect_a,
    );
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateMetaFileA",
        crate::vm::stdcall_args(1),
        create_meta_file_a,
    );
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateRectRgnIndirect",
        crate::vm::stdcall_args(1),
        create_rect_rgn_indirect,
    );
    vm.register_import_stdcall(
        "GDI32.dll",
        "CreateSolidBrush",
        crate::vm::stdcall_args(1),
        create_solid_brush,
    );
    vm.register_import_stdcall("GDI32.dll", "DeleteDC", crate::vm::stdcall_args(1), delete_dc);
    vm.register_import_stdcall(
        "GDI32.dll",
        "DeleteMetaFile",
        crate::vm::stdcall_args(1),
        delete_meta_file,
    );
    vm.register_import_stdcall("GDI32.dll", "DeleteObject", crate::vm::stdcall_args(1), delete_object);
    vm.register_import_stdcall("GDI32.dll", "GetDeviceCaps", crate::vm::stdcall_args(2), get_device_caps);
    vm.register_import_stdcall("GDI32.dll", "GetStockObject", crate::vm::stdcall_args(1), get_stock_object);
    vm.register_import_stdcall("GDI32.dll", "Rectangle", crate::vm::stdcall_args(5), rectangle);
}

fn save_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn restore_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn select_object(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let object = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if object == 0 {
        return 0;
    }
    object
}

fn set_bk_color(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 8).unwrap_or(0)
}

fn set_text_color(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 8).unwrap_or(0)
}

fn set_text_align(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 8).unwrap_or(0)
}

fn set_map_mode(vm: &mut Vm, stack_ptr: u32) -> u32 {
    vm.read_u32(stack_ptr + 8).unwrap_or(0)
}

fn set_window_org_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    write_point(vm, out);
    1
}

fn set_window_ext_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    write_point(vm, out);
    1
}

fn set_viewport_org_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 16).unwrap_or(0);
    write_point(vm, out);
    1
}

fn lp_to_dp(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn text_out_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn get_object_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let buffer = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    let size = vm.read_u32(stack_ptr + 8).unwrap_or(0) as usize;
    if buffer != 0 && size > 0 {
        let _ = vm.memset(buffer, 0, size);
    }
    0
}

fn bit_blt(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn close_meta_file(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_compatible_bitmap(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_compatible_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_dca(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_font_indirect_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_meta_file_a(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_rect_rgn_indirect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn create_solid_brush(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn delete_dc(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    free_handle(handle) as u32
}

fn delete_meta_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    free_handle(handle) as u32
}

fn delete_object(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let handle = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    free_handle(handle) as u32
}

fn get_device_caps(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn get_stock_object(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    alloc_handle()
}

fn rectangle(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}

fn write_point(vm: &mut Vm, ptr: u32) {
    if ptr == 0 {
        return;
    }
    let _ = vm.write_u32(ptr, 0);
    let _ = vm.write_u32(ptr + 4, 0);
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
    fn test_alloc_handle_returns_unique_handles() {
        let h1 = alloc_handle();
        let h2 = alloc_handle();
        let h3 = alloc_handle();
        assert_ne!(h1, h2);
        assert_ne!(h2, h3);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_free_handle_success() {
        let handle = alloc_handle();
        assert!(free_handle(handle));
    }

    #[test]
    fn test_free_handle_unknown() {
        assert!(!free_handle(0xDEADBEEF));
    }

    #[test]
    fn test_save_dc_returns_one() {
        let mut vm = create_test_vm();
        let result = save_dc(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_restore_dc_returns_one() {
        let mut vm = create_test_vm();
        let result = restore_dc(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_select_object_null_returns_zero() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 8, 0).unwrap(); // null object
        let result = select_object(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_select_object_returns_object() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm.write_u32(stack + 8, 0x1234).unwrap();
        let result = select_object(&mut vm, stack);
        assert_eq!(result, 0x1234);
    }

    #[test]
    fn test_create_compatible_dc_returns_handle() {
        let mut vm = create_test_vm();
        let result = create_compatible_dc(&mut vm, 0);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_create_solid_brush_returns_handle() {
        let mut vm = create_test_vm();
        let result = create_solid_brush(&mut vm, 0);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_delete_dc_frees_handle() {
        let mut vm = create_test_vm();
        let handle = create_compatible_dc(&mut vm, 0);
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, handle).unwrap();
        let result = delete_dc(&mut vm, stack);
        assert_eq!(result, 1); // true
    }

    #[test]
    fn test_delete_object_frees_handle() {
        let mut vm = create_test_vm();
        let handle = create_solid_brush(&mut vm, 0);
        let stack = vm.stack_top - 8;
        vm.write_u32(stack + 4, handle).unwrap();
        let result = delete_object(&mut vm, stack);
        assert_eq!(result, 1); // true
    }

    #[test]
    fn test_get_device_caps_returns_zero() {
        let mut vm = create_test_vm();
        let result = get_device_caps(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_stock_object_returns_handle() {
        let mut vm = create_test_vm();
        let result = get_stock_object(&mut vm, 0);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_text_out_a_returns_one() {
        let mut vm = create_test_vm();
        let result = text_out_a(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_bit_blt_returns_one() {
        let mut vm = create_test_vm();
        let result = bit_blt(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_rectangle_returns_one() {
        let mut vm = create_test_vm();
        let result = rectangle(&mut vm, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_write_point_null_ptr() {
        let mut vm = create_test_vm();
        // Should not panic with null ptr
        write_point(&mut vm, 0);
    }

    #[test]
    fn test_write_point_writes_zeros() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_u32(ptr, 0xDEADBEEF).unwrap();
        vm.write_u32(ptr + 4, 0xDEADBEEF).unwrap();
        write_point(&mut vm, ptr);
        assert_eq!(vm.read_u32(ptr).unwrap(), 0);
        assert_eq!(vm.read_u32(ptr + 4).unwrap(), 0);
    }
}
