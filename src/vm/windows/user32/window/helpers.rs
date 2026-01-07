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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_write_rect() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_rect(&mut vm, ptr, 10, 20, 100, 200);
        assert_eq!(vm.read_u32(ptr).unwrap() as i32, 10); // left
        assert_eq!(vm.read_u32(ptr + 4).unwrap() as i32, 20); // top
        assert_eq!(vm.read_u32(ptr + 8).unwrap() as i32, 100); // right
        assert_eq!(vm.read_u32(ptr + 12).unwrap() as i32, 200); // bottom
    }

    #[test]
    fn test_write_rect_negative_values() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_rect(&mut vm, ptr, -10, -20, 50, 100);
        assert_eq!(vm.read_u32(ptr).unwrap() as i32, -10);
        assert_eq!(vm.read_u32(ptr + 4).unwrap() as i32, -20);
    }

    #[test]
    fn test_write_point() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_point(&mut vm, ptr, 150, 250);
        assert_eq!(vm.read_u32(ptr).unwrap() as i32, 150); // x
        assert_eq!(vm.read_u32(ptr + 4).unwrap() as i32, 250); // y
    }

    #[test]
    fn test_write_point_negative() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_point(&mut vm, ptr, -100, -200);
        assert_eq!(vm.read_u32(ptr).unwrap() as i32, -100);
        assert_eq!(vm.read_u32(ptr + 4).unwrap() as i32, -200);
    }

    #[test]
    fn test_write_c_string() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_c_string(&mut vm, ptr, "Hello", 10);
        let text = vm.read_c_string(ptr).unwrap();
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_write_c_string_truncation() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_c_string(&mut vm, ptr, "HelloWorld", 6);
        let text = vm.read_c_string(ptr).unwrap();
        assert_eq!(text, "Hello"); // truncated to 5 chars + null
    }

    #[test]
    fn test_write_c_string_empty() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        write_c_string(&mut vm, ptr, "", 10);
        let text = vm.read_c_string(ptr).unwrap();
        assert_eq!(text, "");
    }
}
