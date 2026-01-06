//! Time conversion stubs.

use crate::vm::Vm;

// SystemTimeToVariantTime(...)
pub(super) fn system_time_to_variant_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    if out == 0 {
        return 0;
    }
    let bytes = 0f64.to_le_bytes();
    let _ = vm.write_bytes(out, &bytes);
    1
}

// VariantTimeToSystemTime(...)
pub(super) fn variant_time_to_system_time(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let out = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if out == 0 {
        return 0;
    }
    let _ = vm.write_bytes(out, &[0u8; 16]);
    1
}
