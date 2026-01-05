//! UCRT onexit table stubs.

use crate::vm::Vm;

pub fn register(vm: &mut Vm) {
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_initialize_onexit_table",
        initialize_onexit_table,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_cexit",
        cexit,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_crt_atexit",
        crt_atexit,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_register_onexit_function",
        register_onexit_function,
    );
    vm.register_import(
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "_execute_onexit_table",
        execute_onexit_table,
    );
}

fn initialize_onexit_table(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let table_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if table_ptr == 0 {
        return 1;
    }
    vm.set_default_onexit_table(table_ptr);
    vm.onexit_table_mut(table_ptr).clear();
    let _ = vm.write_u32(table_ptr, 0);
    let _ = vm.write_u32(table_ptr.wrapping_add(4), 0);
    let _ = vm.write_u32(table_ptr.wrapping_add(8), 0);
    0
}

fn register_onexit_function(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let table_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    let func = vm.read_u32(stack_ptr.wrapping_add(8)).unwrap_or(0);
    if table_ptr == 0 || func == 0 {
        return 1;
    }
    vm.onexit_table_mut(table_ptr).push(func);
    0
}

fn execute_onexit_table(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let table_ptr = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if table_ptr == 0 {
        return 0;
    }
    let entries = vm.take_onexit_table(table_ptr);
    for func in entries.into_iter().rev() {
        if func == 0 {
            continue;
        }
        if vm.execute_at_with_stack(func, &[]).is_err() {
            return 1;
        }
    }
    0
}

fn crt_atexit(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let func = vm.read_u32(stack_ptr.wrapping_add(4)).unwrap_or(0);
    if func == 0 {
        return 1;
    }
    let table_ptr = vm.default_onexit_table();
    if table_ptr == 0 {
        return 0;
    }
    vm.onexit_table_mut(table_ptr).push(func);
    0
}

fn cexit(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    let table_ptr = vm.default_onexit_table();
    if table_ptr == 0 {
        return 0;
    }
    let entries = vm.take_onexit_table(table_ptr);
    for func in entries.into_iter().rev() {
        if func == 0 {
            continue;
        }
        let _ = vm.execute_at_with_stack(func, &[]);
    }
    0
}
