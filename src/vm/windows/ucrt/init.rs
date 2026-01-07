//! UCRT init and termination stubs.

use crate::vm::Vm;
use crate::vm_args;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_initterm_e",
        initterm_e,
    );
    vm.register_import("api-ms-win-crt-runtime-l1-1-0.dll", "_initterm", initterm);
}

fn initterm(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (begin, end) = vm_args!(vm, stack_ptr; u32, u32);
    for addr in (begin..end).step_by(4) {
        let func = vm.read_u32(addr).unwrap_or(0);
        if func == 0 {
            continue;
        }
        let _ = vm.execute_at_with_stack(func, &[]);
    }
    0
}

fn initterm_e(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (begin, end) = vm_args!(vm, stack_ptr; u32, u32);
    for addr in (begin..end).step_by(4) {
        let func = vm.read_u32(addr).unwrap_or(0);
        if func == 0 {
            continue;
        }
        let result = vm.execute_at_with_stack(func, &[]).unwrap_or(1);
        if result != 0 {
            return result;
        }
    }
    0
}
