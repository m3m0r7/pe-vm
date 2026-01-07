use std::time::{SystemTime, UNIX_EPOCH};

use crate::vm::{Vm, REG_EDX};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "GetTickCount64",
        crate::vm::stdcall_args(0),
        get_tick_count64,
    );
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
