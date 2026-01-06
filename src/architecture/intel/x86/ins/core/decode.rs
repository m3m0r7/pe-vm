use crate::vm::{Vm, VmError};

#[derive(Default, Clone, Copy)]
pub(crate) struct Prefixes {
    pub(crate) segment_base: u32,
    pub(crate) operand_size_16: bool,
    pub(crate) rep: bool,
    pub(crate) repne: bool,
}

pub(crate) fn parse_prefixes(vm: &Vm, cursor: u32) -> Result<(u32, Prefixes), VmError> {
    let mut cursor = cursor;
    let mut prefixes = Prefixes::default();
    loop {
        let byte = vm.read_u8(cursor)?;
        match byte {
            0xF0 | 0xF2 | 0xF3 => {
                if byte == 0xF2 {
                    prefixes.repne = true;
                    prefixes.rep = false;
                } else if byte == 0xF3 {
                    prefixes.rep = true;
                    prefixes.repne = false;
                }
                cursor = cursor.wrapping_add(1);
            }
            0x64 => {
                prefixes.segment_base = vm.fs_base();
                cursor = cursor.wrapping_add(1);
            }
            0x65 => {
                prefixes.segment_base = vm.gs_base();
                cursor = cursor.wrapping_add(1);
            }
            0x66 => {
                prefixes.operand_size_16 = true;
                cursor = cursor.wrapping_add(1);
            }
            _ => break,
        }
    }
    Ok((cursor, prefixes))
}

#[derive(Debug, Clone)]
pub(crate) struct ModRm {
    pub(crate) mod_bits: u8,
    pub(crate) reg: u8,
    pub(crate) rm: u8,
    pub(crate) disp: i32,
    pub(crate) sib: Option<Sib>,
    pub(crate) len: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Sib {
    pub(crate) scale: u8,
    pub(crate) index: u8,
    pub(crate) base: u8,
}

pub(crate) fn decode_modrm(vm: &Vm, addr: u32) -> Result<ModRm, VmError> {
    let modrm = vm.read_u8(addr)?;
    let mod_bits = (modrm >> 6) & 0x3;
    let reg = (modrm >> 3) & 0x7;
    let rm = modrm & 0x7;
    let mut len = 1usize;
    let mut sib = None;
    let mut disp = 0i32;

    if mod_bits != 3 && rm == 4 {
        let sib_byte = vm.read_u8(addr + len as u32)?;
        len += 1;
        sib = Some(Sib {
            scale: (sib_byte >> 6) & 0x3,
            index: (sib_byte >> 3) & 0x7,
            base: sib_byte & 0x7,
        });
    }

    match mod_bits {
        0 => {
            if rm == 5 {
                disp = vm.read_u32(addr + len as u32)? as i32;
                len += 4;
            } else if let Some(sib_val) = &sib {
                if sib_val.base == 5 {
                    disp = vm.read_u32(addr + len as u32)? as i32;
                    len += 4;
                }
            }
        }
        1 => {
            disp = vm.read_u8(addr + len as u32)? as i8 as i32;
            len += 1;
        }
        2 => {
            disp = vm.read_u32(addr + len as u32)? as i32;
            len += 4;
        }
        _ => {}
    }

    Ok(ModRm {
        mod_bits,
        reg,
        rm,
        disp,
        sib,
        len,
    })
}

pub(crate) fn calc_ea(vm: &Vm, modrm: &ModRm, segment_base: u32) -> Result<u32, VmError> {
    if modrm.mod_bits == 3 {
        return Err(VmError::UnsupportedInstruction(0));
    }
    let mut base = 0u32;
    if let Some(sib) = &modrm.sib {
        if sib.index != 4 {
            let index = vm.reg32(sib.index);
            base = base.wrapping_add(index << sib.scale);
        }
        if !(modrm.mod_bits == 0 && sib.base == 5) {
            base = base.wrapping_add(vm.reg32(sib.base));
        }
    } else if !(modrm.mod_bits == 0 && modrm.rm == 5) {
        base = base.wrapping_add(vm.reg32(modrm.rm));
    }

    Ok(segment_base.wrapping_add(base).wrapping_add(modrm.disp as u32))
}
