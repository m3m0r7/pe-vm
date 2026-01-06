use crate::vm::Vm;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "GetTimeZoneInformation",
        crate::vm::stdcall_args(1),
        get_time_zone_information,
    );
}

fn get_time_zone_information(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let info_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if info_ptr == 0 {
        return 0;
    }
    let _ = vm.write_bytes(info_ptr, &[0u8; 172]);
    0
}
