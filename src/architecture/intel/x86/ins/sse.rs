//! x86 SSE instruction stubs.

use crate::vm::{Vm, VmError};

use super::core::{decode_modrm, Prefixes};

pub(crate) fn exec_rm32(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    let modrm = decode_modrm(vm, cursor + 2)?;
    vm.set_eip(cursor + 2 + modrm.len as u32);
    Ok(())
}
