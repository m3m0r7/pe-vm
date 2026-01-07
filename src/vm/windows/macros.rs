//! Macros for reading Windows API function arguments from the stack.
//!
//! # Usage
//!
//! ```ignore
//! // Basic u32 arguments
//! let (handle, buffer, count, written,) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
//!
//! // Mixed types
//! let (handle, count,) = vm_args!(vm, stack_ptr; u32, usize);
//!
//! // With strings (ANSI)
//! let (path,) = vm_args!(vm, stack_ptr; str);
//!
//! // With wide strings
//! let (path,) = vm_args!(vm, stack_ptr; wstr);
//! ```

use crate::vm::Vm;

/// Read a null-terminated ANSI string from a pointer.
#[inline]
pub fn read_str_arg(vm: &Vm, ptr: u32) -> String {
    if ptr == 0 {
        return String::new();
    }
    vm.read_c_string(ptr).unwrap_or_default()
}

/// Read a null-terminated wide (UTF-16) string from a pointer.
#[inline]
pub fn read_wstr_arg(vm: &Vm, ptr: u32) -> String {
    read_wstr_delim(vm, ptr, 0)
}

/// Read an ANSI string with specified length.
#[inline]
pub fn read_str_len(vm: &Vm, ptr: u32, len: usize) -> String {
    if ptr == 0 || len == 0 {
        return String::new();
    }
    let mut bytes = Vec::with_capacity(len);
    for i in 0..len {
        let b = vm.read_u8(ptr + i as u32).unwrap_or(0);
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Read a wide (UTF-16) string with specified length (in characters).
#[inline]
pub fn read_wstr_len(vm: &Vm, ptr: u32, len: usize) -> String {
    if ptr == 0 || len == 0 {
        return String::new();
    }
    let mut units = Vec::with_capacity(len);
    for i in 0..len {
        let unit = vm.read_u16(ptr + (i as u32) * 2).unwrap_or(0);
        units.push(unit);
    }
    String::from_utf16_lossy(&units)
}

/// Read an ANSI string until delimiter byte (default: 0 for null-terminated).
#[inline]
pub fn read_str_delim(vm: &Vm, ptr: u32, delim: u8) -> String {
    if ptr == 0 {
        return String::new();
    }
    let mut bytes = Vec::new();
    let mut cursor = ptr;
    for _ in 0..0x10000 {
        let b = vm.read_u8(cursor).unwrap_or(delim);
        if b == delim {
            break;
        }
        bytes.push(b);
        cursor = cursor.wrapping_add(1);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Read a wide (UTF-16) string until delimiter (default: 0 for null-terminated).
#[inline]
pub fn read_wstr_delim(vm: &Vm, ptr: u32, delim: u16) -> String {
    if ptr == 0 {
        return String::new();
    }
    let mut units = Vec::new();
    let mut cursor = ptr;
    for _ in 0..0x10000 {
        let unit = vm.read_u16(cursor).unwrap_or(delim);
        if unit == delim {
            break;
        }
        units.push(unit);
        cursor = cursor.wrapping_add(2);
    }
    String::from_utf16_lossy(&units)
}

/// Macro to read function arguments from the stack.
///
/// # Syntax
///
/// ```ignore
/// vm_args!(vm, stack_ptr; type1, type2, ...)
/// ```
///
/// # Supported Types
///
/// - `u32` - 32-bit unsigned integer (default for handles, pointers, etc.)
/// - `i32` - 32-bit signed integer
/// - `u16` - 16-bit unsigned integer (read from low 16 bits of u32)
/// - `usize` - Platform-sized unsigned integer (cast from u32)
/// - `str` - ANSI null-terminated string (reads pointer, then string)
/// - `wstr` - Wide null-terminated string (reads pointer, then UTF-16 string)
///
/// # Examples
///
/// ```ignore
/// // Read 4 u32 arguments
/// let (handle, buffer, count, written,) = vm_args!(vm, stack_ptr; u32, u32, u32, u32);
///
/// // Read mixed types
/// let (handle, index,) = vm_args!(vm, stack_ptr; u32, usize);
///
/// // Read with string (returns tuple for strings)
/// let (path, flags) = vm_args!(vm, stack_ptr; str, u32);
/// ```
#[macro_export]
macro_rules! vm_args {
    // Internal: count arguments
    (@count) => { 0usize };
    (@count $head:tt $($tail:tt)*) => { 1usize + $crate::vm_args!(@count $($tail)*) };

    // Internal: read single argument by type
    (@read $vm:expr, $offset:expr, u32) => {
        $vm.read_u32($offset).unwrap_or(0)
    };
    (@read $vm:expr, $offset:expr, i32) => {
        $vm.read_u32($offset).unwrap_or(0) as i32
    };
    (@read $vm:expr, $offset:expr, u16) => {
        ($vm.read_u32($offset).unwrap_or(0) & 0xFFFF) as u16
    };
    (@read $vm:expr, $offset:expr, usize) => {
        $vm.read_u32($offset).unwrap_or(0) as usize
    };
    (@read $vm:expr, $offset:expr, str) => {{
        let ptr = $vm.read_u32($offset).unwrap_or(0);
        $crate::vm::windows::macros::read_str_arg($vm, ptr)
    }};
    (@read $vm:expr, $offset:expr, wstr) => {{
        let ptr = $vm.read_u32($offset).unwrap_or(0);
        $crate::vm::windows::macros::read_wstr_arg($vm, ptr)
    }};

    // Entry point: generate tuple/array based on types
    // For all-numeric types, returns array. For strings, returns tuple.

    // 1 argument (returns single-element tuple for consistency)
    ($vm:expr, $sp:expr; $t1:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        (v1,)
    }};

    // 2 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        (v1, v2)
    }};

    // 3 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        (v1, v2, v3)
    }};

    // 4 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        (v1, v2, v3, v4)
    }};

    // 5 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        (v1, v2, v3, v4, v5)
    }};

    // 6 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        (v1, v2, v3, v4, v5, v6)
    }};

    // 7 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        (v1, v2, v3, v4, v5, v6, v7)
    }};

    // 8 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt, $t8:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        let v8 = $crate::vm_args!(@read $vm, $sp + 32, $t8);
        (v1, v2, v3, v4, v5, v6, v7, v8)
    }};

    // 9 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt, $t8:tt, $t9:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        let v8 = $crate::vm_args!(@read $vm, $sp + 32, $t8);
        let v9 = $crate::vm_args!(@read $vm, $sp + 36, $t9);
        (v1, v2, v3, v4, v5, v6, v7, v8, v9)
    }};

    // 10 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt, $t8:tt, $t9:tt, $t10:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        let v8 = $crate::vm_args!(@read $vm, $sp + 32, $t8);
        let v9 = $crate::vm_args!(@read $vm, $sp + 36, $t9);
        let v10 = $crate::vm_args!(@read $vm, $sp + 40, $t10);
        (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
    }};

    // 11 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt, $t8:tt, $t9:tt, $t10:tt, $t11:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        let v8 = $crate::vm_args!(@read $vm, $sp + 32, $t8);
        let v9 = $crate::vm_args!(@read $vm, $sp + 36, $t9);
        let v10 = $crate::vm_args!(@read $vm, $sp + 40, $t10);
        let v11 = $crate::vm_args!(@read $vm, $sp + 44, $t11);
        (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
    }};

    // 12 arguments
    ($vm:expr, $sp:expr; $t1:tt, $t2:tt, $t3:tt, $t4:tt, $t5:tt, $t6:tt, $t7:tt, $t8:tt, $t9:tt, $t10:tt, $t11:tt, $t12:tt) => {{
        let v1 = $crate::vm_args!(@read $vm, $sp + 4, $t1);
        let v2 = $crate::vm_args!(@read $vm, $sp + 8, $t2);
        let v3 = $crate::vm_args!(@read $vm, $sp + 12, $t3);
        let v4 = $crate::vm_args!(@read $vm, $sp + 16, $t4);
        let v5 = $crate::vm_args!(@read $vm, $sp + 20, $t5);
        let v6 = $crate::vm_args!(@read $vm, $sp + 24, $t6);
        let v7 = $crate::vm_args!(@read $vm, $sp + 28, $t7);
        let v8 = $crate::vm_args!(@read $vm, $sp + 32, $t8);
        let v9 = $crate::vm_args!(@read $vm, $sp + 36, $t9);
        let v10 = $crate::vm_args!(@read $vm, $sp + 40, $t10);
        let v11 = $crate::vm_args!(@read $vm, $sp + 44, $t11);
        let v12 = $crate::vm_args!(@read $vm, $sp + 48, $t12);
        (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    }};
}

