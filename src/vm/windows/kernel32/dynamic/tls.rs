use crate::vm::Vm;
use crate::vm_args;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_any_stdcall("FlsAlloc", crate::vm::stdcall_args(1), fls_alloc);
    vm.register_import_any_stdcall("FlsFree", crate::vm::stdcall_args(1), fls_free);
    vm.register_import_any_stdcall("FlsGetValue", crate::vm::stdcall_args(1), fls_get_value);
    vm.register_import_any_stdcall("FlsSetValue", crate::vm::stdcall_args(2), fls_set_value);
}

fn fls_alloc(vm: &mut Vm, _stack_ptr: u32) -> u32 {
    vm.tls_alloc()
}

fn fls_free(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index,) = vm_args!(vm, stack_ptr; u32);
    if vm.tls_free(index) {
        1
    } else {
        0
    }
}

fn fls_get_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index,) = vm_args!(vm, stack_ptr; u32);
    let value = vm.tls_get(index);
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] FlsGetValue: index={index} -> value=0x{value:08X}");
        // Dump first 64 bytes of the TLS data structure if value is valid
        if value != 0 && value >= vm.base && value < vm.base + vm.memory.len() as u32 {
            let mut bytes = Vec::new();
            for i in 0..64u32 {
                if let Ok(b) = vm.read_u8(value.wrapping_add(i)) {
                    bytes.push(format!("{b:02X}"));
                }
            }
            eprintln!("[pe_vm]   TLS data at 0x{value:08X}: {}", bytes.join(" "));
        }
    }
    value
}

fn fls_set_value(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (index, value) = vm_args!(vm, stack_ptr; u32, u32);
    if vm.tls_set(index, value) {
        1
    } else {
        0
    }
}
