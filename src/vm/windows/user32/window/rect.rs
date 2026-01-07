use crate::register_func_stub;
use crate::vm::windows::user32::DLL_NAME;
use crate::vm::Vm;
use crate::vm_args;

use super::helpers::write_rect;

register_func_stub!(DLL_NAME, pt_in_rect, 0);
register_func_stub!(DLL_NAME, equal_rect, 0);
register_func_stub!(DLL_NAME, fill_rect, 1);

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(DLL_NAME, "PtInRect", crate::vm::stdcall_args(3), pt_in_rect);
    vm.register_import_stdcall(DLL_NAME, "EqualRect", crate::vm::stdcall_args(2), equal_rect);
    vm.register_import_stdcall(DLL_NAME, "OffsetRect", crate::vm::stdcall_args(3), offset_rect);
    vm.register_import_stdcall(DLL_NAME, "UnionRect", crate::vm::stdcall_args(3), union_rect);
    vm.register_import_stdcall(DLL_NAME, "IntersectRect", crate::vm::stdcall_args(3), intersect_rect);
    vm.register_import_stdcall(DLL_NAME, "FillRect", crate::vm::stdcall_args(3), fill_rect);
}

pub(super) fn offset_rect(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (rect_ptr, dx, dy) = vm_args!(vm, stack_ptr; u32, i32, i32);
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
    let (dst_ptr, src1_ptr, src2_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
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
    let (dst_ptr, src1_ptr, src2_ptr) = vm_args!(vm, stack_ptr; u32, u32, u32);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BypassSettings;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut bypass = BypassSettings::new();
        bypass.not_implemented_module = true;
        let mut vm = Vm::new(
            VmConfig::new()
                .architecture(Architecture::X86)
                .bypass(bypass),
        )
        .expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    fn write_rect_to_mem(vm: &mut Vm, ptr: u32, left: i32, top: i32, right: i32, bottom: i32) {
        vm.write_u32(ptr, left as u32).unwrap();
        vm.write_u32(ptr + 4, top as u32).unwrap();
        vm.write_u32(ptr + 8, right as u32).unwrap();
        vm.write_u32(ptr + 12, bottom as u32).unwrap();
    }

    #[test]
    fn test_pt_in_rect_returns_zero() {
        let mut vm = create_test_vm();
        let result = pt_in_rect(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_equal_rect_returns_zero() {
        let mut vm = create_test_vm();
        let result = equal_rect(&mut vm, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_offset_rect_adds_delta() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        let rect_ptr = vm.heap_start as u32;
        // rect = {10, 20, 100, 200}
        write_rect_to_mem(&mut vm, rect_ptr, 10, 20, 100, 200);
        vm.write_u32(stack + 4, rect_ptr).unwrap();
        vm.write_u32(stack + 8, 5u32).unwrap();  // dx = 5
        vm.write_u32(stack + 12, 10u32).unwrap(); // dy = 10
        let result = offset_rect(&mut vm, stack);
        assert_eq!(result, 1);
        assert_eq!(vm.read_u32(rect_ptr).unwrap() as i32, 15);   // left + dx
        assert_eq!(vm.read_u32(rect_ptr + 4).unwrap() as i32, 30); // top + dy
        assert_eq!(vm.read_u32(rect_ptr + 8).unwrap() as i32, 105); // right + dx
        assert_eq!(vm.read_u32(rect_ptr + 12).unwrap() as i32, 210); // bottom + dy
    }

    #[test]
    fn test_union_rect() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let dst = vm.heap_start as u32;
        let src1 = dst + 16;
        let src2 = dst + 32;
        // src1 = {10, 20, 50, 60}
        write_rect_to_mem(&mut vm, src1, 10, 20, 50, 60);
        // src2 = {30, 10, 100, 80}
        write_rect_to_mem(&mut vm, src2, 30, 10, 100, 80);
        vm.write_u32(stack + 4, dst).unwrap();
        vm.write_u32(stack + 8, src1).unwrap();
        vm.write_u32(stack + 12, src2).unwrap();
        let result = union_rect(&mut vm, stack);
        assert_eq!(result, 1);
        // union should be {min(10,30), min(20,10), max(50,100), max(60,80)} = {10, 10, 100, 80}
        assert_eq!(vm.read_u32(dst).unwrap() as i32, 10);
        assert_eq!(vm.read_u32(dst + 4).unwrap() as i32, 10);
        assert_eq!(vm.read_u32(dst + 8).unwrap() as i32, 100);
        assert_eq!(vm.read_u32(dst + 12).unwrap() as i32, 80);
    }

    #[test]
    fn test_intersect_rect_overlapping() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let dst = vm.heap_start as u32;
        let src1 = dst + 16;
        let src2 = dst + 32;
        // src1 = {0, 0, 100, 100}
        write_rect_to_mem(&mut vm, src1, 0, 0, 100, 100);
        // src2 = {50, 50, 150, 150}
        write_rect_to_mem(&mut vm, src2, 50, 50, 150, 150);
        vm.write_u32(stack + 4, dst).unwrap();
        vm.write_u32(stack + 8, src1).unwrap();
        vm.write_u32(stack + 12, src2).unwrap();
        let result = intersect_rect(&mut vm, stack);
        assert_eq!(result, 1);
        // intersection should be {50, 50, 100, 100}
        assert_eq!(vm.read_u32(dst).unwrap() as i32, 50);
        assert_eq!(vm.read_u32(dst + 4).unwrap() as i32, 50);
        assert_eq!(vm.read_u32(dst + 8).unwrap() as i32, 100);
        assert_eq!(vm.read_u32(dst + 12).unwrap() as i32, 100);
    }

    #[test]
    fn test_intersect_rect_no_overlap() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        let dst = vm.heap_start as u32;
        let src1 = dst + 16;
        let src2 = dst + 32;
        // src1 = {0, 0, 10, 10}
        write_rect_to_mem(&mut vm, src1, 0, 0, 10, 10);
        // src2 = {20, 20, 30, 30}
        write_rect_to_mem(&mut vm, src2, 20, 20, 30, 30);
        vm.write_u32(stack + 4, dst).unwrap();
        vm.write_u32(stack + 8, src1).unwrap();
        vm.write_u32(stack + 12, src2).unwrap();
        let result = intersect_rect(&mut vm, stack);
        assert_eq!(result, 0); // no intersection
        // dst should be {0, 0, 0, 0}
        assert_eq!(vm.read_u32(dst).unwrap(), 0);
        assert_eq!(vm.read_u32(dst + 4).unwrap(), 0);
        assert_eq!(vm.read_u32(dst + 8).unwrap(), 0);
        assert_eq!(vm.read_u32(dst + 12).unwrap(), 0);
    }

    #[test]
    fn test_intersect_rect_null_ptrs() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        vm.write_u32(stack + 4, 0).unwrap();
        vm.write_u32(stack + 8, 0).unwrap();
        vm.write_u32(stack + 12, 0).unwrap();
        let result = intersect_rect(&mut vm, stack);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fill_rect_returns_one() {
        let mut vm = create_test_vm();
        let result = fill_rect(&mut vm, 0);
        assert_eq!(result, 1);
    }
}
