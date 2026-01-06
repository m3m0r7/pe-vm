use super::super::*;

// Register index helpers to avoid repeating match ladders.
macro_rules! reg32_read {
    ($regs:expr, $index:expr) => {
        match $index {
            REG_EAX => $regs.eax,
            REG_ECX => $regs.ecx,
            REG_EDX => $regs.edx,
            REG_EBX => $regs.ebx,
            REG_ESP => $regs.esp,
            REG_EBP => $regs.ebp,
            REG_ESI => $regs.esi,
            _ => $regs.edi,
        }
    };
}

macro_rules! reg32_write {
    ($regs:expr, $index:expr, $value:expr) => {
        match $index {
            REG_EAX => $regs.eax = $value,
            REG_ECX => $regs.ecx = $value,
            REG_EDX => $regs.edx = $value,
            REG_EBX => $regs.ebx = $value,
            REG_ESP => $regs.esp = $value,
            REG_EBP => $regs.ebp = $value,
            REG_ESI => $regs.esi = $value,
            _ => $regs.edi = $value,
        }
    };
}

macro_rules! reg8_read {
    ($regs:expr, $index:expr) => {
        match $index {
            REG_AL => $regs.eax as u8,
            REG_CL => $regs.ecx as u8,
            REG_DL => $regs.edx as u8,
            REG_BL => $regs.ebx as u8,
            REG_AH => ($regs.eax >> 8) as u8,
            REG_CH => ($regs.ecx >> 8) as u8,
            REG_DH => ($regs.edx >> 8) as u8,
            REG_BH => ($regs.ebx >> 8) as u8,
            _ => ($regs.ebx >> 8) as u8,
        }
    };
}

macro_rules! reg8_write {
    ($regs:expr, $index:expr, $value:expr) => {
        match $index {
            REG_AL => $regs.eax = ($regs.eax & 0xFFFF_FF00) | $value as u32,
            REG_CL => $regs.ecx = ($regs.ecx & 0xFFFF_FF00) | $value as u32,
            REG_DL => $regs.edx = ($regs.edx & 0xFFFF_FF00) | $value as u32,
            REG_BL => $regs.ebx = ($regs.ebx & 0xFFFF_FF00) | $value as u32,
            REG_AH => $regs.eax = ($regs.eax & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_CH => $regs.ecx = ($regs.ecx & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_DH => $regs.edx = ($regs.edx & 0xFFFF_00FF) | (($value as u32) << 8),
            REG_BH => $regs.ebx = ($regs.ebx & 0xFFFF_00FF) | (($value as u32) << 8),
            _ => $regs.ebx = ($regs.ebx & 0xFFFF_00FF) | (($value as u32) << 8),
        }
    };
}

impl Vm {
    pub(crate) fn eip(&self) -> u32 {
        self.regs.eip
    }

    pub(crate) fn set_eip(&mut self, value: u32) {
        self.regs.eip = value;
    }

    pub(crate) fn reg32(&self, index: u8) -> u32 {
        reg32_read!(self.regs, index)
    }

    pub(crate) fn set_reg32(&mut self, index: u8, value: u32) {
        reg32_write!(self.regs, index, value);
    }

    pub(crate) fn reg16(&self, index: u8) -> u16 {
        self.reg32(index) as u16
    }

    pub(crate) fn set_reg16(&mut self, index: u8, value: u16) {
        let reg = self.reg32(index);
        let next = (reg & 0xFFFF_0000) | value as u32;
        self.set_reg32(index, next);
    }

    pub(crate) fn reg8(&self, index: u8) -> u8 {
        reg8_read!(self.regs, index)
    }

    pub(crate) fn set_reg8(&mut self, index: u8, value: u8) {
        reg8_write!(self.regs, index, value);
    }

    pub(crate) fn xmm(&self, index: u8) -> [u8; 16] {
        self.xmm[index as usize]
    }

    pub(crate) fn set_xmm(&mut self, index: u8, value: [u8; 16]) {
        self.xmm[index as usize] = value;
    }

    pub(crate) fn zf(&self) -> bool {
        self.flags.zf
    }

    pub(crate) fn sf(&self) -> bool {
        self.flags.sf
    }

    pub(crate) fn of(&self) -> bool {
        self.flags.of
    }

    pub(crate) fn cf(&self) -> bool {
        self.flags.cf
    }

    pub(crate) fn set_flags(&mut self, zf: bool, sf: bool, of: bool, cf: bool) {
        self.flags = Flags { cf, zf, sf, of };
    }

    pub(crate) fn fs_base(&self) -> u32 {
        self.fs_base
    }

    pub(crate) fn gs_base(&self) -> u32 {
        self.gs_base
    }
}
