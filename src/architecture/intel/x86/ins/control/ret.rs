use crate::vm::{Vm, VmError, REG_EAX, REG_ESP};

use crate::architecture::intel::x86::ins::core::Prefixes;

pub(crate) fn ret_near(vm: &mut Vm, _cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    if std::env::var("PE_VM_TRACE_RET_INTERNAL").is_ok()
        && matches!(_cursor, 0x1000C608 | 0x1000C6EB)
    {
        eprintln!(
            "[pe_vm] ret_near at 0x{_cursor:08X} eax=0x{:08X} esp=0x{:08X}",
            vm.reg32(REG_EAX),
            vm.reg32(REG_ESP)
        );
    }
    let ret = vm.pop()?;
    if ret != 0 && !vm.contains_addr(ret) && std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!("[pe_vm] ret_near target outside vm: ret=0x{ret:08X} from=0x{_cursor:08X}");
    }
    vm.set_eip(ret);
    Ok(())
}

pub(crate) fn ret_imm16(vm: &mut Vm, cursor: u32, _prefixes: Prefixes) -> Result<(), VmError> {
    if std::env::var("PE_VM_TRACE_RET_INTERNAL").is_ok()
        && matches!(cursor, 0x1000C659 | 0x1000C74F)
    {
        eprintln!(
            "[pe_vm] ret_imm16 at 0x{cursor:08X} eax=0x{:08X} esp=0x{:08X}",
            vm.reg32(REG_EAX),
            vm.reg32(REG_ESP)
        );
    }
    let ret = vm.pop()?;
    let imm = vm.read_u16(cursor + 1)? as u32;
    if ret != 0 && !vm.contains_addr(ret) && std::env::var("PE_VM_TRACE").is_ok() {
        eprintln!(
            "[pe_vm] ret_imm16 target outside vm: ret=0x{ret:08X} from=0x{cursor:08X} imm=0x{imm:04X}"
        );
    }
    vm.set_reg32(REG_ESP, vm.reg32(REG_ESP).wrapping_add(imm));
    vm.set_eip(ret);
    Ok(())
}
