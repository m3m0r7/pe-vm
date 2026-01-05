//! x86 FPU instruction stubs.

use crate::vm::{Vm, VmError};

use super::core::{decode_modrm, Prefixes};

pub(crate) fn exec(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 1)?;
    vm.set_eip(cursor + 1 + modrm.len as u32);
    Ok(())
}
