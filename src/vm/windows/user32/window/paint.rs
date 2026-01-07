use crate::register_func_stub;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::constants::DUMMY_HDC;

register_func_stub!(DLL_NAME, release_dc, 1);
register_func_stub!(DLL_NAME, end_paint, 1);
register_func_stub!(DLL_NAME, create_accelerator_table_a, 1);
register_func_stub!(DLL_NAME, destroy_accelerator_table, 1);

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "GetDC", crate::vm::stdcall_args(1), get_dc);
    vm.register_import_stdcall(DLL_NAME, "ReleaseDC", crate::vm::stdcall_args(2), release_dc);
    vm.register_import_stdcall(DLL_NAME, "BeginPaint", crate::vm::stdcall_args(2), begin_paint);
    vm.register_import_stdcall(DLL_NAME, "EndPaint", crate::vm::stdcall_args(2), end_paint);
    vm.register_import_stdcall(DLL_NAME, "CreateAcceleratorTableA", crate::vm::stdcall_args(2), create_accelerator_table_a);
    vm.register_import_stdcall(DLL_NAME, "DestroyAcceleratorTable", crate::vm::stdcall_args(1), destroy_accelerator_table);
}

pub(super) fn get_dc(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    DUMMY_HDC
}

pub(super) fn begin_paint(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (_, ps_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    if ps_ptr != 0 {
        let _ = vm.write_bytes(ps_ptr, &[0u8; 64]);
    }
    DUMMY_HDC
}
