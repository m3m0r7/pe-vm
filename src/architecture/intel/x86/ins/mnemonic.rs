//! x86 instruction mnemonics and opcode definitions.

/// Primary opcodes (single-byte)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum Opcode {
    // ADD instructions
    AddRm8R8 = 0x00,
    AddRm32R32 = 0x01,
    AddR8Rm8 = 0x02,
    AddR32Rm32 = 0x03,
    AddAlImm8 = 0x04,
    AddEaxImm32 = 0x05,

    // OR instructions
    OrRm8R8 = 0x08,
    OrRm32R32 = 0x09,
    OrR8Rm8 = 0x0A,
    OrR32Rm32 = 0x0B,
    OrAlImm8 = 0x0C,
    OrEaxImm32 = 0x0D,

    // Extended prefix
    Extended = 0x0F,

    // ADC instructions
    AdcRm8R8 = 0x10,
    AdcRm32R32 = 0x11,
    AdcR8Rm8 = 0x12,
    AdcR32Rm32 = 0x13,
    AdcAlImm8 = 0x14,
    AdcEaxImm32 = 0x15,

    // SBB instructions
    SbbRm8R8 = 0x18,
    SbbRm32R32 = 0x19,
    SbbR8Rm8 = 0x1A,
    SbbR32Rm32 = 0x1B,
    SbbAlImm8 = 0x1C,
    SbbEaxImm32 = 0x1D,

    // AND instructions
    AndRm8R8 = 0x20,
    AndRm32R32 = 0x21,
    AndR8Rm8 = 0x22,
    AndR32Rm32 = 0x23,
    AndAlImm8 = 0x24,
    AndEaxImm32 = 0x25,

    // SUB instructions
    SubRm8R8 = 0x28,
    SubRm32R32 = 0x29,
    SubR8Rm8 = 0x2A,
    SubR32Rm32 = 0x2B,
    SubAlImm8 = 0x2C,
    SubEaxImm32 = 0x2D,

    // XOR instructions
    XorRm8R8 = 0x30,
    XorRm32R32 = 0x31,
    XorR8Rm8 = 0x32,
    XorR32Rm32 = 0x33,
    XorAlImm8 = 0x34,
    XorEaxImm32 = 0x35,

    // CMP instructions
    CmpRm8R8 = 0x38,
    CmpRm32R32 = 0x39,
    CmpR8Rm8 = 0x3A,
    CmpR32Rm32 = 0x3B,
    CmpAlImm8 = 0x3C,
    CmpEaxImm32 = 0x3D,

    // INC register (0x40-0x47)
    IncEax = 0x40,
    IncEcx = 0x41,
    IncEdx = 0x42,
    IncEbx = 0x43,
    IncEsp = 0x44,
    IncEbp = 0x45,
    IncEsi = 0x46,
    IncEdi = 0x47,

    // DEC register (0x48-0x4F)
    DecEax = 0x48,
    DecEcx = 0x49,
    DecEdx = 0x4A,
    DecEbx = 0x4B,
    DecEsp = 0x4C,
    DecEbp = 0x4D,
    DecEsi = 0x4E,
    DecEdi = 0x4F,

    // PUSH register (0x50-0x57)
    PushEax = 0x50,
    PushEcx = 0x51,
    PushEdx = 0x52,
    PushEbx = 0x53,
    PushEsp = 0x54,
    PushEbp = 0x55,
    PushEsi = 0x56,
    PushEdi = 0x57,

    // POP register (0x58-0x5F)
    PopEax = 0x58,
    PopEcx = 0x59,
    PopEdx = 0x5A,
    PopEbx = 0x5B,
    PopEsp = 0x5C,
    PopEbp = 0x5D,
    PopEsi = 0x5E,
    PopEdi = 0x5F,

    // PUSH/IMUL immediates
    PushImm32 = 0x68,
    ImulRm32Imm32 = 0x69,
    PushImm8 = 0x6A,
    ImulRm32Imm8 = 0x6B,

    // I/O string instructions
    Insb = 0x6C,
    Insd = 0x6D,
    Outsb = 0x6E,
    Outsd = 0x6F,

    // Jcc rel8 (0x70-0x7F)
    Jo = 0x70,
    Jno = 0x71,
    Jb = 0x72,
    Jae = 0x73,
    Je = 0x74,
    Jne = 0x75,
    Jbe = 0x76,
    Ja = 0x77,
    Js = 0x78,
    Jns = 0x79,
    Jp = 0x7A,
    Jnp = 0x7B,
    Jl = 0x7C,
    Jge = 0x7D,
    Jle = 0x7E,
    Jg = 0x7F,

    // Group 1 instructions
    Group1Rm8Imm8 = 0x80,
    Group1Rm32Imm32 = 0x81,
    Group1Rm32Imm8 = 0x83,

    // TEST instructions
    TestRm8R8 = 0x84,
    TestRm32R32 = 0x85,

    // XCHG r/m32, r32
    XchgRm32R32 = 0x87,

    // MOV instructions
    MovRm8R8 = 0x88,
    MovRm32R32 = 0x89,
    MovR8Rm8 = 0x8A,
    MovR32Rm32 = 0x8B,
    MovSegToRm16 = 0x8C,
    Lea = 0x8D,
    PopRm32 = 0x8F,

    // NOP / XCHG EAX, reg
    Nop = 0x90,
    XchgEaxEcx = 0x91,
    XchgEaxEdx = 0x92,
    XchgEaxEbx = 0x93,
    XchgEaxEsp = 0x94,
    XchgEaxEbp = 0x95,
    XchgEaxEsi = 0x96,
    XchgEaxEdi = 0x97,

    // CDQ, PUSHFD
    Cdq = 0x99,
    Pushfd = 0x9C,

    // MOV with memory offset
    MovMoffsToEax = 0xA1,
    MovEaxToMoffs = 0xA3,

    // String operations
    Movsb = 0xA4,
    Movsd = 0xA5,
    TestAlImm8 = 0xA8,
    TestEaxImm32 = 0xA9,
    Stosb = 0xAA,
    Stosd = 0xAB,
    Scasb = 0xAE,
    Scasd = 0xAF,

    // MOV r8, imm8 (0xB0-0xB7)
    MovAlImm8 = 0xB0,
    MovClImm8 = 0xB1,
    MovDlImm8 = 0xB2,
    MovBlImm8 = 0xB3,
    MovAhImm8 = 0xB4,
    MovChImm8 = 0xB5,
    MovDhImm8 = 0xB6,
    MovBhImm8 = 0xB7,

    // MOV r32, imm32 (0xB8-0xBF)
    MovEaxImm32 = 0xB8,
    MovEcxImm32 = 0xB9,
    MovEdxImm32 = 0xBA,
    MovEbxImm32 = 0xBB,
    MovEspImm32 = 0xBC,
    MovEbpImm32 = 0xBD,
    MovEsiImm32 = 0xBE,
    MovEdiImm32 = 0xBF,

    // Shift instructions
    ShiftRm8Imm8 = 0xC0,
    ShiftRm32Imm8 = 0xC1,

    // RET instructions
    RetImm16 = 0xC2,
    RetNear = 0xC3,

    // MOV r/m, imm
    MovRm8Imm8 = 0xC6,
    MovRm32Imm32 = 0xC7,

    // LEAVE
    Leave = 0xC9,

    // INT instructions
    Int3 = 0xCC,
    Int = 0xCD,

    // Shift by 1 or CL
    ShiftRm8By1 = 0xD0,
    ShiftRm32By1 = 0xD1,
    ShiftRm8ByCl = 0xD2,
    ShiftRm32ByCl = 0xD3,

    // SALC (undocumented)
    Salc = 0xD6,

    // FPU instructions (0xD8-0xDF)
    FpuD8 = 0xD8,
    FpuD9 = 0xD9,
    FpuDA = 0xDA,
    FpuDB = 0xDB,
    FpuDC = 0xDC,
    FpuDD = 0xDD,
    FpuDE = 0xDE,
    FpuDF = 0xDF,

    // CALL/JMP
    CallRel32 = 0xE8,
    JmpRel32 = 0xE9,
    JmpRel8 = 0xEB,

    // I/O instructions
    InAlDx = 0xEC,
    InEaxDx = 0xED,
    OutDxAl = 0xEE,
    OutDxEax = 0xEF,

    // Group instructions
    GroupF6 = 0xF6,
    GroupF7 = 0xF7,
    GroupFE = 0xFE,
    GroupFF = 0xFF,
}

