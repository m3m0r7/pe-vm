//! x86 register index constants shared across the VM.

// 32-bit general purpose registers.
pub(crate) const REG_EAX: u8 = 0;
pub(crate) const REG_ECX: u8 = 1;
pub(crate) const REG_EDX: u8 = 2;
pub(crate) const REG_EBX: u8 = 3;
pub(crate) const REG_ESP: u8 = 4;
pub(crate) const REG_EBP: u8 = 5;
pub(crate) const REG_ESI: u8 = 6;
pub(crate) const REG_EDI: u8 = 7;

// 8-bit register indices (AL/CL/DL/BL/AH/CH/DH/BH).
pub(crate) const REG_AL: u8 = 0;
pub(crate) const REG_CL: u8 = 1;
pub(crate) const REG_DL: u8 = 2;
pub(crate) const REG_BL: u8 = 3;
pub(crate) const REG_AH: u8 = 4;
pub(crate) const REG_CH: u8 = 5;
pub(crate) const REG_DH: u8 = 6;
pub(crate) const REG_BH: u8 = 7;