/// Macro to read a string with specified length from a pointer on the stack.
///
/// # Syntax
///
/// ```ignore
/// // ANSI string with length
/// let text = vm_args_str_len!(vm, stack_ptr; len);
///
/// // Wide string with length (in characters)
/// let text = vm_args_wstr_len!(vm, stack_ptr; len);
/// ```
///
/// The first argument at stack_ptr+4 is the buffer pointer,
/// and `len` is the number of characters to read.
#[macro_export]
macro_rules! vm_args_str_len {
    ($vm:expr, $sp:expr; $len:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_str_len($vm, ptr, $len)
    }};
}

/// Macro to read a wide string with specified length from a pointer on the stack.
#[macro_export]
macro_rules! vm_args_wstr_len {
    ($vm:expr, $sp:expr; $len:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_wstr_len($vm, ptr, $len)
    }};
}

/// Macro to read a string until delimiter from a pointer on the stack.
///
/// # Syntax
///
/// ```ignore
/// // ANSI string with default null terminator
/// let text = vm_args_str_delim!(vm, stack_ptr);
///
/// // ANSI string with custom delimiter
/// let text = vm_args_str_delim!(vm, stack_ptr; b'\n');
/// ```
#[macro_export]
macro_rules! vm_args_str_delim {
    ($vm:expr, $sp:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_str_delim($vm, ptr, 0)
    }};
    ($vm:expr, $sp:expr; $delim:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_str_delim($vm, ptr, $delim)
    }};
}

