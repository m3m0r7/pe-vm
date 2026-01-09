//! Minimal MSVCR40D.dll stubs needed by legacy DLLs.

use crate::vm::Vm;
use crate::vm_args;

pub const DLL_NAME: &str = "MSVCR40D.dll";

const FILE_STRUCT_SIZE: usize = 0x20;
const FILE_FLAG_OFFSET: u32 = 0x0C;
const FILE_HANDLE_OFFSET: u32 = 0x10;
const IOREAD: u32 = 0x0001;
const IOWRT: u32 = 0x0002;
const IOEOF: u32 = 0x0010;
const IOERR: u32 = 0x0020;

pub fn register(vm: &mut Vm) {
    vm.register_import(DLL_NAME, "sprintf", sprintf);
    vm.register_import(DLL_NAME, "malloc", malloc_impl);
    vm.register_import(DLL_NAME, "memcpy", memcpy_impl);
    vm.register_import(DLL_NAME, "fopen", fopen_impl);
    vm.register_import(DLL_NAME, "fclose", fclose_impl);
    vm.register_import(DLL_NAME, "fgetc", fgetc_impl);
    vm.register_import(DLL_NAME, "__CxxFrameHandler", cxx_frame_handler);
    vm.register_import(DLL_NAME, "??1type_info@@UAE@XZ", type_info_dtor);
    vm.register_import(DLL_NAME, "??3@YAXPAX@Z", operator_delete);
    vm.register_import(DLL_NAME, "_adjust_fdiv", adjust_fdiv);
    vm.register_import(DLL_NAME, "_malloc_dbg", malloc_dbg);
    vm.register_import(DLL_NAME, "_initterm", initterm);
    vm.register_import(DLL_NAME, "_free_dbg", free_dbg);
    vm.register_import(DLL_NAME, "_onexit", onexit);
    vm.register_import(DLL_NAME, "fputc", fputc_impl);
    vm.register_import(DLL_NAME, "strcpy", strcpy_impl);
    vm.register_import(DLL_NAME, "memset", memset_impl);
    vm.register_import(DLL_NAME, "strcat", strcat_impl);
    vm.register_import(DLL_NAME, "__dllonexit", dll_onexit);
}

fn malloc_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (size,) = vm_args!(vm, stack_ptr; u32);
    vm.heap_alloc(size as usize)
}

fn malloc_dbg(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (size, _block, _file, _line) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
    vm.heap_alloc(size as usize)
}

fn free_dbg(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (mem, _block) = vm_args!(vm, stack_ptr; u32, u32);
    if mem != 0 {
        let _ = vm.heap_free(mem);
    }
    0
}

fn operator_delete(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (mem,) = vm_args!(vm, stack_ptr; u32);
    if mem != 0 {
        let _ = vm.heap_free(mem);
    }
    0
}

fn memcpy_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dst, src, len) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if dst == 0 || src == 0 || len == 0 {
        return dst;
    }
    let mut bytes = Vec::with_capacity(len as usize);
    for offset in 0..len {
        bytes.push(vm.read_u8(src.wrapping_add(offset)).unwrap_or(0));
    }
    let _ = vm.write_bytes(dst, &bytes);
    dst
}

fn memset_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dst, value, len) = vm_args!(vm, stack_ptr; u32, u32, u32);
    if dst != 0 && len != 0 {
        let _ = vm.memset(dst, value as u8, len as usize);
    }
    dst
}

fn strcpy_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dst, src) = vm_args!(vm, stack_ptr; u32, u32);
    if dst == 0 {
        return 0;
    }
    let bytes = read_c_string_bytes(vm, src);
    if let Some(text) = bytes_to_ascii(&bytes) {
        if looks_like_ipv4(&text) {
            trace_msvcr(&format!("MSVCR40D strcpy src={text:?}"));
        }
    }
    let mut out = bytes;
    out.push(0);
    let _ = vm.write_bytes(dst, &out);
    dst
}

fn strcat_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (dst, src) = vm_args!(vm, stack_ptr; u32, u32);
    if dst == 0 {
        return 0;
    }
    let mut dst_bytes = read_c_string_bytes(vm, dst);
    let src_bytes = read_c_string_bytes(vm, src);
    if let Some(text) = bytes_to_ascii(&src_bytes) {
        if looks_like_ipv4(&text) {
            trace_msvcr(&format!("MSVCR40D strcat src={text:?}"));
        }
    }
    dst_bytes.extend_from_slice(&src_bytes);
    dst_bytes.push(0);
    let _ = vm.write_bytes(dst, &dst_bytes);
    dst
}

