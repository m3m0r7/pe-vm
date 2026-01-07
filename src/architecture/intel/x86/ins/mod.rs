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
mod mnemonic;
mod mov;
mod shift;
mod sse;
mod stack;
mod sub;
mod system;

use crate::vm::{Vm, VmError};

pub(crate) use core::{parse_prefixes, Prefixes};
use mnemonic::Opcode;

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

    pub(crate) fn register(&mut self, opcode: Opcode, handler: ExecFn) {
        self.handlers[opcode as usize] = Some(handler);
    }

    pub(crate) fn register_range(&mut self, start: Opcode, end: Opcode, handler: ExecFn) {
        for opcode in (start as u8)..=(end as u8) {
            self.handlers[opcode as usize] = Some(handler);
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

pub(crate) fn supported_extended_opcodes() -> Vec<u8> {
    extended::supported_opcodes()
}

pub(crate) fn build_instruction_set() -> InstructionSet {
    use Opcode::*;

    let mut ins = InstructionSet::new();

    // Extended prefix
    ins.register(Extended, extended::exec);

    // ADD instructions
    ins.register(AddRm8R8, add::add_rm8_r8);
    ins.register(AddRm32R32, add::add_rm32_r32);
    ins.register(AddR8Rm8, add::add_r8_rm8);
    ins.register(AddR32Rm32, add::add_r32_rm32);
    ins.register(AddAlImm8, add::add_al_imm8);
    ins.register(AddEaxImm32, add::add_eax_imm32);

    // OR instructions
    ins.register(OrRm8R8, logic::or_rm8_r8);
    ins.register(OrRm32R32, logic::or_rm32_r32);
    ins.register(OrR8Rm8, logic::or_r8_rm8);
    ins.register(OrR32Rm32, logic::or_r32_rm32);
    ins.register(OrAlImm8, logic::or_al_imm8);
    ins.register(OrEaxImm32, logic::or_eax_imm32);

    // ADC instructions
    ins.register(AdcRm8R8, add::adc_rm8_r8);
    ins.register(AdcRm32R32, add::adc_rm32_r32);
    ins.register(AdcR8Rm8, add::adc_r8_rm8);
    ins.register(AdcR32Rm32, add::adc_r32_rm32);
    ins.register(AdcAlImm8, add::adc_al_imm8);
    ins.register(AdcEaxImm32, add::adc_eax_imm32);

    // SBB instructions
    ins.register(SbbRm8R8, sub::sbb_rm8_r8);
    ins.register(SbbRm32R32, sub::sbb_rm32_r32);
    ins.register(SbbR8Rm8, sub::sbb_r8_rm8);
    ins.register(SbbR32Rm32, sub::sbb_r32_rm32);
    ins.register(SbbAlImm8, sub::sbb_al_imm8);
    ins.register(SbbEaxImm32, sub::sbb_eax_imm32);

    // AND instructions
    ins.register(AndRm8R8, logic::and_rm8_r8);
    ins.register(AndRm32R32, logic::and_rm32_r32);
    ins.register(AndR8Rm8, logic::and_r8_rm8);
    ins.register(AndR32Rm32, logic::and_r32_rm32);
    ins.register(AndAlImm8, logic::and_al_imm8);
    ins.register(AndEaxImm32, logic::and_eax_imm32);

    // SUB instructions
    ins.register(SubRm8R8, sub::sub_rm8_r8);
    ins.register(SubRm32R32, sub::sub_rm32_r32);
    ins.register(SubR8Rm8, sub::sub_r8_rm8);
    ins.register(SubR32Rm32, sub::sub_r32_rm32);
    ins.register(SubAlImm8, sub::sub_al_imm8);
    ins.register(SubEaxImm32, sub::sub_eax_imm32);

    // XOR instructions
    ins.register(XorRm8R8, logic::xor_rm8_r8);
    ins.register(XorRm32R32, logic::xor_rm32_r32);
    ins.register(XorR8Rm8, logic::xor_r8_rm8);
    ins.register(XorR32Rm32, logic::xor_r32_rm32);
    ins.register(XorAlImm8, logic::xor_al_imm8);
    ins.register(XorEaxImm32, logic::xor_eax_imm32);

    // CMP instructions
    ins.register(CmpRm8R8, sub::cmp_rm8_r8);
    ins.register(CmpRm32R32, sub::cmp_rm32_r32);
    ins.register(CmpR8Rm8, sub::cmp_r8_rm8);
    ins.register(CmpR32Rm32, sub::cmp_r32_rm32);
    ins.register(CmpAlImm8, sub::cmp_al_imm8);
    ins.register(CmpEaxImm32, sub::cmp_eax_imm32);

    // INC register
    ins.register_range(IncEax, IncEdi, add::inc_reg);

    // DEC register
    ins.register_range(DecEax, DecEdi, sub::dec_reg);

    // PUSH register
    ins.register_range(PushEax, PushEdi, stack::push_reg);

    // POP register
    ins.register_range(PopEax, PopEdi, stack::pop_reg);

    // PUSH/IMUL immediates
    ins.register(PushImm32, stack::push_imm32);
    ins.register(ImulRm32Imm32, imul::imul_rm32_imm32);
    ins.register(PushImm8, stack::push_imm8);
    ins.register(ImulRm32Imm8, imul::imul_rm32_imm8);

    // I/O string instructions
    ins.register(Insb, system::insb);
    ins.register(Insd, system::insd);
    ins.register(Outsb, system::outsb);
    ins.register(Outsd, system::outsd);

    // Jcc rel8
    ins.register_range(Jo, Jg, control::jcc_rel8);

    // Group 1 instructions
    ins.register(Group1Rm8Imm8, group1::exec_group1_8);
    ins.register(Group1Rm32Imm32, group1::exec_group1_32);
    ins.register(Group1Rm32Imm8, group1::exec_group1_32);

    // TEST instructions
    ins.register(TestRm8R8, logic::test_rm8_r8);
    ins.register(TestRm32R32, logic::test_rm32_r32);

    // XCHG
    ins.register(XchgRm32R32, atomic::xchg_rm32_r32);

    // MOV instructions
    ins.register(MovRm8R8, mov::mov_rm8_r8);
    ins.register(MovRm32R32, mov::mov_rm32_r32);
    ins.register(MovR8Rm8, mov::mov_r8_rm8);
    ins.register(MovR32Rm32, mov::mov_r32_rm32);
    ins.register(MovSegToRm16, mov::mov_seg_to_rm16);
    ins.register(Lea, mov::lea);
    ins.register(PopRm32, stack::pop_rm32);

    // NOP
    ins.register(Nop, system::nop);

    // XCHG EAX, reg
    ins.register_range(XchgEaxEcx, XchgEaxEdi, atomic::xchg_eax_reg);

    // CDQ, PUSHFD
    ins.register(Cdq, system::cdq);
    ins.register(Pushfd, stack::pushfd);

    // MOV with memory offset
    ins.register(MovMoffsToEax, mov::mov_moffs_to_eax);
    ins.register(MovEaxToMoffs, mov::mov_eax_to_moffs);

    // String operations
    ins.register(Movsb, mov::movsb);
    ins.register(Movsd, mov::movsd);
    ins.register(TestAlImm8, logic::test_al_imm8);
    ins.register(TestEaxImm32, logic::test_eax_imm32);
    ins.register(Stosb, mov::stosb);
    ins.register(Stosd, mov::stosd);
    ins.register(Scasb, mov::scasb);
    ins.register(Scasd, mov::scasd);

    // MOV r8, imm8
    ins.register_range(MovAlImm8, MovBhImm8, mov::mov_r8_imm8);

    // MOV r32, imm32
    ins.register_range(MovEaxImm32, MovEdiImm32, mov::mov_r32_imm32);

    // Shift instructions
    ins.register(ShiftRm8Imm8, shift::shift_rm8_imm8);
    ins.register(ShiftRm32Imm8, shift::shift_rm32_imm8);

    // RET instructions
    ins.register(RetImm16, control::ret_imm16);
    ins.register(RetNear, control::ret_near);

    // MOV r/m, imm
    ins.register(MovRm8Imm8, mov::mov_rm8_imm8);
    ins.register(MovRm32Imm32, mov::mov_rm32_imm32);

    // LEAVE
    ins.register(Leave, stack::leave);

    // INT instructions
    ins.register(Int3, system::int3);
    ins.register(Int, system::int);

    // Shift by 1 or CL
    ins.register(ShiftRm8By1, shift::shift_rm8_1);
    ins.register(ShiftRm32By1, shift::shift_rm32_1);
    ins.register(ShiftRm8ByCl, shift::shift_rm8_cl);
    ins.register(ShiftRm32ByCl, shift::shift_rm32_cl);

    // SALC
    ins.register(Salc, system::salc);

    // FPU instructions
    ins.register_range(FpuD8, FpuDF, fpu::exec);

    // CALL/JMP
    ins.register(CallRel32, control::call_rel32);
    ins.register(JmpRel32, control::jmp_rel32);
    ins.register(JmpRel8, control::jmp_rel8);

    // I/O instructions
    ins.register(InAlDx, system::in_al_dx);
    ins.register(InEaxDx, system::in_eax_dx);
    ins.register(OutDxAl, system::out_dx_al);
    ins.register(OutDxEax, system::out_dx_eax);

    // Group instructions
    ins.register(GroupF6, group_f6::exec);
    ins.register(GroupF7, group_f7::exec);
    ins.register(GroupFE, group_fe::exec);
    ins.register(GroupFF, group_ff::exec);

    ins
}
