//! x86 register index enums shared across the VM.

/// 32-bit general purpose register indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Reg32 {
    Eax = 0,
    Ecx = 1,
    Edx = 2,
    Ebx = 3,
    Esp = 4,
    Ebp = 5,
    Esi = 6,
    Edi = 7,
}

impl Reg32 {
    /// Convert from u8 index to Reg32.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Eax),
            1 => Some(Self::Ecx),
            2 => Some(Self::Edx),
            3 => Some(Self::Ebx),
            4 => Some(Self::Esp),
            5 => Some(Self::Ebp),
            6 => Some(Self::Esi),
            7 => Some(Self::Edi),
            _ => None,
        }
    }

    /// Get the u8 index value.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn index(self) -> u8 {
        self as u8
    }
}

/// 8-bit register indices (AL/CL/DL/BL/AH/CH/DH/BH).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Reg8 {
    Al = 0,
    Cl = 1,
    Dl = 2,
    Bl = 3,
    Ah = 4,
    Ch = 5,
    Dh = 6,
    Bh = 7,
}

impl Reg8 {
    /// Convert from u8 index to Reg8.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Al),
            1 => Some(Self::Cl),
            2 => Some(Self::Dl),
            3 => Some(Self::Bl),
            4 => Some(Self::Ah),
            5 => Some(Self::Ch),
            6 => Some(Self::Dh),
            7 => Some(Self::Bh),
            _ => None,
        }
    }

    /// Get the u8 index value.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn index(self) -> u8 {
        self as u8
    }
}

// Backward compatibility constants for gradual migration.
pub(crate) const REG_EAX: u8 = Reg32::Eax as u8;
pub(crate) const REG_ECX: u8 = Reg32::Ecx as u8;
pub(crate) const REG_EDX: u8 = Reg32::Edx as u8;
pub(crate) const REG_EBX: u8 = Reg32::Ebx as u8;
pub(crate) const REG_ESP: u8 = Reg32::Esp as u8;
pub(crate) const REG_EBP: u8 = Reg32::Ebp as u8;
pub(crate) const REG_ESI: u8 = Reg32::Esi as u8;
pub(crate) const REG_EDI: u8 = Reg32::Edi as u8;

pub(crate) const REG_AL: u8 = Reg8::Al as u8;
pub(crate) const REG_CL: u8 = Reg8::Cl as u8;
pub(crate) const REG_DL: u8 = Reg8::Dl as u8;
pub(crate) const REG_BL: u8 = Reg8::Bl as u8;
pub(crate) const REG_AH: u8 = Reg8::Ah as u8;
pub(crate) const REG_CH: u8 = Reg8::Ch as u8;
pub(crate) const REG_DH: u8 = Reg8::Dh as u8;
pub(crate) const REG_BH: u8 = Reg8::Bh as u8;
