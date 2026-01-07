//! Heuristics for recovering COM objects from in-memory layouts.

use crate::pe::PeFile;
use crate::vm::Vm;

pub(super) fn recover_dispatch_from_heap(
    vm: &Vm,
    file: &PeFile,
    internal_create: u32,
) -> Option<u32> {
    let vtable = find_vtable_from_internal_create(vm, file, internal_create)?;
    if std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] recover IDispatch: vtable=0x{vtable:08X} heap_allocs={}",
            vm.heap_allocs.len()
        );
    }
    for (&ptr, &size) in vm.heap_allocs.iter() {
        if size < 4 {
            continue;
        }
        if vm.read_u32(ptr).ok()? == vtable {
            return Some(ptr);
        }
    }
    let base = vm.base();
    let start = vm.heap_start;
    let end = vm.heap_end;
    let mut offset = start;
    while offset + 4 <= end {
        let addr = base.wrapping_add(offset as u32);
        if vm.read_u32(addr).ok()? == vtable {
            return Some(addr);
        }
        offset += 1;
    }
    for section in &file.sections {
        if (section.characteristics & 0x8000_0000) == 0 {
            continue;
        }
        let size = section.virtual_size.max(section.raw_size) as usize;
        let start = section.virtual_address as usize;
        let end = start.saturating_add(size).min(vm.memory.len());
        let mut offset = start;
        while offset + 4 <= end {
            let addr = base.wrapping_add(offset as u32);
            if vm.read_u32(addr).ok()? == vtable {
                return Some(addr);
            }
            offset += 1;
        }
    }
    None
}

fn find_vtable_from_internal_create(vm: &Vm, file: &PeFile, internal_create: u32) -> Option<u32> {
    if internal_create == 0 {
        return None;
    }
    let code_start = vm.base().wrapping_add(file.optional_header.base_of_code);
    let code_end = code_start.wrapping_add(file.optional_header.size_of_code);
    let mut candidates = vec![internal_create];
    let mut stub = [0u8; 64];
    for (idx, slot) in stub.iter_mut().enumerate() {
        *slot = vm.read_u8(internal_create.wrapping_add(idx as u32)).ok()?;
    }
    for idx in 0..stub.len().saturating_sub(5) {
        if stub[idx] == 0xE9 {
            let rel =
                i32::from_le_bytes([stub[idx + 1], stub[idx + 2], stub[idx + 3], stub[idx + 4]]);
            let target = internal_create
                .wrapping_add(idx as u32 + 5)
                .wrapping_add(rel as u32);
            candidates.push(target);
        } else if stub[idx] == 0xEB {
            let rel = stub[idx + 1] as i8 as i32;
            let target = internal_create
                .wrapping_add(idx as u32 + 2)
                .wrapping_add(rel as u32);
            candidates.push(target);
        }
    }
    if std::env::var("PE_VM_TRACE").is_ok() {
        let list = candidates
            .iter()
            .map(|addr| format!("0x{addr:08X}"))
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!("[pe_vm] internal_create scan targets: {list}");
    }

    for target in candidates {
        let mut window = [0u8; 256];
        for (idx, slot) in window.iter_mut().enumerate() {
            *slot = vm.read_u8(target.wrapping_add(idx as u32)).ok()?;
        }
        for idx in 0..window.len().saturating_sub(6) {
            if window[idx] != 0xC7 {
                continue;
            }
            let modrm = window[idx + 1];
            if (modrm & 0xC0) != 0x00 {
                continue;
            }
            if ((modrm >> 3) & 0x7) != 0 {
                continue;
            }
            let imm = u32::from_le_bytes([
                window[idx + 2],
                window[idx + 3],
                window[idx + 4],
                window[idx + 5],
            ]);
            if !vm.contains_addr(imm) {
                continue;
            }
            let first = vm.read_u32(imm).ok()?;
            if first < code_start || first >= code_end {
                continue;
            }
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] internal_create vtable candidate 0x{imm:08X} first=0x{first:08X}"
                );
            }
            return Some(imm);
        }
    }
    None
}