impl Opcode {
    /// Try to convert a byte to an Opcode
    pub(crate) fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            // ADD
            0x00 => Some(Self::AddRm8R8),
            0x01 => Some(Self::AddRm32R32),
            0x02 => Some(Self::AddR8Rm8),
            0x03 => Some(Self::AddR32Rm32),
            0x04 => Some(Self::AddAlImm8),
            0x05 => Some(Self::AddEaxImm32),

            // OR
            0x08 => Some(Self::OrRm8R8),
            0x09 => Some(Self::OrRm32R32),
            0x0A => Some(Self::OrR8Rm8),
            0x0B => Some(Self::OrR32Rm32),
            0x0C => Some(Self::OrAlImm8),
            0x0D => Some(Self::OrEaxImm32),

            // Extended
            0x0F => Some(Self::Extended),

            // ADC
            0x10 => Some(Self::AdcRm8R8),
            0x11 => Some(Self::AdcRm32R32),
            0x12 => Some(Self::AdcR8Rm8),
            0x13 => Some(Self::AdcR32Rm32),
            0x14 => Some(Self::AdcAlImm8),
            0x15 => Some(Self::AdcEaxImm32),

            // SBB
            0x18 => Some(Self::SbbRm8R8),
            0x19 => Some(Self::SbbRm32R32),
            0x1A => Some(Self::SbbR8Rm8),
            0x1B => Some(Self::SbbR32Rm32),
            0x1C => Some(Self::SbbAlImm8),
            0x1D => Some(Self::SbbEaxImm32),

