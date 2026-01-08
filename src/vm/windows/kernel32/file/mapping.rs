//! File mapping API stubs (CreateFileMappingA, MapViewOfFile, etc.)

use crate::vm::Vm;
use crate::vm_args;

use super::constants::INVALID_HANDLE_VALUE;

pub(super) fn register(vm: &mut Vm) {
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CreateFileMappingA",
        crate::vm::stdcall_args(6),
        create_file_mapping_a,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "CreateFileMappingW",
        crate::vm::stdcall_args(6),
        create_file_mapping_w,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "MapViewOfFile",
        crate::vm::stdcall_args(5),
        map_view_of_file,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "MapViewOfFileEx",
        crate::vm::stdcall_args(6),
        map_view_of_file_ex,
    );
    vm.register_import_stdcall(
        "KERNEL32.dll",
        "UnmapViewOfFile",
        crate::vm::stdcall_args(1),
        unmap_view_of_file,
    );
}

/// CreateFileMappingA - Creates or opens a named or unnamed file mapping object
///
/// HANDLE CreateFileMappingA(
///   HANDLE hFile,                    // Handle to file (or INVALID_HANDLE_VALUE for pagefile)
///   LPSECURITY_ATTRIBUTES lpAttr,    // Security attributes (can be NULL)
///   DWORD flProtect,                 // Page protection
///   DWORD dwMaximumSizeHigh,         // High-order DWORD of max size
///   DWORD dwMaximumSizeLow,          // Low-order DWORD of max size
///   LPCSTR lpName                    // Name of mapping object (can be NULL)
/// );
fn create_file_mapping_a(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (file_handle, _security_attrs, _protect, size_high, size_low, _name_ptr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] CreateFileMappingA: handle=0x{:08X} size_high={} size_low={}",
            file_handle, size_high, size_low
        );
    }

    // Get file size if handle is valid
    let size = if file_handle != INVALID_HANDLE_VALUE && file_handle != 0 {
        vm.file_size(file_handle).unwrap_or(0) as usize
    } else {
        // Page file backed mapping
        ((size_high as usize) << 32) | (size_low as usize)
    };

    if size == 0 {
        // Empty file or invalid - return failure
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] CreateFileMappingA: file is empty, returning NULL");
        }
        return 0;
    }

    // Allocate mapping handle
    match vm.create_file_mapping(file_handle, size) {
        Some(mapping_handle) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] CreateFileMappingA: created mapping handle=0x{:08X} size={}",
                    mapping_handle, size
                );
            }
            mapping_handle
        }
        None => 0,
    }
}

fn create_file_mapping_w(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // Same implementation as A version
    create_file_mapping_a(vm, stack_ptr)
}

/// MapViewOfFile - Maps a view of a file mapping into the address space
///
/// LPVOID MapViewOfFile(
///   HANDLE hFileMappingObject,  // Handle to file mapping object
///   DWORD dwDesiredAccess,      // Access mode
///   DWORD dwFileOffsetHigh,     // High-order DWORD of offset
///   DWORD dwFileOffsetLow,      // Low-order DWORD of offset
///   SIZE_T dwNumberOfBytesToMap // Number of bytes to map (0 = entire file)
/// );
fn map_view_of_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (mapping_handle, _access, offset_high, offset_low, bytes_to_map) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32);

    let offset = ((offset_high as u64) << 32) | (offset_low as u64);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] MapViewOfFile: mapping=0x{:08X} offset={} bytes={}",
            mapping_handle, offset, bytes_to_map
        );
    }

    match vm.map_view_of_file(mapping_handle, offset as usize, bytes_to_map as usize) {
        Some(addr) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] MapViewOfFile: mapped at 0x{:08X}", addr);
            }
            addr
        }
        None => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] MapViewOfFile: failed to map");
            }
            0
        }
    }
}

fn map_view_of_file_ex(vm: &mut Vm, stack_ptr: u32) -> u32 {
    // MapViewOfFileEx has an extra parameter for base address hint
    let (mapping_handle, _access, offset_high, offset_low, bytes_to_map, _base_addr) =
        vm_args!(vm, stack_ptr; u32, u32, u32, u32, u32, u32);

    let offset = ((offset_high as u64) << 32) | (offset_low as u64);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] MapViewOfFileEx: mapping=0x{:08X} offset={} bytes={}",
            mapping_handle, offset, bytes_to_map
        );
    }

    match vm.map_view_of_file(mapping_handle, offset as usize, bytes_to_map as usize) {
        Some(addr) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] MapViewOfFileEx: mapped at 0x{:08X}", addr);
            }
            addr
        }
        None => 0,
    }
}

/// UnmapViewOfFile - Unmaps a mapped view of a file
fn unmap_view_of_file(vm: &mut Vm, stack_ptr: u32) -> u32 {
    let (base_addr,) = vm_args!(vm, stack_ptr; u32);

    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] UnmapViewOfFile: addr=0x{:08X}", base_addr);
    }

    vm.unmap_view_of_file(base_addr);
    1 // Success
}
