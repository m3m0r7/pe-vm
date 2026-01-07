//! x86 instruction registration table.

mod add;
mod atomic;
mod bit;
mod control;
mod core;
mod extended;
mod fpu;
mod group1;
mod group_f6;
mod group_f7;
mod group_fe;
mod group_ff;
mod imul;
mod logic;
mod mov;
mod shift;
mod sse;
mod stack;
mod sub;
mod system;

use crate::vm::{Vm, VmError};

pub(crate) use core::{parse_prefixes, Prefixes};

pub(crate) type ExecFn = fn(&mut Vm, u32, Prefixes) -> Result<(), VmError>;

pub(crate) struct InstructionSet {
    handlers: [Option<ExecFn>; 256],
}

impl InstructionSet {
    pub(crate) fn new() -> Self {
        Self {
            handlers: [None; 256],
        }
    }

    pub(crate) fn register(&mut self, opcode: u8, handler: ExecFn) {
        self.handlers[opcode as usize] = Some(handler);
    }

    pub(crate) fn register_range(&mut self, start: u8, end: u8, handler: ExecFn) {
        for opcode in start..=end {
            self.register(opcode, handler);
        }
    }

    pub(crate) fn execute(
        &self,
        opcode: u8,
        vm: &mut Vm,
        cursor: u32,
        prefixes: Prefixes,
    ) -> Result<(), VmError> {
        if let Some(handler) = self.handlers[opcode as usize] {
            handler(vm, cursor, prefixes)
        } else {
            Err(VmError::UnsupportedInstruction(opcode))
        }
    }

    pub(crate) fn supported_opcodes(&self) -> Vec<u8> {
        self.handlers
            .iter()
            .enumerate()
            .filter_map(|(opcode, handler)| handler.map(|_| opcode as u8))
            .collect()
    }
}

pub(crate) fn register(set: &mut InstructionSet, opcode: u8, handler: ExecFn) {
    set.register(opcode, handler);
}

pub(crate) fn register_range(set: &mut InstructionSet, start: u8, end: u8, handler: ExecFn) {
    set.register_range(start, end, handler);
}

pub(crate) fn supported_extended_opcodes() -> Vec<u8> {
    extended::supported_opcodes()
}

