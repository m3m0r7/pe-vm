use crate::pe::PeFile;

use crate::vm::*;

impl Vm {
    pub fn register_import(&mut self, module: &str, name: &str, func: HostCall) {
        self.register_import_with_cleanup(module, name, func, 0);
    }

    pub fn register_import_stdcall(
        &mut self,
        module: &str,
        name: &str,
        stack_cleanup: u32,
        func: HostCall,
    ) {
        self.register_import_with_cleanup(module, name, func, stack_cleanup);
    }

    fn register_import_with_cleanup(
        &mut self,
        module: &str,
        name: &str,
        func: HostCall,
        stack_cleanup: u32,
    ) {
        self.imports_by_name.insert(
            import_key(module, name),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn register_import_any(&mut self, name: &str, func: HostCall) {
        self.register_import_any_with_cleanup(name, func, 0);
    }

    pub fn register_import_any_stdcall(&mut self, name: &str, stack_cleanup: u32, func: HostCall) {
        self.register_import_any_with_cleanup(name, func, stack_cleanup);
    }

    fn register_import_any_with_cleanup(&mut self, name: &str, func: HostCall, stack_cleanup: u32) {
        self.imports_by_any.insert(
            name.to_ascii_lowercase(),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn register_import_ordinal(&mut self, module: &str, ordinal: u16, func: HostCall) {
        self.register_import_ordinal_with_cleanup(module, ordinal, func, 0);
    }

    pub fn register_import_ordinal_stdcall(
        &mut self,
        module: &str,
        ordinal: u16,
        stack_cleanup: u32,
        func: HostCall,
    ) {
        self.register_import_ordinal_with_cleanup(module, ordinal, func, stack_cleanup);
    }

    fn register_import_ordinal_with_cleanup(
        &mut self,
        module: &str,
        ordinal: u16,
        func: HostCall,
        stack_cleanup: u32,
    ) {
        self.imports_by_ordinal.insert(
            import_ordinal_key(module, ordinal),
            HostFunction {
                func,
                stack_cleanup,
            },
        );
    }

    pub fn resolve_imports(&mut self, pe: &PeFile) -> Result<(), VmError> {
        self.resolve_imports_at(pe, self.base, true)
    }

    pub fn resolve_imports_at(
        &mut self,
        pe: &PeFile,
        base: u32,
        clear_existing: bool,
    ) -> Result<(), VmError> {
        if clear_existing {
            self.imports_by_iat.clear();
            self.imports_by_iat_name.clear();
        }
        let mut missing = Vec::new();
        for import in &pe.imports {
            let mut resolved = None;
            let mut resolved_addr = None;
            let label = if let Some(name) = &import.name {
                format!("{}!{}", import.module, name)
            } else if let Some(ordinal) = import.ordinal {
                format!("{}!#{}", import.module, ordinal)
            } else {
                format!("{}!<unknown>", import.module)
            };
            if let Some(name) = &import.name {
                if let Some(func) = self
                    .imports_by_name
                    .get(&import_key(&import.module, name))
                    .copied()
                {
                    resolved = Some(func);
                } else if let Some(func) =
                    self.imports_by_any.get(&name.to_ascii_lowercase()).copied()
                {
                    resolved = Some(func);
                }
            } else if let Some(ordinal) = import.ordinal {
                if let Some(func) = self
                    .imports_by_ordinal
                    .get(&import_ordinal_key(&import.module, ordinal))
                    .copied()
                {
                    resolved = Some(func);
                }
            }
            if resolved.is_none() {
                if let Some(handle) = self.module_handle_by_name(&import.module) {
                    if let Some(module) = self.module_by_handle(handle) {
                        if let Some(name) = &import.name {
                            if let Some(rva) = module.pe.export_rva(name) {
                                resolved_addr = Some(module.base.wrapping_add(rva));
                            }
                        } else if let Some(ordinal) = import.ordinal {
                            if let Some(symbol) = module
                                .pe
                                .exports
                                .iter()
                                .find(|sym| sym.ordinal == ordinal)
                            {
                                resolved_addr = Some(module.base.wrapping_add(symbol.rva));
                            }
                        }
                    }
                }
            }

            if let Some(func) = resolved {
                let addr = base + import.iat_rva;
                self.imports_by_iat.insert(addr, func);
                self.imports_by_iat_name.insert(addr, label.clone());
                if let Ok(value) = self.read_u32(addr) {
                    if value != 0 {
                        self.imports_by_iat.insert(value, func);
                        self.imports_by_iat_name.insert(value, label.clone());
                    }
                }
            } else if let Some(addr) = resolved_addr {
                let iat_addr = base + import.iat_rva;
                let _ = self.write_u32(iat_addr, addr);
                self.imports_by_iat_name.insert(iat_addr, label.clone());
            } else {
                missing.push(label.clone());
                if std::env::var("PE_VM_TRACE").is_ok() {
                    if let Some(name) = &import.name {
                        eprintln!("[pe_vm] Unresolved import: {}!{}", import.module, name);
                    } else if let Some(ordinal) = import.ordinal {
                        eprintln!("[pe_vm] Unresolved import: {}!#{}", import.module, ordinal);
                    }
                }
                let addr = base + import.iat_rva;
                self.imports_by_iat_name.insert(addr, label.clone());
                if let Ok(value) = self.read_u32(addr) {
                    if value != 0 {
                        self.imports_by_iat_name.insert(value, label);
                    }
                }
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(VmError::MissingImports(missing))
        }
    }

    pub(crate) fn resolve_dynamic_import(&mut self, name: &str) -> Option<u32> {
        let key = name.to_ascii_lowercase();
        if let Some(addr) = self.dynamic_imports.get(&key) {
            return Some(*addr);
        }
        let host = self.imports_by_any.get(&key).copied()?;
        let addr = self.dynamic_import_next;
        self.dynamic_import_next = self.dynamic_import_next.wrapping_add(4);
        self.imports_by_iat.insert(addr, host);
        self.imports_by_iat_name
            .insert(addr, format!("dynamic!{name}"));
        self.dynamic_imports.insert(key, addr);
        Some(addr)
    }

    pub(crate) fn try_call_import(&mut self, addr: u32, return_eip: u32) -> Result<bool, VmError> {
        if let Some(host) = self.imports_by_iat.get(&addr).copied() {
            if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!("[pe_vm] Import call: {name} addr=0x{addr:08X}");
                }
            }
            self.call_host(host, return_eip)?;
            if std::env::var("PE_VM_TRACE_IMPORTS").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!("[pe_vm] Import return: {name} eax=0x{:08X}", self.regs.eax);
                }
            }
            Ok(true)
        } else {
            if std::env::var("PE_VM_TRACE").is_ok() {
                if let Some(name) = self.imports_by_iat_name.get(&addr) {
                    eprintln!("[pe_vm] Missing import call: {name} addr=0x{addr:08X}");
                }
            }
            if std::env::var("PE_VM_ABORT_ON_MISSING_IMPORT").is_ok()
                && self.imports_by_iat_name.contains_key(&addr)
            {
                return Err(VmError::InvalidConfig("missing import"));
            }
            Ok(false)
        }
    }

    pub(crate) fn try_jump_import(&mut self, addr: u32) -> Result<bool, VmError> {
        if let Some(host) = self.imports_by_iat.get(&addr).copied() {
            self.call_host_tail(host)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn import_key(module: &str, name: &str) -> String {
    format!(
        "{}!{}",
        module.to_ascii_lowercase(),
        name.to_ascii_lowercase()
    )
}

fn import_ordinal_key(module: &str, ordinal: u16) -> String {
    format!("{}!#{}", module.to_ascii_lowercase(), ordinal)
}
