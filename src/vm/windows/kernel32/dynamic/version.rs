use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall("GetVersionExA", crate::vm::stdcall_args(1), get_version_ex_a);
    vm.register_import_any_stdcall("GetVersionExW", crate::vm::stdcall_args(1), get_version_ex_w);
    vm.register_import_any_stdcall(
        "RtlGetNtVersionNumbers",
        crate::vm::stdcall_args(3),
        rtl_get_nt_version_numbers,
    );
}

// Provide a stable OS version for version checks inside DLLs.
fn get_version_ex_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_ptr,) = vm_args!(vm, stack_ptr; u32);
    if info_ptr == 0 {
        return 0;
    }
    let size = vm.read_u32(info_ptr).unwrap_or(0) as usize;
    write_os_version_a(vm, info_ptr, size);
    1
}

fn get_version_ex_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (info_ptr,) = vm_args!(vm, stack_ptr; u32);
    if info_ptr == 0 {
        return 0;
    }
    let size = vm.read_u32(info_ptr).unwrap_or(0) as usize;
    write_os_version_w(vm, info_ptr, size);
    1
}

fn rtl_get_nt_version_numbers(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (major_ptr, minor_ptr, build_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
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
