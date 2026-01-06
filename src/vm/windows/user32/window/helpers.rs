use crate::vm::Vm;

pub(super) fn write_rect(vm: &mut Vm, rect_ptr: u32, left: i32, top: i32, right: i32, bottom: i32) {
    let _ = vm.write_u32(rect_ptr, left as u32);
    let _ = vm.write_u32(rect_ptr + 4, top as u32);
    let _ = vm.write_u32(rect_ptr + 8, right as u32);
    let _ = vm.write_u32(rect_ptr + 12, bottom as u32);
}

pub(super) fn write_point(vm: &mut Vm, point_ptr: u32, x: i32, y: i32) {
    let _ = vm.write_u32(point_ptr, x as u32);
    let _ = vm.write_u32(point_ptr + 4, y as u32);
}

pub(super) fn write_c_string(vm: &mut Vm, dest: u32, text: &str, max_len: usize) {
    let mut bytes = text.as_bytes().to_vec();
    if bytes.len() + 1 > max_len {
        bytes.truncate(max_len.saturating_sub(1));
    }
    bytes.push(0);
    let _ = vm.write_bytes(dest, &bytes);
}
