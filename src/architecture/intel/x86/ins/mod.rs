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
            if std::env::var("PE_VM_TRACE_UNSUPPORTED").is_ok() {
                eprintln!(
                    "[pe_vm] unsupported opcode 0x{opcode:02X} at cursor=0x{cursor:08X} eip=0x{:08X}",
                    vm.eip()
                );
            }
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

    let mut inst = InstructionSet::new();

    // Extended prefix
    inst.register(Extended, extended::exec);

    // ADD instructions
    inst.register(AddRm8R8, add::add_rm8_r8);
    inst.register(AddRm32R32, add::add_rm32_r32);
    inst.register(AddR8Rm8, add::add_r8_rm8);
    inst.register(AddR32Rm32, add::add_r32_rm32);
    inst.register(AddAlImm8, add::add_al_imm8);
    inst.register(AddEaxImm32, add::add_eax_imm32);

    // OR instructions
    inst.register(OrRm8R8, logic::or_rm8_r8);
    inst.register(OrRm32R32, logic::or_rm32_r32);
    inst.register(OrR8Rm8, logic::or_r8_rm8);
    inst.register(OrR32Rm32, logic::or_r32_rm32);
    inst.register(OrAlImm8, logic::or_al_imm8);
    inst.register(OrEaxImm32, logic::or_eax_imm32);

    // ADC instructions
    inst.register(AdcRm8R8, add::adc_rm8_r8);
    inst.register(AdcRm32R32, add::adc_rm32_r32);
    inst.register(AdcR8Rm8, add::adc_r8_rm8);
    inst.register(AdcR32Rm32, add::adc_r32_rm32);
    inst.register(AdcAlImm8, add::adc_al_imm8);
    inst.register(AdcEaxImm32, add::adc_eax_imm32);

    // SBB instructions
    inst.register(SbbRm8R8, sub::sbb_rm8_r8);
    inst.register(SbbRm32R32, sub::sbb_rm32_r32);
    inst.register(SbbR8Rm8, sub::sbb_r8_rm8);
    inst.register(SbbR32Rm32, sub::sbb_r32_rm32);
    inst.register(SbbAlImm8, sub::sbb_al_imm8);
    inst.register(SbbEaxImm32, sub::sbb_eax_imm32);

    // AND instructions
    inst.register(AndRm8R8, logic::and_rm8_r8);
    inst.register(AndRm32R32, logic::and_rm32_r32);
    inst.register(AndR8Rm8, logic::and_r8_rm8);
    inst.register(AndR32Rm32, logic::and_r32_rm32);
    inst.register(AndAlImm8, logic::and_al_imm8);
    inst.register(AndEaxImm32, logic::and_eax_imm32);

    // SUB instructions
    inst.register(SubRm8R8, sub::sub_rm8_r8);
    inst.register(SubRm32R32, sub::sub_rm32_r32);
    inst.register(SubR8Rm8, sub::sub_r8_rm8);
    inst.register(SubR32Rm32, sub::sub_r32_rm32);
    inst.register(SubAlImm8, sub::sub_al_imm8);
    inst.register(SubEaxImm32, sub::sub_eax_imm32);

    // XOR instructions
    inst.register(XorRm8R8, logic::xor_rm8_r8);
    inst.register(XorRm32R32, logic::xor_rm32_r32);
    inst.register(XorR8Rm8, logic::xor_r8_rm8);
    inst.register(XorR32Rm32, logic::xor_r32_rm32);
    inst.register(XorAlImm8, logic::xor_al_imm8);
    inst.register(XorEaxImm32, logic::xor_eax_imm32);

    // CMP instructions
    inst.register(CmpRm8R8, sub::cmp_rm8_r8);
    inst.register(CmpRm32R32, sub::cmp_rm32_r32);
    inst.register(CmpR8Rm8, sub::cmp_r8_rm8);
    inst.register(CmpR32Rm32, sub::cmp_r32_rm32);
    inst.register(CmpAlImm8, sub::cmp_al_imm8);
    inst.register(CmpEaxImm32, sub::cmp_eax_imm32);

    // INC register
    inst.register_range(IncEax, IncEdi, add::inc_reg);

    // DEC register
    inst.register_range(DecEax, DecEdi, sub::dec_reg);

    // PUSH register
    inst.register_range(PushEax, PushEdi, stack::push_reg);

    // POP register
    inst.register_range(PopEax, PopEdi, stack::pop_reg);

    // PUSH/IMUL immediates
    inst.register(PushImm32, stack::push_imm32);
    inst.register(ImulRm32Imm32, imul::imul_rm32_imm32);
    inst.register(PushImm8, stack::push_imm8);
    inst.register(ImulRm32Imm8, imul::imul_rm32_imm8);

    // I/O string instructions
    inst.register(Insb, system::insb);
    inst.register(Insd, system::insd);
    inst.register(Outsb, system::outsb);
    inst.register(Outsd, system::outsd);

    // Jcc rel8
    inst.register_range(Jo, Jg, control::jcc_rel8);

    // Group 1 instructions
    inst.register(Group1Rm8Imm8, group1::exec_group1_8);
    inst.register(Group1Rm32Imm32, group1::exec_group1_32);
    inst.register(Group1Rm32Imm8, group1::exec_group1_32);

    // TEST instructions
    inst.register(TestRm8R8, logic::test_rm8_r8);
    inst.register(TestRm32R32, logic::test_rm32_r32);

    // XCHG
    inst.register(XchgRm32R32, atomic::xchg_rm32_r32);

    // MOV instructions
    inst.register(MovRm8R8, mov::mov_rm8_r8);
    inst.register(MovRm32R32, mov::mov_rm32_r32);
    inst.register(MovR8Rm8, mov::mov_r8_rm8);
    inst.register(MovR32Rm32, mov::mov_r32_rm32);
    inst.register(MovSegToRm16, mov::mov_seg_to_rm16);
    inst.register(Lea, mov::lea);
    inst.register(PopRm32, stack::pop_rm32);

    // NOP
    inst.register(Nop, system::nop);

    // XCHG EAX, reg
    inst.register_range(XchgEaxEcx, XchgEaxEdi, atomic::xchg_eax_reg);

    // CDQ, PUSHFD
    inst.register(Cdq, system::cdq);
    inst.register(Pushfd, stack::pushfd);

    // MOV with memory offset
    inst.register(MovMoffsToAl, mov::mov_moffs_to_al);
    inst.register(MovMoffsToEax, mov::mov_moffs_to_eax);
    inst.register(MovAlToMoffs, mov::mov_al_to_moffs);
    inst.register(MovEaxToMoffs, mov::mov_eax_to_moffs);

    // String operations
    inst.register(Movsb, mov::movsb);
    inst.register(Movsd, mov::movsd);
    inst.register(TestAlImm8, logic::test_al_imm8);
    inst.register(TestEaxImm32, logic::test_eax_imm32);
    inst.register(Stosb, mov::stosb);
    inst.register(Stosd, mov::stosd);
    inst.register(Scasb, mov::scasb);
    inst.register(Scasd, mov::scasd);

    // MOV r8, imm8
    inst.register_range(MovAlImm8, MovBhImm8, mov::mov_r8_imm8);

    // MOV r32, imm32
    inst.register_range(MovEaxImm32, MovEdiImm32, mov::mov_r32_imm32);

    // Shift instructions
    inst.register(ShiftRm8Imm8, shift::shift_rm8_imm8);
    inst.register(ShiftRm32Imm8, shift::shift_rm32_imm8);

    // RET instructions
    inst.register(RetImm16, control::ret_imm16);
    inst.register(RetNear, control::ret_near);

    // MOV r/m, imm
    inst.register(MovRm8Imm8, mov::mov_rm8_imm8);
    inst.register(MovRm32Imm32, mov::mov_rm32_imm32);

    // LEAVE
    inst.register(Leave, stack::leave);

    // INT instructions
    inst.register(Int3, system::int3);
    inst.register(Int, system::int);

    // Shift by 1 or CL
    inst.register(ShiftRm8By1, shift::shift_rm8_1);
    inst.register(ShiftRm32By1, shift::shift_rm32_1);
    inst.register(ShiftRm8ByCl, shift::shift_rm8_cl);
    inst.register(ShiftRm32ByCl, shift::shift_rm32_cl);

    // SALC
    inst.register(Salc, system::salc);

    // FPU instructions
    inst.register_range(FpuD8, FpuDF, fpu::exec);

    // CALL/JMP
    inst.register(CallRel32, control::call_rel32);
    inst.register(JmpRel32, control::jmp_rel32);
    inst.register(JmpRel8, control::jmp_rel8);

    // I/O instructions
    inst.register(InAlDx, system::in_al_dx);
    inst.register(InEaxDx, system::in_eax_dx);
    inst.register(OutDxAl, system::out_dx_al);
    inst.register(OutDxEax, system::out_dx_eax);

    // Group instructions
    inst.register(GroupF6, group_f6::exec);
    inst.register(GroupF7, group_f7::exec);
    inst.register(GroupFE, group_fe::exec);
    inst.register(GroupFF, group_ff::exec);

    inst
}