            // AND
            0x20 => Some(Self::AndRm8R8),
            0x21 => Some(Self::AndRm32R32),
            0x22 => Some(Self::AndR8Rm8),
            0x23 => Some(Self::AndR32Rm32),
            0x24 => Some(Self::AndAlImm8),
            0x25 => Some(Self::AndEaxImm32),

            // SUB
            0x28 => Some(Self::SubRm8R8),
            0x29 => Some(Self::SubRm32R32),
            0x2A => Some(Self::SubR8Rm8),
            0x2B => Some(Self::SubR32Rm32),
            0x2C => Some(Self::SubAlImm8),
            0x2D => Some(Self::SubEaxImm32),

            // XOR
            0x30 => Some(Self::XorRm8R8),
            0x31 => Some(Self::XorRm32R32),
            0x32 => Some(Self::XorR8Rm8),
            0x33 => Some(Self::XorR32Rm32),
            0x34 => Some(Self::XorAlImm8),
            0x35 => Some(Self::XorEaxImm32),

            // CMP
            0x38 => Some(Self::CmpRm8R8),
            0x39 => Some(Self::CmpRm32R32),
            0x3A => Some(Self::CmpR8Rm8),
            0x3B => Some(Self::CmpR32Rm32),
            0x3C => Some(Self::CmpAlImm8),
            0x3D => Some(Self::CmpEaxImm32),

            // INC
            0x40 => Some(Self::IncEax),
            0x41 => Some(Self::IncEcx),
            0x42 => Some(Self::IncEdx),
            0x43 => Some(Self::IncEbx),
            0x44 => Some(Self::IncEsp),
            0x45 => Some(Self::IncEbp),
            0x46 => Some(Self::IncEsi),
            0x47 => Some(Self::IncEdi),

            // DEC
            0x48 => Some(Self::DecEax),
            0x49 => Some(Self::DecEcx),
            0x4A => Some(Self::DecEdx),
            0x4B => Some(Self::DecEbx),
            0x4C => Some(Self::DecEsp),
            0x4D => Some(Self::DecEbp),
            0x4E => Some(Self::DecEsi),
            0x4F => Some(Self::DecEdi),

            // PUSH
            0x50 => Some(Self::PushEax),
            0x51 => Some(Self::PushEcx),
            0x52 => Some(Self::PushEdx),
            0x53 => Some(Self::PushEbx),
            0x54 => Some(Self::PushEsp),
            0x55 => Some(Self::PushEbp),
            0x56 => Some(Self::PushEsi),
            0x57 => Some(Self::PushEdi),