/// Macro to read a wide string until delimiter from a pointer on the stack.
///
/// # Syntax
///
/// ```ignore
/// // Wide string with default null terminator
/// let text = vm_args_wstr_delim!(vm, stack_ptr);
///
/// // Wide string with custom delimiter
/// let text = vm_args_wstr_delim!(vm, stack_ptr; 0x000A); // newline
/// ```
#[macro_export]
macro_rules! vm_args_wstr_delim {
    ($vm:expr, $sp:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_wstr_delim($vm, ptr, 0)
    }};
    ($vm:expr, $sp:expr; $delim:expr) => {{
        let ptr = $vm.read_u32($sp + 4).unwrap_or(0);
        $crate::vm::windows::macros::read_wstr_delim($vm, ptr, $delim)
    }};
}

/// Macro to generate stub functions for not-yet-implemented Windows API functions.
///
/// # Usage
///
/// ```ignore
/// define_stub_fn!(DLL_NAME, function_name, return_value);
/// ```
///
/// This generates a function that calls `check_stub()` and returns the specified value.
/// The function name is used as-is for the Rust function and stringified for logging.
///
/// # Examples
///
/// ```ignore
/// use crate::vm::windows::kernel32::DLL_NAME;
/// define_stub_fn!(DLL_NAME, create_event_w, 1);  // Returns 1 (success)
/// define_stub_fn!(DLL_NAME, get_version, 0);     // Returns 0
/// ```
#[macro_export]
macro_rules! define_stub_fn {
    ($dll:expr, $name:ident, $ret:expr) => {
        fn $name(vm: &mut $crate::vm::Vm, _sp: u32) -> u32 {
            $crate::vm::windows::check_stub(vm, $dll, stringify!($name));
            $ret
        }
    };
}

/// Wrapper for wide (UTF-16) string arguments in vm_set_args!.
pub struct VmWideStr<'a>(pub &'a str);

impl<'a> VmWideStr<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value)
    }
}

/// Helper macro to mark a UTF-16 string argument for vm_set_args!.
#[macro_export]
macro_rules! vm_wstr {
    ($value:expr) => {
        $crate::vm::windows::macros::VmWideStr::new($value)
    };
}

pub(crate) trait VmSetArg {
    fn write(self, vm: &mut Vm, offset: u32);
}

impl Vm {
    pub(crate) fn write_arg<T: VmSetArg>(&mut self, offset: u32, value: T) {
        value.write(self, offset);
    }
}

