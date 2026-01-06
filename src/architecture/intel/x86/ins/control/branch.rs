use crate::vm::{Vm, VmError, REG_AL, REG_EDX, REG_EDI};

use crate::architecture::intel::x86::ins::core::{
    decode_modrm, read_rm16, read_rm32, write_rm8, Prefixes,
};

pub(crate) fn jcc_rel8(
    vm: &mut Vm,
    cursor: u32,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let opcode = vm.read_u8(cursor)?;
    let cond = condition(vm, opcode).ok_or(VmError::UnsupportedInstruction(opcode))?;
    let rel = vm.read_u8(cursor + 1)? as i8 as i32;
    let next = cursor + 2;
    if opcode == 0x74 {
        let pattern = [
            0x88u8, 0x07, 0x83, 0xC7, 0x01, 0x83, 0xEA, 0x01, 0x75, 0xF6,
        ];
        let mut matches = true;
        for (idx, byte) in pattern.iter().enumerate() {
            if vm.read_u8(cursor + 2 + idx as u32).unwrap_or(0) != *byte {
                matches = false;
                break;
            }
        }
        if matches {
            let count = vm.reg32(REG_EDX);
            if count == 0 {
                if std::env::var("PE_VM_TRACE_MEM").is_ok() {
                    eprintln!(
                        "[pe_vm] fast memset skip loop at 0x{cursor:08X} count=0 dst=0x{:08X} value=0x{:02X}",
                        vm.reg32(REG_EDI),
                        vm.reg8(REG_AL)
                    );
                }
                vm.set_flags(true, false, false, false);
                vm.set_eip((next as i32).wrapping_add(rel) as u32);
                return Ok(());
            }
            if count > 0 {
                let dst = vm.reg32(REG_EDI);
                let value = vm.reg8(REG_AL);
                if std::env::var("PE_VM_TRACE_MEM").is_ok() {
                    eprintln!(
                        "[pe_vm] fast memset at 0x{cursor:08X} dst=0x{dst:08X} count=0x{count:08X} value=0x{value:02X}"
                    );
                }
                vm.memset(dst, value, count as usize)?;
                vm.set_reg32(REG_EDI, dst.wrapping_add(count));
                vm.set_reg32(REG_EDX, 0);
                vm.set_flags(true, false, false, false);
                vm.set_eip((next as i32).wrapping_add(rel) as u32);
                return Ok(());
            }
        }
    }
    if cond {
        vm.set_eip((next as i32).wrapping_add(rel) as u32);
    } else {
        vm.set_eip(next);
    }
    Ok(())
}

pub(crate) fn jcc_rel32_ext(
    vm: &mut Vm,
    cursor: u32,
    opcode: u8,
    _prefixes: Prefixes,
) -> Result<(), VmError> {
    let cond = condition(vm, opcode).ok_or(VmError::UnsupportedInstruction(opcode))?;
    let rel = vm.read_u32(cursor + 2)? as i32;
    let next = cursor + 6;
    if cond {
        vm.set_eip((next as i32).wrapping_add(rel) as u32);
    } else {
        vm.set_eip(next);
    }
    Ok(())
}

pub(crate) fn setcc(
    vm: &mut Vm,
    cursor: u32,
    ext: u8,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    let cond = condition(vm, ext.wrapping_sub(0x20))
        .ok_or(VmError::UnsupportedInstruction(ext))?;
    let value = if cond { 1 } else { 0 };
    write_rm8(vm, &modrm, prefixes.segment_base, value)?;
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

pub(crate) fn cmovcc(
    vm: &mut Vm,
    cursor: u32,
    ext: u8,
    prefixes: Prefixes,
) -> Result<(), VmError> {
    let cond = condition(vm, ext.wrapping_add(0x30))
        .ok_or(VmError::UnsupportedInstruction(ext))?;
    if ext == 0x49 && std::env::var("PE_VM_TRACE_CMOV").is_ok() {
        eprintln!(
            "[pe_vm] cmovns at 0x{cursor:08X} cond={cond} sf={} eax=0x{:08X} esi=0x{:08X}",
            vm.sf(),
            vm.reg32(crate::vm::REG_EAX),
            vm.reg32(crate::vm::REG_ESI)
        );
    }
    let modrm = decode_modrm(vm, cursor + 2)?;
    if cond {
        if prefixes.operand_size_16 {
            let value = read_rm16(vm, &modrm, prefixes.segment_base)?;
            vm.set_reg16(modrm.reg, value);
        } else {
            let value = read_rm32(vm, &modrm, prefixes.segment_base)?;
            vm.set_reg32(modrm.reg, value);
        }
    }
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}

fn condition(vm: &Vm, opcode: u8) -> Option<bool> {
    match opcode {
        0x70 | 0x80 => Some(vm.of()),
        0x71 | 0x81 => Some(!vm.of()),
        0x72 | 0x82 => Some(vm.cf()),
        0x73 | 0x83 => Some(!vm.cf()),
        0x74 | 0x84 => Some(vm.zf()),
        0x75 | 0x85 => Some(!vm.zf()),
        0x76 | 0x86 => Some(vm.cf() || vm.zf()),
        0x77 | 0x87 => Some(!vm.cf() && !vm.zf()),
        0x78 | 0x88 => Some(vm.sf()),
        0x79 | 0x89 => Some(!vm.sf()),
        0x7A | 0x8A => Some(false),
        0x7B | 0x8B => Some(true),
        0x7C | 0x8C => Some(vm.sf() != vm.of()),
        0x7D | 0x8D => Some(vm.sf() == vm.of()),
        0x7E | 0x8E => Some(vm.zf() || (vm.sf() != vm.of())),
        0x7F | 0x8F => Some(!vm.zf() && (vm.sf() == vm.of())),
        _ => None,
    }
}