            // POP
            0x58 => Some(Self::PopEax),
            0x59 => Some(Self::PopEcx),
            0x5A => Some(Self::PopEdx),
            0x5B => Some(Self::PopEbx),
            0x5C => Some(Self::PopEsp),
            0x5D => Some(Self::PopEbp),
            0x5E => Some(Self::PopEsi),
            0x5F => Some(Self::PopEdi),

            // PUSH/IMUL immediates
            0x68 => Some(Self::PushImm32),
            0x69 => Some(Self::ImulRm32Imm32),
            0x6A => Some(Self::PushImm8),
            0x6B => Some(Self::ImulRm32Imm8),

            // I/O string
            0x6C => Some(Self::Insb),
            0x6D => Some(Self::Insd),
            0x6E => Some(Self::Outsb),
            0x6F => Some(Self::Outsd),

            // Jcc rel8
            0x70 => Some(Self::Jo),
            0x71 => Some(Self::Jno),
            0x72 => Some(Self::Jb),
            0x73 => Some(Self::Jae),
            0x74 => Some(Self::Je),
            0x75 => Some(Self::Jne),
            0x76 => Some(Self::Jbe),
            0x77 => Some(Self::Ja),
            0x78 => Some(Self::Js),
            0x79 => Some(Self::Jns),
            0x7A => Some(Self::Jp),
            0x7B => Some(Self::Jnp),
            0x7C => Some(Self::Jl),
            0x7D => Some(Self::Jge),
            0x7E => Some(Self::Jle),
            0x7F => Some(Self::Jg),

            // Group 1
            0x80 => Some(Self::Group1Rm8Imm8),
            0x81 => Some(Self::Group1Rm32Imm32),
            0x83 => Some(Self::Group1Rm32Imm8),

            // TEST
            0x84 => Some(Self::TestRm8R8),
            0x85 => Some(Self::TestRm32R32),

            // XCHG
            0x87 => Some(Self::XchgRm32R32),

            // MOV
            0x88 => Some(Self::MovRm8R8),
            0x89 => Some(Self::MovRm32R32),
            0x8A => Some(Self::MovR8Rm8),
            0x8B => Some(Self::MovR32Rm32),
            0x8C => Some(Self::MovSegToRm16),
            0x8D => Some(Self::Lea),
            0x8F => Some(Self::PopRm32),

            // NOP / XCHG EAX
            0x90 => Some(Self::Nop),
            0x91 => Some(Self::XchgEaxEcx),
            0x92 => Some(Self::XchgEaxEdx),
            0x93 => Some(Self::XchgEaxEbx),
            0x94 => Some(Self::XchgEaxEsp),
            0x95 => Some(Self::XchgEaxEbp),
            0x96 => Some(Self::XchgEaxEsi),
            0x97 => Some(Self::XchgEaxEdi),

            // CDQ, PUSHFD
            0x99 => Some(Self::Cdq),
            0x9C => Some(Self::Pushfd),

            // MOV moffs
            0xA1 => Some(Self::MovMoffsToEax),
            0xA3 => Some(Self::MovEaxToMoffs),

            // String ops
            0xA4 => Some(Self::Movsb),
            0xA5 => Some(Self::Movsd),
            0xA8 => Some(Self::TestAlImm8),
            0xA9 => Some(Self::TestEaxImm32),
            0xAA => Some(Self::Stosb),
            0xAB => Some(Self::Stosd),
            0xAE => Some(Self::Scasb),
            0xAF => Some(Self::Scasd),

            // MOV r8, imm8
            0xB0 => Some(Self::MovAlImm8),
            0xB1 => Some(Self::MovClImm8),
            0xB2 => Some(Self::MovDlImm8),
            0xB3 => Some(Self::MovBlImm8),
            0xB4 => Some(Self::MovAhImm8),
            0xB5 => Some(Self::MovChImm8),
            0xB6 => Some(Self::MovDhImm8),
            0xB7 => Some(Self::MovBhImm8),

            // MOV r32, imm32
            0xB8 => Some(Self::MovEaxImm32),
            0xB9 => Some(Self::MovEcxImm32),
            0xBA => Some(Self::MovEdxImm32),
            0xBB => Some(Self::MovEbxImm32),
            0xBC => Some(Self::MovEspImm32),
            0xBD => Some(Self::MovEbpImm32),
            0xBE => Some(Self::MovEsiImm32),
            0xBF => Some(Self::MovEdiImm32),

