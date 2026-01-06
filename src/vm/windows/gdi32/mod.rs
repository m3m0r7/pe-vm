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
