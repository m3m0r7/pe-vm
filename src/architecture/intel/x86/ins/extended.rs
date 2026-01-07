//! x86 0F-prefixed instruction handlers.

use crate::vm::{Vm, VmError};

use super::core::Prefixes;
use super::mnemonic::ExtendedOpcode;
use super::{atomic, bit, control, imul, mov, sse, system};

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let ext = vm.read_u8(cursor + 1)?;

    let Some(opcode) = ExtendedOpcode::from_byte(ext) else {
        return Err(VmError::UnsupportedInstruction(ext));
    };

    use ExtendedOpcode::*;

    match opcode {
        // System instructions
        Cpuid => system::cpuid(vm, cursor, prefixes),
        Xgetbv => system::xgetbv(vm, cursor, prefixes),

        // Conditional move (CMOVcc)
        Cmovo | Cmovno | Cmovb | Cmovae | Cmove | Cmovne | Cmovbe | Cmova | Cmovs | Cmovns
        | Cmovp | Cmovnp | Cmovl | Cmovge | Cmovle | Cmovg => {
            control::cmovcc(vm, cursor, ext, prefixes)
        }

        // SSE instructions
        Xorps | Punpcklbw | Punpcklwd | MovdToXmm | Movdqa | Pshufd | MovdFromXmm | Movdqu
        | Movq | Pxor => sse::exec_rm32(vm, cursor, prefixes),

        // Conditional jumps (Jcc rel32)
        Jo | Jno | Jb | Jae | Je | Jne | Jbe | Ja | Js | Jns | Jp | Jnp | Jl | Jge | Jle | Jg => {
            control::jcc_rel32_ext(vm, cursor, ext, prefixes)
        }

        // Conditional set (SETcc)
        Seto | Setno | Setb | Setae | Sete | Setne | Setbe | Seta | Sets | Setns | Setp
        | Setnp | Setl | Setge | Setle | Setg => control::setcc(vm, cursor, ext, prefixes),

        // Multiplication
        Imul => imul::imul_r32_rm32(vm, cursor, prefixes),

        // Atomic operations
        Cmpxchg => atomic::cmpxchg_rm32_r32(vm, cursor, prefixes),
        Xadd => atomic::xadd_rm32_r32(vm, cursor, prefixes),

        // Move with extension
        MovzxRm8 => mov::movzx_rm8(vm, cursor, prefixes),
        MovzxRm16 => mov::movzx_rm16(vm, cursor, prefixes),
        MovsxRm8 => mov::movsx_rm8(vm, cursor, prefixes),
        MovsxRm16 => mov::movsx_rm16(vm, cursor, prefixes),

        // Bit operations
        BitGroup => bit::group_ba(vm, cursor, prefixes),
    }
}

pub(crate) fn supported_opcodes() -> Vec<u8> {
    super::mnemonic::supported_extended_opcodes()
}