fn fopen_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (path_ptr, mode_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    let path = read_c_string(vm, path_ptr);
    if path.is_empty() {
        return 0;
    }
    let mode = read_c_string(vm, mode_ptr);
    let (readable, writable, create, truncate, append) = parse_mode(&mode);
    let handle = vm
        .file_open(&path, readable, writable, create, truncate)
        .unwrap_or(0);
    if handle == 0 {
        return 0;
    }
    if append {
        let _ = vm.file_seek(handle, 0, 2);
    }
    let file_ptr = vm.heap_alloc(FILE_STRUCT_SIZE);
    if file_ptr == 0 {
        return 0;
    }
    let _ = vm.memset(file_ptr, 0, FILE_STRUCT_SIZE);
    let mut flags = 0;
    if readable {
        flags |= IOREAD;
    }
    if writable {
        flags |= IOWRT;
    }
    let _ = vm.write_u32(file_ptr + FILE_FLAG_OFFSET, flags);
    let _ = vm.write_u32(file_ptr + FILE_HANDLE_OFFSET, handle);
    file_ptr
}

fn fclose_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (file_ptr,) = vm_args!(vm, stack_ptr; u32);
    if file_ptr == 0 {
        return 0xFFFF_FFFF;
    }
    if let Some(handle) = read_file_handle(vm, file_ptr) {
        let _ = vm.file_close(handle);
    }
    let _ = vm.write_u32(file_ptr + FILE_HANDLE_OFFSET, 0);
    let _ = vm.write_u32(file_ptr + FILE_FLAG_OFFSET, 0);
    0
}

fn fgetc_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (file_ptr,) = vm_args!(vm, stack_ptr; u32);
    let Some(handle) = read_file_handle(vm, file_ptr) else {
        return 0xFFFF_FFFF;
    };
    let Some(bytes) = vm.file_read(handle, 1) else {
        set_file_flag(vm, file_ptr, IOERR, true);
        return 0xFFFF_FFFF;
    };
    if let Some(byte) = bytes.first() {
        set_file_flag(vm, file_ptr, IOEOF, false);
        *byte as u32
    } else {
        set_file_flag(vm, file_ptr, IOEOF, true);
        0xFFFF_FFFF
    }
}

fn fputc_impl(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (value, file_ptr) = vm_args!(vm, stack_ptr; u32, u32);
    let Some(handle) = read_file_handle(vm, file_ptr) else {
        return 0xFFFF_FFFF;
    };
    let byte = (value & 0xFF) as u8;
    let Some(written) = vm.file_write(handle, &[byte]) else {
        set_file_flag(vm, file_ptr, IOERR, true);
        return 0xFFFF_FFFF;
    };
    if written == 1 {
        set_file_flag(vm, file_ptr, IOERR, false);
        value & 0xFF
    } else {
        set_file_flag(vm, file_ptr, IOERR, true);
        0xFFFF_FFFF
    }
}

fn sprintf(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let dst = vm.read_u32(stack_ptr + 4).unwrap_or(0);
    let fmt_ptr = vm.read_u32(stack_ptr + 8).unwrap_or(0);
    let format = read_c_string(vm, fmt_ptr);
    if format.is_empty() {
        if dst != 0 {
            let _ = vm.write_u8(dst, 0);
        }
        return 0;
    }
    let mut arg_ptr = stack_ptr + 12;
    let mut output = String::new();
    let mut chars = format.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            output.push(ch);
            continue;
        }
        if matches!(chars.peek(), Some('%')) {
            chars.next();
            output.push('%');
            continue;
        }
        while matches!(chars.peek(), Some('-' | '+' | ' ' | '#' | '0')) {
            chars.next();
        }
        if matches!(chars.peek(), Some('*')) {
            chars.next();
            let _ = read_arg_u32(vm, &mut arg_ptr);
        } else {
            while matches!(chars.peek(), Some('0'..='9')) {
                chars.next();
            }
        }
        if matches!(chars.peek(), Some('.')) {
            chars.next();
            if matches!(chars.peek(), Some('*')) {
                chars.next();
                let _ = read_arg_u32(vm, &mut arg_ptr);
            } else {
                while matches!(chars.peek(), Some('0'..='9')) {
                    chars.next();
                }
            }
        }
        if matches!(chars.peek(), Some('h' | 'l' | 'L' | 'I')) {
            let marker = chars.next().unwrap_or(' ');
            if marker == 'l' && matches!(chars.peek(), Some('l')) {
                chars.next();
            } else if marker == 'I' {
                if matches!(chars.peek(), Some('6')) {
                    chars.next();
                    if matches!(chars.peek(), Some('4')) {
                        chars.next();
                    }
                }
            }
        }
        let spec = chars.next().unwrap_or('%');
        match spec {
            's' => {
                let ptr = read_arg_u32(vm, &mut arg_ptr);
                output.push_str(&read_c_string(vm, ptr));
            }
            'd' | 'i' => {
                let value = read_arg_u32(vm, &mut arg_ptr) as i32;
                output.push_str(&value.to_string());
            }
            'u' => {
                let value = read_arg_u32(vm, &mut arg_ptr);
                output.push_str(&value.to_string());
            }
            'x' => {
                let value = read_arg_u32(vm, &mut arg_ptr);
                output.push_str(&format!("{value:x}"));
            }
            'X' => {
                let value = read_arg_u32(vm, &mut arg_ptr);
                output.push_str(&format!("{value:X}"));
            }
            'c' => {
                let value = read_arg_u32(vm, &mut arg_ptr);
                output.push((value as u8) as char);
            }
            'p' => {
                let value = read_arg_u32(vm, &mut arg_ptr);
                output.push_str(&format!("0x{value:08X}"));
            }
            _ => {
                output.push('%');
                output.push(spec);
            }
        }
    }
    if dst != 0 {
        let mut bytes = output.as_bytes().to_vec();
        bytes.push(0);
        let _ = vm.write_bytes(dst, &bytes);
    }
    output.len() as u32
}

