//! x86 executor and instruction registry.

mod ins;
#[cfg(test)]
mod tests;

use std::sync::OnceLock;

use crate::vm::{Vm, VmError};

use ins::{build_instruction_set, parse_prefixes, InstructionSet};

#[derive(Clone, Copy)]
pub struct X86Executor;

impl X86Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn step(&self, vm: &mut Vm) -> Result<(), VmError> {
        let (cursor, prefixes) = parse_prefixes(vm, vm.eip())?;
        let opcode = vm.read_u8(cursor)?;
        instruction_set().execute(opcode, vm, cursor, prefixes)
    }

    pub(crate) fn supported_opcodes(&self) -> (Vec<u8>, Vec<u8>) {
        (
            instruction_set().supported_opcodes(),
            ins::supported_extended_opcodes(),
        )
    }
}

fn instruction_set() -> &'static InstructionSet {
    static INSTRUCTIONS: OnceLock<InstructionSet> = OnceLock::new();
    INSTRUCTIONS.get_or_init(build_instruction_set)
}