            // Shift
            0xC0 => Some(Self::ShiftRm8Imm8),
            0xC1 => Some(Self::ShiftRm32Imm8),

            // RET
            0xC2 => Some(Self::RetImm16),
            0xC3 => Some(Self::RetNear),

            // MOV r/m, imm
            0xC6 => Some(Self::MovRm8Imm8),
            0xC7 => Some(Self::MovRm32Imm32),

            // LEAVE
            0xC9 => Some(Self::Leave),

            // INT
            0xCC => Some(Self::Int3),
            0xCD => Some(Self::Int),

            // Shift by 1/CL
            0xD0 => Some(Self::ShiftRm8By1),
            0xD1 => Some(Self::ShiftRm32By1),
            0xD2 => Some(Self::ShiftRm8ByCl),
            0xD3 => Some(Self::ShiftRm32ByCl),

            // SALC
            0xD6 => Some(Self::Salc),

            // FPU
            0xD8 => Some(Self::FpuD8),
            0xD9 => Some(Self::FpuD9),
            0xDA => Some(Self::FpuDA),
            0xDB => Some(Self::FpuDB),
            0xDC => Some(Self::FpuDC),
            0xDD => Some(Self::FpuDD),
            0xDE => Some(Self::FpuDE),
            0xDF => Some(Self::FpuDF),

            // CALL/JMP
            0xE8 => Some(Self::CallRel32),
            0xE9 => Some(Self::JmpRel32),
            0xEB => Some(Self::JmpRel8),

            // I/O
            0xEC => Some(Self::InAlDx),
            0xED => Some(Self::InEaxDx),
            0xEE => Some(Self::OutDxAl),
            0xEF => Some(Self::OutDxEax),

            // Groups
            0xF6 => Some(Self::GroupF6),
            0xF7 => Some(Self::GroupF7),
            0xFE => Some(Self::GroupFE),
            0xFF => Some(Self::GroupFF),

            _ => None,
        }
    }

    /// Check if this is an INC register instruction
    #[cfg(test)]
    pub(crate) fn is_inc_reg(self) -> bool {
        let byte = self as u8;
        (0x40..=0x47).contains(&byte)
    }

    /// Check if this is a DEC register instruction
    #[cfg(test)]
    pub(crate) fn is_dec_reg(self) -> bool {
        let byte = self as u8;
        (0x48..=0x4F).contains(&byte)
    }

    /// Check if this is a PUSH register instruction
    #[cfg(test)]
    pub(crate) fn is_push_reg(self) -> bool {
        let byte = self as u8;
        (0x50..=0x57).contains(&byte)
    }

    /// Check if this is a POP register instruction
    #[cfg(test)]
    pub(crate) fn is_pop_reg(self) -> bool {
        let byte = self as u8;
        (0x58..=0x5F).contains(&byte)
    }

    /// Check if this is a Jcc rel8 instruction
    #[cfg(test)]
    pub(crate) fn is_jcc_rel8(self) -> bool {
        let byte = self as u8;
        (0x70..=0x7F).contains(&byte)
    }

    /// Check if this is a XCHG EAX, reg instruction
    #[cfg(test)]
    pub(crate) fn is_xchg_eax_reg(self) -> bool {
        let byte = self as u8;
        (0x91..=0x97).contains(&byte)
    }

    /// Check if this is a MOV r8, imm8 instruction
    #[cfg(test)]
    pub(crate) fn is_mov_r8_imm8(self) -> bool {
        let byte = self as u8;
        (0xB0..=0xB7).contains(&byte)
    }

    /// Check if this is a MOV r32, imm32 instruction
    #[cfg(test)]
    pub(crate) fn is_mov_r32_imm32(self) -> bool {
        let byte = self as u8;
        (0xB8..=0xBF).contains(&byte)
    }

    /// Check if this is an FPU instruction
    #[cfg(test)]
    pub(crate) fn is_fpu(self) -> bool {
        let byte = self as u8;
        (0xD8..=0xDF).contains(&byte)
    }
}

