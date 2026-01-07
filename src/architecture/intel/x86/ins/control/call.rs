use crate::vm::{Vm, VmError, REG_EAX, REG_ECX, REG_EDX, REG_ESP};

use crate::architecture::intel::x86::ins::core::{calc_ea, ModRm, Prefixes};

pub(crate) fn call_rel32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let rel = vm.read_u32(cursor + 1)? as i32;
    let next = cursor + 5;
    let target = (next as i32).wrapping_add(rel) as u32;
    if target == 0 && std::env::var("PE_VM_ABORT_ON_NULL_CALL").is_ok() {
        if std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] null call_rel32 target at eip=0x{cursor:08X} next=0x{next:08X}");
        }
        return Err(VmError::InvalidConfig("null call"));
    }
    if !vm.try_call_import(target, next)? {
        vm.push(next)?;
        vm.set_eip(target);
    }
    Ok(())
}

pub(crate) fn call_rm32(
    vm: &mut Vm,
    modrm: &ModRm,
    prefixes: Prefixes,
    next: u32,
) -> Result<(), VmError> {
    if modrm.mod_bits == 3 {
        let target = vm.reg32(modrm.rm);
        if std::env::var("PE_VM_TRACE_CALLS").is_ok() {
            let mut line = format!(
                "[pe_vm] call_rm32 target=0x{target:08X} next=0x{next:08X} esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
                vm.reg32(REG_ESP),
                vm.reg32(REG_EAX),
                vm.reg32(REG_ECX),
                vm.reg32(REG_EDX)
            );
            for idx in 0..5 {
                let addr = vm.reg32(REG_ESP).wrapping_add((idx * 4) as u32);
                let value = vm.read_u32(addr).unwrap_or(0);
                line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
            }
            eprintln!("{line}");
        }
        if target == 0 && std::env::var("PE_VM_ABORT_ON_NULL_CALL").is_ok() {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] null call_rm32 target reg at next=0x{next:08X}");
            }
            return Err(VmError::InvalidConfig("null call"));
        }
        if !vm.try_call_import(target, next)? {
            if !vm.contains_addr(target) && std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] call_rm32 target outside vm: target=0x{target:08X} next=0x{next:08X}"
                );
            }
            vm.push(next)?;
            vm.set_eip(target);
        }
        return Ok(());
    }

    let mem_addr = calc_ea(vm, modrm, prefixes.segment_base)?;
    if !vm.try_call_import(mem_addr, next)? {
        let target = vm.read_u32(mem_addr)?;
        if std::env::var("PE_VM_TRACE_CALLS").is_ok() {
            let mut line = format!(
                "[pe_vm] call_rm32 target=0x{target:08X} mem=0x{mem_addr:08X} next=0x{next:08X} esp=0x{:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X}",
                vm.reg32(REG_ESP),
                vm.reg32(REG_EAX),
                vm.reg32(REG_ECX),
                vm.reg32(REG_EDX)
            );
            for idx in 0..5 {
                let addr = vm.reg32(REG_ESP).wrapping_add((idx * 4) as u32);
                let value = vm.read_u32(addr).unwrap_or(0);
                line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
            }
            eprintln!("{line}");
        }
        if target == 0 && std::env::var("PE_VM_ABORT_ON_NULL_CALL").is_ok() {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] null call_rm32 target mem=0x{mem_addr:08X} next=0x{next:08X}");
            }
            return Err(VmError::InvalidConfig("null call"));
        }
        if !vm.try_call_import(target, next)? {
            if !vm.contains_addr(target) && std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!(
                    "[pe_vm] call_rm32 target outside vm: target=0x{target:08X} mem=0x{mem_addr:08X} next=0x{next:08X}"
                );
            }
            vm.push(next)?;
            vm.set_eip(target);
        }
    }
    Ok(())
}
