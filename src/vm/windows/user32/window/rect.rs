use crate::vm::Vm;

use super::helpers::write_rect;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "USER32.dll",
        "PtInRect",
        crate::vm::stdcall_args(3),
        pt_in_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "EqualRect",
        crate::vm::stdcall_args(2),
        equal_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "OffsetRect",
        crate::vm::stdcall_args(3),
        offset_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "UnionRect",
        crate::vm::stdcall_args(3),
        union_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "IntersectRect",
        crate::vm::stdcall_args(3),
        intersect_rect,
    );
    vm.register_import_stdcall(
        "USER32.dll",
        "FillRect",
        crate::vm::stdcall_args(3),
        fill_rect,
    );
}

pub(super) fn pt_in_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn equal_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

pub(super) fn offset_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let rect_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let dx = vm.read_u32(stack_ptr + 8).unwrap_or(0) as i32;
    let dy = vm.read_u32(stack_ptr + 12).unwrap_or(0) as i32;
    if rect_ptr != 0 {
        let left = vm.read_u32(rect_ptr).unwrap_or(0) as i32 + dx;
        let top = vm.read_u32(rect_ptr + 4).unwrap_or(0) as i32 + dy;
        let right = vm.read_u32(rect_ptr + 8).unwrap_or(0) as i32 + dx;
        let bottom = vm.read_u32(rect_ptr + 12).unwrap_or(0) as i32 + dy;
        write_rect(vm, rect_ptr, left, top, right, bottom);
    }
    1
}

pub(super) fn union_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dst_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src1_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src2_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if dst_ptr != 0 && src1_ptr != 0 && src2_ptr != 0 {
        let l1 = vm.read_u32(src1_ptr).unwrap_or(0) as i32;
        let t1 = vm.read_u32(src1_ptr + 4).unwrap_or(0) as i32;
        let r1 = vm.read_u32(src1_ptr + 8).unwrap_or(0) as i32;
        let b1 = vm.read_u32(src1_ptr + 12).unwrap_or(0) as i32;
        let l2 = vm.read_u32(src2_ptr).unwrap_or(0) as i32;
        let t2 = vm.read_u32(src2_ptr + 4).unwrap_or(0) as i32;
        let r2 = vm.read_u32(src2_ptr + 8).unwrap_or(0) as i32;
        let b2 = vm.read_u32(src2_ptr + 12).unwrap_or(0) as i32;
        write_rect(vm, dst_ptr, l1.min(l2), t1.min(t2), r1.max(r2), b1.max(b2));
    }
    1
}

pub(super) fn intersect_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dst_ptr = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let src1_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let src2_ptr = vm.read_u32(stack_ptr + 12).unwrap_or(0);
    if dst_ptr == 0 || src1_ptr == 0 || src2_ptr == 0 {
        return 0;
    }
    let l1 = vm.read_u32(src1_ptr).unwrap_or(0) as i32;
    let t1 = vm.read_u32(src1_ptr + 4).unwrap_or(0) as i32;
    let r1 = vm.read_u32(src1_ptr + 8).unwrap_or(0) as i32;
    let b1 = vm.read_u32(src1_ptr + 12).unwrap_or(0) as i32;
    let l2 = vm.read_u32(src2_ptr).unwrap_or(0) as i32;
    let t2 = vm.read_u32(src2_ptr + 4).unwrap_or(0) as i32;
    let r2 = vm.read_u32(src2_ptr + 8).unwrap_or(0) as i32;
    let b2 = vm.read_u32(src2_ptr + 12).unwrap_or(0) as i32;
    let left = l1.max(l2);
    let top = t1.max(t2);
    let right = r1.min(r2);
    let bottom = b1.min(b2);
    if right <= left || bottom <= top {
        write_rect(vm, dst_ptr, 0, 0, 0, 0);
        return 0;
    }
    write_rect(vm, dst_ptr, left, top, right, bottom);
    1
}

pub(super) fn fill_rect(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    1
}