/// Extended opcodes (0F prefix)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum ExtendedOpcode {
    // System instructions
    Xgetbv = 0x01,
    Cpuid = 0xA2,

    // Conditional move (CMOVcc)
    Cmovo = 0x40,
    Cmovno = 0x41,
    Cmovb = 0x42,
    Cmovae = 0x43,
    Cmove = 0x44,
    Cmovne = 0x45,
    Cmovbe = 0x46,
    Cmova = 0x47,
    Cmovs = 0x48,
    Cmovns = 0x49,
    Cmovp = 0x4A,
    Cmovnp = 0x4B,
    Cmovl = 0x4C,
    Cmovge = 0x4D,
    Cmovle = 0x4E,
    Cmovg = 0x4F,

    // SSE instructions
    Xorps = 0x57,
    Punpcklbw = 0x60,
    Punpcklwd = 0x61,
    MovdToXmm = 0x6E,
    Movdqa = 0x6F,
    Pshufd = 0x70,
    MovdFromXmm = 0x7E,
    Movdqu = 0x7F,
    Movq = 0xD6,
    Pxor = 0xEF,

    // Conditional jumps (Jcc rel32)
    Jo = 0x80,
    Jno = 0x81,
    Jb = 0x82,
    Jae = 0x83,
    Je = 0x84,
    Jne = 0x85,
    Jbe = 0x86,
    Ja = 0x87,
    Js = 0x88,
    Jns = 0x89,
    Jp = 0x8A,
    Jnp = 0x8B,
    Jl = 0x8C,
    Jge = 0x8D,
    Jle = 0x8E,
    Jg = 0x8F,

    // Conditional set (SETcc)
    Seto = 0x90,
    Setno = 0x91,
    Setb = 0x92,
    Setae = 0x93,
    Sete = 0x94,
    Setne = 0x95,
    Setbe = 0x96,
    Seta = 0x97,
    Sets = 0x98,
    Setns = 0x99,
    Setp = 0x9A,
    Setnp = 0x9B,
    Setl = 0x9C,
    Setge = 0x9D,
    Setle = 0x9E,
    Setg = 0x9F,

    // Multiplication
    Imul = 0xAF,

    // Atomic operations
    Cmpxchg = 0xB1,
    Xadd = 0xC1,

    // Move with zero/sign extension
    MovzxRm8 = 0xB6,
    MovzxRm16 = 0xB7,
    MovsxRm8 = 0xBE,
    MovsxRm16 = 0xBF,

    // Bit operations
    BitGroup = 0xBA,
}