fn alloc_c_string(vm: &mut Vm, value: &str) -> u32 {
    let mut bytes = value.as_bytes().to_vec();
    bytes.push(0);
    vm.alloc_bytes(&bytes, 1).unwrap()
}

fn alloc_wide_string(vm: &mut Vm, value: &str) -> u32 {
    let mut bytes = Vec::new();
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0u16.to_le_bytes());
    vm.alloc_bytes(&bytes, 2).unwrap()
}

impl VmSetArg for u32 {
    fn write(self, vm: &mut Vm, offset: u32) {
        vm.write_u32(offset, self).unwrap();
    }
}

impl VmSetArg for i32 {
    fn write(self, vm: &mut Vm, offset: u32) {
        vm.write_u32(offset, self as u32).unwrap();
    }
}

impl VmSetArg for u16 {
    fn write(self, vm: &mut Vm, offset: u32) {
        vm.write_u32(offset, self as u32).unwrap();
    }
}

impl VmSetArg for usize {
    fn write(self, vm: &mut Vm, offset: u32) {
        vm.write_u32(offset, self as u32).unwrap();
    }
}

impl VmSetArg for &str {
    fn write(self, vm: &mut Vm, offset: u32) {
        let ptr = alloc_c_string(vm, self);
        vm.write_u32(offset, ptr).unwrap();
    }
}

impl VmSetArg for String {
    fn write(self, vm: &mut Vm, offset: u32) {
        let ptr = alloc_c_string(vm, &self);
        vm.write_u32(offset, ptr).unwrap();
    }
}

impl<'a> VmSetArg for &'a [u8] {
    fn write(self, vm: &mut Vm, offset: u32) {
        let ptr = vm.alloc_bytes(self, 1).unwrap();
        vm.write_u32(offset, ptr).unwrap();
    }
}

impl VmSetArg for Vec<u8> {
    fn write(self, vm: &mut Vm, offset: u32) {
        let ptr = vm.alloc_bytes(&self, 1).unwrap();
        vm.write_u32(offset, ptr).unwrap();
    }
}

impl<'a> VmSetArg for VmWideStr<'a> {
    fn write(self, vm: &mut Vm, offset: u32) {
        let ptr = alloc_wide_string(vm, self.0);
        vm.write_u32(offset, ptr).unwrap();
    }
}

