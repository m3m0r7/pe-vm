use crate::vm::{Vm, VmError};

use crate::architecture::intel::x86::ins::core::{calc_ea, ModRm, Prefixes};

pub(crate) fn jmp_rel32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let rel = vm.read_u32(cursor + 1)? as i32;
    let next = cursor + 5;
    vm.set_eip((next as i32).wrapping_add(rel) as u32);
    Ok(())
}

pub(crate) fn jmp_rel8(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let rel = vm.read_u8(cursor + 1)? as i8 as i32;
    let next = cursor + 2;
    vm.set_eip((next as i32).wrapping_add(rel) as u32);
    Ok(())
}

pub(crate) fn jmp_rm32(vm: &mut Vm, modrm: &ModRm, prefixes: Prefixes) -> Result<(), VmError> {
    if modrm.mod_bits == 3 {
        let target = vm.reg32(modrm.rm);
        if !vm.try_jump_import(target)? {
            if !vm.contains_addr(target) && std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] jmp_rm32 target outside vm: target=0x{target:08X}");
            }
            vm.set_eip(target);
        }
        return Ok(());
    }

    let addr = calc_ea(vm, modrm, prefixes.segment_base)?;
    if vm.try_jump_import(addr)? {
        return Ok(());
    }
    let target = vm.read_u32(addr)?;
    if !vm.try_jump_import(target)? {
        if !vm.contains_addr(target) && std::env::var("PE_VM_TRACE").is_ok() {
            eprintln!("[pe_vm] jmp_rm32 target outside vm: target=0x{target:08X} mem=0x{addr:08X}");
        }
        vm.set_eip(target);
    }
    Ok(())
}
