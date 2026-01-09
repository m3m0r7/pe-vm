use std::sync::OnceLock;
use std::time::Instant;

use crate::vm::{Vm, REG_EDX};

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall(
        "GetTickCount",
        crate::vm::stdcall_args(0),
        get_tick_count,
    );
    vm.register_import_any_stdcall(
        "GetTickCount64",
        crate::vm::stdcall_args(0),
        get_tick_count64,
    );
}

fn get_tick_count(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let start = TICK_START.get_or_init(Instant::now);
    start.elapsed().as_millis() as u32
}

fn get_tick_count64(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let start = TICK_START.get_or_init(Instant::now);
    let millis = start.elapsed().as_millis() as u64;
    let low = millis as u32;
    let high = (millis >> 32) as u32;
    vm.set_reg32(REG_EDX, high);
    low
}

static TICK_START: OnceLock<Instant> = OnceLock::new();
