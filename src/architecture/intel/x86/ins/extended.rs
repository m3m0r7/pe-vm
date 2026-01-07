//! x86 0F-prefixed instruction handlers.

use crate::vm::{Vm, VmError};

use super::core::Prefixes;
use super::{atomic, bit, control, imul, mov, sse, system};

pub(crate) fn exec(vm: &mut Vm, cursor: u32, prefixes: Prefixes) -> Result<(), VmError> {
    let ext = vm.read_u8(cursor + 1)?;
    match ext {
        0x82..=0x8F => control::jcc_rel32_ext(vm, cursor, ext, prefixes),
        0x90..=0x9F => control::setcc(vm, cursor, ext, prefixes),
        0xA2 => system::cpuid(vm, cursor, prefixes),
        0x40..=0x4F => control::cmovcc(vm, cursor, ext, prefixes),
        0x57 => sse::exec_rm32(vm, cursor, prefixes),
        0x6F => sse::exec_rm32(vm, cursor, prefixes),
        0x7F => sse::exec_rm32(vm, cursor, prefixes),
        0xAF => imul::imul_r32_rm32(vm, cursor, prefixes),
        0xBA => bit::group_ba(vm, cursor, prefixes),
        0xB1 => atomic::cmpxchg_rm32_r32(vm, cursor, prefixes),
        0xB6 => mov::movzx_rm8(vm, cursor, prefixes),
        0xB7 => mov::movzx_rm16(vm, cursor, prefixes),
        0xBE => mov::movsx_rm8(vm, cursor, prefixes),
        0xBF => mov::movsx_rm16(vm, cursor, prefixes),
        0x01 => system::xgetbv(vm, cursor, prefixes),
        0xC1 => atomic::xadd_rm32_r32(vm, cursor, prefixes),
        0xD6 => sse::exec_rm32(vm, cursor, prefixes),
        _ => Err(VmError::UnsupportedInstruction(ext)),
    }
}

pub(crate) fn supported_opcodes() -> Vec<u8> {
    let mut ops: Vec<u8> = (0x40..=0x4F).collect();
    ops.extend(0x82..=0x8F);
    ops.extend([
        0x01, 0x57, 0x6F, 0x7F, 0xA2, 0xAF, 0xBA, 0xB1, 0xB6, 0xB7, 0xC1, 0xD6,
    ]);
    ops.extend(0x90..=0x9F);
    ops.extend([0xBE, 0xBF]);
    ops.sort_unstable();
    ops.dedup();
    ops
}