pub(crate) fn build_instruction_set() -> InstructionSet {
    let mut ins = InstructionSet::new();

    register(&mut ins, 0x0F, extended::exec);
    register(&mut ins, 0x00, add::add_rm8_r8);
    register(&mut ins, 0x01, add::add_rm32_r32);
    register(&mut ins, 0x02, add::add_r8_rm8);
    register(&mut ins, 0x03, add::add_r32_rm32);
    register(&mut ins, 0x04, add::add_al_imm8);
    register(&mut ins, 0x05, add::add_eax_imm32);
    register(&mut ins, 0x08, logic::or_rm8_r8);
    register(&mut ins, 0x09, logic::or_rm32_r32);
    register(&mut ins, 0x0A, logic::or_r8_rm8);
    register(&mut ins, 0x0B, logic::or_r32_rm32);
    register(&mut ins, 0x0C, logic::or_al_imm8);
    register(&mut ins, 0x0D, logic::or_eax_imm32);
    register(&mut ins, 0x10, add::adc_rm8_r8);
    register(&mut ins, 0x11, add::adc_rm32_r32);
    register(&mut ins, 0x12, add::adc_r8_rm8);
    register(&mut ins, 0x13, add::adc_r32_rm32);
    register(&mut ins, 0x14, add::adc_al_imm8);
    register(&mut ins, 0x15, add::adc_eax_imm32);
    register(&mut ins, 0x18, sub::sbb_rm8_r8);
    register(&mut ins, 0x19, sub::sbb_rm32_r32);
    register(&mut ins, 0x1A, sub::sbb_r8_rm8);
    register(&mut ins, 0x1B, sub::sbb_r32_rm32);
    register(&mut ins, 0x1C, sub::sbb_al_imm8);
    register(&mut ins, 0x1D, sub::sbb_eax_imm32);
    register(&mut ins, 0x20, logic::and_rm8_r8);
    register(&mut ins, 0x21, logic::and_rm32_r32);
    register(&mut ins, 0x22, logic::and_r8_rm8);
    register(&mut ins, 0x23, logic::and_r32_rm32);
    register(&mut ins, 0x24, logic::and_al_imm8);
    register(&mut ins, 0x25, logic::and_eax_imm32);
    register(&mut ins, 0x28, sub::sub_rm8_r8);
    register(&mut ins, 0x29, sub::sub_rm32_r32);
    register(&mut ins, 0x2A, sub::sub_r8_rm8);
    register(&mut ins, 0x2B, sub::sub_r32_rm32);
    register(&mut ins, 0x2C, sub::sub_al_imm8);
    register(&mut ins, 0x2D, sub::sub_eax_imm32);
    register(&mut ins, 0x30, logic::xor_rm8_r8);
    register(&mut ins, 0x31, logic::xor_rm32_r32);
    register(&mut ins, 0x32, logic::xor_r8_rm8);
    register(&mut ins, 0x33, logic::xor_r32_rm32);
    register(&mut ins, 0x34, logic::xor_al_imm8);
    register(&mut ins, 0x35, logic::xor_eax_imm32);
    register(&mut ins, 0x38, sub::cmp_rm8_r8);
    register(&mut ins, 0x39, sub::cmp_rm32_r32);
    register(&mut ins, 0x3A, sub::cmp_r8_rm8);
    register(&mut ins, 0x3B, sub::cmp_r32_rm32);
    register(&mut ins, 0x3C, sub::cmp_al_imm8);
    register(&mut ins, 0x3D, sub::cmp_eax_imm32);
    register_range(&mut ins, 0x40, 0x47, add::inc_reg);
    register_range(&mut ins, 0x48, 0x4F, sub::dec_reg);
    register_range(&mut ins, 0x50, 0x57, stack::push_reg);
    register_range(&mut ins, 0x58, 0x5F, stack::pop_reg);
    register(&mut ins, 0x6A, stack::push_imm8);
    register(&mut ins, 0x68, stack::push_imm32);
    register(&mut ins, 0x69, imul::imul_rm32_imm32);
    register(&mut ins, 0x6B, imul::imul_rm32_imm8);
    register_range(&mut ins, 0x70, 0x7F, control::jcc_rel8);
    register(&mut ins, 0x80, group1::exec_group1_8);
    register(&mut ins, 0x81, group1::exec_group1_32);
    register(&mut ins, 0x83, group1::exec_group1_32);
    register(&mut ins, 0x84, logic::test_rm8_r8);
    register(&mut ins, 0x85, logic::test_rm32_r32);
    register(&mut ins, 0x87, atomic::xchg_rm32_r32);
    register(&mut ins, 0x88, mov::mov_rm8_r8);
    register(&mut ins, 0x89, mov::mov_rm32_r32);
    register(&mut ins, 0x8A, mov::mov_r8_rm8);
    register(&mut ins, 0x8B, mov::mov_r32_rm32);
    register(&mut ins, 0x8C, mov::mov_seg_to_rm16);
    register(&mut ins, 0x8D, mov::lea);
    register(&mut ins, 0x8F, stack::pop_rm32);
    register(&mut ins, 0x90, system::nop);
    register_range(&mut ins, 0x91, 0x97, atomic::xchg_eax_reg);
    register(&mut ins, 0x9C, stack::pushfd);
    register(&mut ins, 0x99, system::cdq);
    register(&mut ins, 0xA1, mov::mov_moffs_to_eax);
    register(&mut ins, 0xA3, mov::mov_eax_to_moffs);
    register(&mut ins, 0xA4, mov::movsb);
    register(&mut ins, 0xA5, mov::movsd);
    register(&mut ins, 0xAA, mov::stosb);
    register(&mut ins, 0xAB, mov::stosd);
    register(&mut ins, 0xAE, mov::scasb);
    register(&mut ins, 0xAF, mov::scasd);
    register(&mut ins, 0xA8, logic::test_al_imm8);
    register(&mut ins, 0xA9, logic::test_eax_imm32);
    register_range(&mut ins, 0xB0, 0xB7, mov::mov_r8_imm8);
    register_range(&mut ins, 0xB8, 0xBF, mov::mov_r32_imm32);
    register(&mut ins, 0xC0, shift::shift_rm8_imm8);
    register(&mut ins, 0xC1, shift::shift_rm32_imm8);
    register(&mut ins, 0xC2, control::ret_imm16);
    register(&mut ins, 0xC3, control::ret_near);
    register(&mut ins, 0xC6, mov::mov_rm8_imm8);
    register(&mut ins, 0xC7, mov::mov_rm32_imm32);
    register(&mut ins, 0xC9, stack::leave);
    register(&mut ins, 0xCC, system::int3);
    register(&mut ins, 0xCD, system::int);
    register(&mut ins, 0xD6, system::salc);
    register_range(&mut ins, 0xD8, 0xDF, fpu::exec);
    register(&mut ins, 0xD0, shift::shift_rm8_1);
    register(&mut ins, 0xD1, shift::shift_rm32_1);
    register(&mut ins, 0xD2, shift::shift_rm8_cl);
    register(&mut ins, 0xD3, shift::shift_rm32_cl);
    register(&mut ins, 0xE8, control::call_rel32);
    register(&mut ins, 0xE9, control::jmp_rel32);
    register(&mut ins, 0xEB, control::jmp_rel8);
    register(&mut ins, 0xF6, group_f6::exec);
    register(&mut ins, 0xF7, group_f7::exec);
    register(&mut ins, 0xFE, group_fe::exec);
    register(&mut ins, 0xFF, group_ff::exec);

    ins
}
