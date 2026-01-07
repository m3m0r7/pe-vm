use crate::vm::windows::oleaut32::typelib::TypeInfoData;
use crate::vm::Vm;

#[derive(Clone, Copy)]
pub(super) struct InvokeArgs {
    pub(super) instance: u32,
    pub(super) memid: u32,
    pub(super) flags: u16,
    pub(super) disp_params: u32,
    pub(super) result_ptr: u32,
    pub(super) arg_err: u32,
    layout: &'static str,
}

pub(super) fn read_stack_slots(vm: &Vm, stack_ptr: u32) -> [u32; 9] {
    let mut slots = [0u32; 9];
    for (idx, slot) in slots.iter_mut().enumerate() {
        *slot = vm
            .read_u32(stack_ptr.wrapping_add((idx * 4) as u32))
            .unwrap_or(0);
    }
    slots
}

pub(super) fn select_invoke_args(
    info: &TypeInfoData,
    slots: &[u32; 9],
    thiscall: bool,
) -> InvokeArgs {
    let base = if thiscall { 1 } else { 2 };
    let normal = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "normal",
    };
    let no_flags = InvokeArgs {
        instance: slots[base],
        memid: slots[base + 1],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "no_flags",
    };
    let swapped_no_flags = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: 0,
        disp_params: slots[base + 2],
        result_ptr: slots[base + 3],
        arg_err: slots[base + 5],
        layout: "swapped_no_flags",
    };
    let swapped_normal = InvokeArgs {
        instance: slots[base + 1],
        memid: slots[base],
        flags: slots[base + 2] as u16,
        disp_params: slots[base + 3],
        result_ptr: slots[base + 4],
        arg_err: slots[base + 6],
        layout: "swapped_normal",
    };

    let mut selected = normal;
    for candidate in [normal, no_flags, swapped_no_flags, swapped_normal] {
        if info.funcs.iter().any(|func| func.memid == candidate.memid) {
            selected = candidate;
            break;
        }
    }

    if std::env::var("PE_VM_TRACE_COM").is_ok() {
        eprintln!(
            "[pe_vm] ITypeInfo::Invoke layout={} memid=0x{:08X} flags=0x{:04X} disp=0x{:08X}",
            selected.layout, selected.memid, selected.flags, selected.disp_params
        );
    }

    selected
}
