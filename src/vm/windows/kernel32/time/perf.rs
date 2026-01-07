use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "QueryPerformanceCounter",
        crate::vm::stdcall_args(1),
        query_performance_counter,
    );
}

fn query_performance_counter(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (counter_ptr,) = vm_args!(vm, stack_ptr; u32);
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