/// Macro to write function arguments to the stack (for testing).
///
/// # Syntax
///
/// ```ignore
/// vm_set_args!(vm, stack_ptr; val1, val2, ...)
/// ```
///
/// This writes u32 values to stack_ptr+4, stack_ptr+8, etc.
/// For ANSI strings, pass a string slice or &[u8] to write into the heap and
/// place its pointer on the stack. Use `vm_wstr!()` for UTF-16 strings.
///
/// # Examples
///
/// ```ignore
/// // Set up 3 arguments
/// vm_set_args!(vm, stack; 0x1234, 0x5678, 42);
///
/// // With string (writes to heap and puts pointer on stack)
/// vm_set_args!(vm, stack; "Hello", 42);
/// ```
#[macro_export]
macro_rules! vm_set_args {
    // Internal: write single value
    (@write $vm:expr, $offset:expr, $val:expr) => {
        $vm.write_arg($offset, $val);
    };

    // 1 argument
    ($vm:expr, $sp:expr; $v1:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
    }};

    // 2 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
    }};

    // 3 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
    }};

    // 4 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr, $v4:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
        $crate::vm_set_args!(@write $vm, $sp + 16, $v4);
    }};

    // 5 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
        $crate::vm_set_args!(@write $vm, $sp + 16, $v4);
        $crate::vm_set_args!(@write $vm, $sp + 20, $v5);
    }};

    // 6 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
        $crate::vm_set_args!(@write $vm, $sp + 16, $v4);
        $crate::vm_set_args!(@write $vm, $sp + 20, $v5);
        $crate::vm_set_args!(@write $vm, $sp + 24, $v6);
    }};

    // 7 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
        $crate::vm_set_args!(@write $vm, $sp + 16, $v4);
        $crate::vm_set_args!(@write $vm, $sp + 20, $v5);
        $crate::vm_set_args!(@write $vm, $sp + 24, $v6);
        $crate::vm_set_args!(@write $vm, $sp + 28, $v7);
    }};

    // 8 arguments
    ($vm:expr, $sp:expr; $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr, $v7:expr, $v8:expr) => {{
        $crate::vm_set_args!(@write $vm, $sp + 4, $v1);
        $crate::vm_set_args!(@write $vm, $sp + 8, $v2);
        $crate::vm_set_args!(@write $vm, $sp + 12, $v3);
        $crate::vm_set_args!(@write $vm, $sp + 16, $v4);
        $crate::vm_set_args!(@write $vm, $sp + 20, $v5);
        $crate::vm_set_args!(@write $vm, $sp + 24, $v6);
        $crate::vm_set_args!(@write $vm, $sp + 28, $v7);
        $crate::vm_set_args!(@write $vm, $sp + 32, $v8);
    }};
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

    #[test]
    fn test_vm_args_single_u32() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; 0x1234u32);
        let (value,) = vm_args!(&vm, stack; u32);
        assert_eq!(value, 0x1234);
    }

    #[test]
    fn test_vm_args_four_u32() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 20;
        vm_set_args!(vm, stack; 0xAAAAu32, 0xBBBBu32, 0xCCCCu32, 0xDDDDu32);
        let (a, b, c, d) = vm_args!(&vm, stack; u32, u32, u32, u32);
        assert_eq!(a, 0xAAAA);
        assert_eq!(b, 0xBBBB);
        assert_eq!(c, 0xCCCC);
        assert_eq!(d, 0xDDDD);
    }

    #[test]
    fn test_vm_args_mixed_types() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 16;
        vm_set_args!(vm, stack; 0x1234u32, -1i32, 42usize);
        let (handle, signed, index) = vm_args!(&vm, stack; u32, i32, usize);
        assert_eq!(handle, 0x1234);
        assert_eq!(signed, -1);
        assert_eq!(index, 42usize);
    }

    #[test]
    fn test_vm_args_u16() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; 0x1234u16);
        let (value,) = vm_args!(&vm, stack; u16);
        assert_eq!(value, 0x1234u16);
    }

    #[test]
    fn test_vm_args_str() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 12;
        vm_set_args!(vm, stack; "Hello", 42u32);
        let (s, n) = vm_args!(&vm, stack; str, u32);
        assert_eq!(s, "Hello");
        assert_eq!(n, 42);
    }

    #[test]
    fn test_vm_args_wstr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; vm_wstr!("Hi"));
        let (s,) = vm_args!(&vm, stack; wstr);
        assert_eq!(s, "Hi");
    }

    #[test]
    fn test_vm_args_null_str() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; 0u32); // NULL pointer
        let (s,) = vm_args!(&vm, stack; str);
        assert_eq!(s, "");
    }

    #[test]
    fn test_vm_args_null_wstr() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; 0u32); // NULL pointer
        let (s,) = vm_args!(&vm, stack; wstr);
        assert_eq!(s, "");
    }

    #[test]
    fn test_read_str_arg_basic() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_bytes(ptr, b"Test\0").unwrap();
        assert_eq!(read_str_arg(&vm, ptr), "Test");
    }

    #[test]
    fn test_read_str_arg_null() {
        let vm = create_test_vm();
        assert_eq!(read_str_arg(&vm, 0), "");
    }

    #[test]
    fn test_read_wstr_arg_basic() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write L"ABC\0"
        vm.write_u16(ptr, 'A' as u16).unwrap();
        vm.write_u16(ptr + 2, 'B' as u16).unwrap();
        vm.write_u16(ptr + 4, 'C' as u16).unwrap();
        vm.write_u16(ptr + 6, 0).unwrap();
        assert_eq!(read_wstr_arg(&vm, ptr), "ABC");
    }

    #[test]
    fn test_read_wstr_arg_null() {
        let vm = create_test_vm();
        assert_eq!(read_wstr_arg(&vm, 0), "");
    }

    #[test]
    fn test_read_str_len() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_bytes(ptr, b"Hello World").unwrap();
        assert_eq!(read_str_len(&vm, ptr, 5), "Hello");
        assert_eq!(read_str_len(&vm, ptr, 11), "Hello World");
    }

    #[test]
    fn test_read_str_len_null() {
        let vm = create_test_vm();
        assert_eq!(read_str_len(&vm, 0, 5), "");
    }

    #[test]
    fn test_read_str_len_zero() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_bytes(ptr, b"Hello").unwrap();
        assert_eq!(read_str_len(&vm, ptr, 0), "");
    }

    #[test]
    fn test_read_wstr_len() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write L"Hello" (no null terminator needed)
        vm.write_u16(ptr, 'H' as u16).unwrap();
        vm.write_u16(ptr + 2, 'e' as u16).unwrap();
        vm.write_u16(ptr + 4, 'l' as u16).unwrap();
        vm.write_u16(ptr + 6, 'l' as u16).unwrap();
        vm.write_u16(ptr + 8, 'o' as u16).unwrap();
        assert_eq!(read_wstr_len(&vm, ptr, 5), "Hello");
        assert_eq!(read_wstr_len(&vm, ptr, 3), "Hel");
    }

    #[test]
    fn test_read_wstr_len_null() {
        let vm = create_test_vm();
        assert_eq!(read_wstr_len(&vm, 0, 5), "");
    }

    #[test]
    fn test_read_str_delim_null_term() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_bytes(ptr, b"Hello\0World").unwrap();
        assert_eq!(read_str_delim(&vm, ptr, 0), "Hello");
    }

    #[test]
    fn test_read_str_delim_custom() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        vm.write_bytes(ptr, b"Hello\nWorld").unwrap();
        assert_eq!(read_str_delim(&vm, ptr, b'\n'), "Hello");
    }

    #[test]
    fn test_read_str_delim_null_ptr() {
        let vm = create_test_vm();
        assert_eq!(read_str_delim(&vm, 0, 0), "");
    }

    #[test]
    fn test_read_wstr_delim_null_term() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write L"Hi\0X"
        vm.write_u16(ptr, 'H' as u16).unwrap();
        vm.write_u16(ptr + 2, 'i' as u16).unwrap();
        vm.write_u16(ptr + 4, 0).unwrap();
        vm.write_u16(ptr + 6, 'X' as u16).unwrap();
        assert_eq!(read_wstr_delim(&vm, ptr, 0), "Hi");
    }

    #[test]
    fn test_read_wstr_delim_custom() {
        let mut vm = create_test_vm();
        let ptr = vm.heap_start as u32;
        // Write L"Hi|X"
        vm.write_u16(ptr, 'H' as u16).unwrap();
        vm.write_u16(ptr + 2, 'i' as u16).unwrap();
        vm.write_u16(ptr + 4, '|' as u16).unwrap();
        vm.write_u16(ptr + 6, 'X' as u16).unwrap();
        assert_eq!(read_wstr_delim(&vm, ptr, '|' as u16), "Hi");
    }

    #[test]
    fn test_read_wstr_delim_null_ptr() {
        let vm = create_test_vm();
        assert_eq!(read_wstr_delim(&vm, 0, 0), "");
    }

    #[test]
    fn test_vm_args_str_len_macro() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; "Hello World");
        let text = vm_args_str_len!(&vm, stack; 5);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_vm_args_wstr_len_macro() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; vm_wstr!("Test"));
        let text = vm_args_wstr_len!(&vm, stack; 4);
        assert_eq!(text, "Test");
    }

    #[test]
    fn test_vm_args_str_delim_macro_default() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; "Hello\0World");
        let text = vm_args_str_delim!(&vm, stack);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_vm_args_str_delim_macro_custom() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; "Line1\nLine2");
        let text = vm_args_str_delim!(&vm, stack; b'\n');
        assert_eq!(text, "Line1");
    }

    #[test]
    fn test_vm_args_wstr_delim_macro_default() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; vm_wstr!("OK"));
        let text = vm_args_wstr_delim!(&vm, stack);
        assert_eq!(text, "OK");
    }

    #[test]
    fn test_vm_args_wstr_delim_macro_custom() {
        let mut vm = create_test_vm();
        let stack = vm.stack_top - 8;
        vm_set_args!(vm, stack; vm_wstr!("A;B"));
        let text = vm_args_wstr_delim!(&vm, stack; ';' as u16);
        assert_eq!(text, "A");
    }
}