impl ExtendedOpcode {
    /// Try to convert a byte to an ExtendedOpcode
    pub(crate) fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Self::Xgetbv),
            0xA2 => Some(Self::Cpuid),

            // CMOVcc
            0x40 => Some(Self::Cmovo),
            0x41 => Some(Self::Cmovno),
            0x42 => Some(Self::Cmovb),
            0x43 => Some(Self::Cmovae),
            0x44 => Some(Self::Cmove),
            0x45 => Some(Self::Cmovne),
            0x46 => Some(Self::Cmovbe),
            0x47 => Some(Self::Cmova),
            0x48 => Some(Self::Cmovs),
            0x49 => Some(Self::Cmovns),
            0x4A => Some(Self::Cmovp),
            0x4B => Some(Self::Cmovnp),
            0x4C => Some(Self::Cmovl),
            0x4D => Some(Self::Cmovge),
            0x4E => Some(Self::Cmovle),
            0x4F => Some(Self::Cmovg),

            // SSE
            0x57 => Some(Self::Xorps),
            0x60 => Some(Self::Punpcklbw),
            0x61 => Some(Self::Punpcklwd),
            0x6E => Some(Self::MovdToXmm),
            0x6F => Some(Self::Movdqa),
            0x70 => Some(Self::Pshufd),
            0x7E => Some(Self::MovdFromXmm),
            0x7F => Some(Self::Movdqu),
            0xD6 => Some(Self::Movq),
            0xEF => Some(Self::Pxor),

            // Jcc rel32
            0x80 => Some(Self::Jo),
            0x81 => Some(Self::Jno),
            0x82 => Some(Self::Jb),
            0x83 => Some(Self::Jae),
            0x84 => Some(Self::Je),
            0x85 => Some(Self::Jne),
            0x86 => Some(Self::Jbe),
            0x87 => Some(Self::Ja),
            0x88 => Some(Self::Js),
            0x89 => Some(Self::Jns),
            0x8A => Some(Self::Jp),
            0x8B => Some(Self::Jnp),
            0x8C => Some(Self::Jl),
            0x8D => Some(Self::Jge),
            0x8E => Some(Self::Jle),
            0x8F => Some(Self::Jg),

            // SETcc
            0x90 => Some(Self::Seto),
            0x91 => Some(Self::Setno),
            0x92 => Some(Self::Setb),
            0x93 => Some(Self::Setae),
            0x94 => Some(Self::Sete),
            0x95 => Some(Self::Setne),
            0x96 => Some(Self::Setbe),
            0x97 => Some(Self::Seta),
            0x98 => Some(Self::Sets),
            0x99 => Some(Self::Setns),
            0x9A => Some(Self::Setp),
            0x9B => Some(Self::Setnp),
            0x9C => Some(Self::Setl),
            0x9D => Some(Self::Setge),
            0x9E => Some(Self::Setle),
            0x9F => Some(Self::Setg),

            // Other
            0xAF => Some(Self::Imul),
            0xB1 => Some(Self::Cmpxchg),
            0xB6 => Some(Self::MovzxRm8),
            0xB7 => Some(Self::MovzxRm16),
            0xBA => Some(Self::BitGroup),
            0xBE => Some(Self::MovsxRm8),
            0xBF => Some(Self::MovsxRm16),
            0xC1 => Some(Self::Xadd),

            _ => None,
        }
    }

    /// Check if this is a CMOVcc instruction
    #[cfg(test)]
    pub(crate) fn is_cmovcc(self) -> bool {
        let byte = self as u8;
        (0x40..=0x4F).contains(&byte)
    }

    /// Check if this is a Jcc rel32 instruction
    #[cfg(test)]
    pub(crate) fn is_jcc(self) -> bool {
        let byte = self as u8;
        (0x80..=0x8F).contains(&byte)
    }

    /// Check if this is a SETcc instruction
    #[cfg(test)]
    pub(crate) fn is_setcc(self) -> bool {
        let byte = self as u8;
        (0x90..=0x9F).contains(&byte)
    }

    /// Check if this is an SSE instruction
    #[cfg(test)]
    pub(crate) fn is_sse(self) -> bool {
        matches!(
            self,
            Self::Xorps
                | Self::Punpcklbw
                | Self::Punpcklwd
                | Self::MovdToXmm
                | Self::Movdqa
                | Self::Pshufd
                | Self::MovdFromXmm
                | Self::Movdqu
                | Self::Movq
                | Self::Pxor
        )
    }
}