fn cxx_frame_handler(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn type_info_dtor(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn adjust_fdiv(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn initterm(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn onexit(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn dll_onexit(_vm: &mut Vm, _stack_ptr: u32) -> u32 {
    0
}

fn read_arg_u32(vm: &Vm, arg_ptr: &mut u32) -> u32 {
    let value = vm.read_u32(*arg_ptr).unwrap_or(0);
    *arg_ptr = arg_ptr.wrapping_add(4);
    value
}

fn read_c_string(vm: &Vm, ptr: u32) -> String {
    read_str_delim!(vm, ptr, 0u8)
}

fn read_file_handle(vm: &Vm, file_ptr: u32) -> Option<u32> {
    if file_ptr == 0 {
        return None;
    }
    if vm.contains_addr(file_ptr) {
        vm.read_u32(file_ptr + FILE_HANDLE_OFFSET).ok()
    } else {
        Some(file_ptr)
    }
}

fn set_file_flag(vm: &mut Vm, file_ptr: u32, mask: u32, set: bool) {
    if file_ptr == 0 || !vm.contains_addr(file_ptr) {
        return;
    }
    let Ok(flags) = vm.read_u32(file_ptr + FILE_FLAG_OFFSET) else {
        return;
    };
    let next = if set { flags | mask } else { flags & !mask };
    let _ = vm.write_u32(file_ptr + FILE_FLAG_OFFSET, next);
}

fn trace_msvcr(message: &str) {
    if std::env::var("PE_VM_TRACE_NET").is_ok() || std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] {message}");
    }
}

fn bytes_to_ascii(bytes: &[u8]) -> Option<String> {
    if bytes.is_empty() {
        return None;
    }
    if !bytes.iter().all(|b| matches!(b, 0x20..=0x7E)) {
        return None;
    }
    Some(String::from_utf8_lossy(bytes).into_owned())
}

fn looks_like_ipv4(text: &str) -> bool {
    let mut dots = 0u8;
    let mut has_digit = false;
    for ch in text.chars() {
        match ch {
            '0'..='9' => has_digit = true,
            '.' => dots += 1,
            _ => return false,
        }
    }
    has_digit && dots == 3
}

fn read_c_string_bytes(vm: &Vm, ptr: u32) -> Vec<u8> {
    if ptr == 0 {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    let mut cursor = ptr;
    for _ in 0..0x10000u32 {
        let byte = vm.read_u8(cursor).unwrap_or(0);
        if byte == 0 {
            break;
        }
        bytes.push(byte);
        cursor = cursor.wrapping_add(1);
    }
    bytes
}

fn parse_mode(mode: &str) -> (bool, bool, bool, bool, bool) {
    let readable = mode.contains('r') || mode.contains('+');
    let writable = mode.contains('w') || mode.contains('a') || mode.contains('+');
    let create = mode.contains('w') || mode.contains('a') || mode.contains('+');
    let truncate = mode.contains('w');
    let append = mode.contains('a');
    (readable, writable, create, truncate, append)
}