/// Returns a list of all supported extended opcodes
pub(crate) fn supported_extended_opcodes() -> Vec<u8> {
    let mut ops = Vec::new();

    // CMOVcc
    ops.extend(0x40u8..=0x4F);

    // Jcc rel32
    ops.extend(0x80u8..=0x8F);

    // SETcc
    ops.extend(0x90u8..=0x9F);

    // Individual opcodes
    ops.extend([
        ExtendedOpcode::Xgetbv as u8,
        ExtendedOpcode::Cpuid as u8,
        ExtendedOpcode::Xorps as u8,
        ExtendedOpcode::Punpcklbw as u8,
        ExtendedOpcode::Punpcklwd as u8,
        ExtendedOpcode::MovdToXmm as u8,
        ExtendedOpcode::Movdqa as u8,
        ExtendedOpcode::Pshufd as u8,
        ExtendedOpcode::MovdFromXmm as u8,
        ExtendedOpcode::Movdqu as u8,
        ExtendedOpcode::Movq as u8,
        ExtendedOpcode::Pxor as u8,
        ExtendedOpcode::Imul as u8,
        ExtendedOpcode::Cmpxchg as u8,
        ExtendedOpcode::Xadd as u8,
        ExtendedOpcode::MovzxRm8 as u8,
        ExtendedOpcode::MovzxRm16 as u8,
        ExtendedOpcode::MovsxRm8 as u8,
        ExtendedOpcode::MovsxRm16 as u8,
        ExtendedOpcode::BitGroup as u8,
    ]);

    ops.sort_unstable();
    ops.dedup();
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_byte_add() {
        assert_eq!(Opcode::from_byte(0x00), Some(Opcode::AddRm8R8));
        assert_eq!(Opcode::from_byte(0x01), Some(Opcode::AddRm32R32));
        assert_eq!(Opcode::from_byte(0x05), Some(Opcode::AddEaxImm32));
    }

    #[test]
    fn test_opcode_from_byte_inc_dec() {
        for byte in 0x40u8..=0x47 {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_inc_reg());
        }
        for byte in 0x48u8..=0x4F {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_dec_reg());
        }
    }

    #[test]
    fn test_opcode_from_byte_push_pop() {
        for byte in 0x50u8..=0x57 {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_push_reg());
        }
        for byte in 0x58u8..=0x5F {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_pop_reg());
        }
    }

    #[test]
    fn test_opcode_from_byte_jcc() {
        for byte in 0x70u8..=0x7F {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_jcc_rel8());
        }
    }

    #[test]
    fn test_opcode_from_byte_mov_imm() {
        for byte in 0xB0u8..=0xB7 {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_mov_r8_imm8());
        }
        for byte in 0xB8u8..=0xBF {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_mov_r32_imm32());
        }
    }

    #[test]
    fn test_opcode_from_byte_fpu() {
        for byte in 0xD8u8..=0xDF {
            let opcode = Opcode::from_byte(byte);
            assert!(opcode.is_some());
            assert!(opcode.unwrap().is_fpu());
        }
    }

    #[test]
    fn test_opcode_from_byte_invalid() {
        assert!(Opcode::from_byte(0x06).is_none());
        assert!(Opcode::from_byte(0x07).is_none());
        assert!(Opcode::from_byte(0x60).is_none());
    }

    #[test]
    fn test_extended_from_byte_cmovcc() {
        for byte in 0x40u8..=0x4F {
            let opcode = ExtendedOpcode::from_byte(byte);
            assert!(opcode.is_some(), "CMOVcc 0x{byte:02X} should be valid");
            assert!(opcode.unwrap().is_cmovcc());
        }
    }

    #[test]
    fn test_extended_from_byte_jcc() {
        for byte in 0x80u8..=0x8F {
            let opcode = ExtendedOpcode::from_byte(byte);
            assert!(opcode.is_some(), "Jcc 0x{byte:02X} should be valid");
            assert!(opcode.unwrap().is_jcc());
        }
    }

    #[test]
    fn test_extended_from_byte_setcc() {
        for byte in 0x90u8..=0x9F {
            let opcode = ExtendedOpcode::from_byte(byte);
            assert!(opcode.is_some(), "SETcc 0x{byte:02X} should be valid");
            assert!(opcode.unwrap().is_setcc());
        }
    }

    #[test]
    fn test_extended_from_byte_sse() {
        let sse_opcodes = [0x57, 0x60, 0x61, 0x6E, 0x6F, 0x70, 0x7E, 0x7F, 0xD6, 0xEF];
        for byte in sse_opcodes {
            let opcode = ExtendedOpcode::from_byte(byte);
            assert!(opcode.is_some(), "SSE 0x{byte:02X} should be valid");
            assert!(opcode.unwrap().is_sse());
        }
    }

    #[test]
    fn test_extended_from_byte_invalid() {
        assert!(ExtendedOpcode::from_byte(0x00).is_none());
        assert!(ExtendedOpcode::from_byte(0xFF).is_none());
        assert!(ExtendedOpcode::from_byte(0x50).is_none());
    }

    #[test]
    fn test_supported_extended_opcodes_contains_all() {
        let ops = supported_extended_opcodes();

        for byte in 0x40u8..=0x4F {
            assert!(ops.contains(&byte), "Missing CMOVcc 0x{byte:02X}");
        }
        for byte in 0x80u8..=0x8F {
            assert!(ops.contains(&byte), "Missing Jcc 0x{byte:02X}");
        }
        for byte in 0x90u8..=0x9F {
            assert!(ops.contains(&byte), "Missing SETcc 0x{byte:02X}");
        }
    }
}
