use crate::pe::PeFile;

use crate::vm::*;

const NESTED_STACK_SLICE_SIZE: u32 = 0x20000;

impl Vm {
    pub fn call_export(&mut self, pe: &PeFile, name: &str) -> Result<(), VmError> {
        let rva = pe
            .export_rva(name)
            .ok_or_else(|| VmError::MissingExport(name.to_string()))?;
        self.execute(self.base + rva)
    }

    pub fn execute_export_with_values(
        &mut self,
        pe: &PeFile,
        name: &str,
        values: &[Value],
        options: ExecuteOptions,
    ) -> Result<u32, VmError> {
        let rva = pe
            .export_rva(name)
            .ok_or_else(|| VmError::MissingExport(name.to_string()))?;
        self.reset_stack();
        if let Some(env) = options.env_ref() {
            self.set_env(env.clone());
        }
        self.apply_values(values)?;
        self.execute(self.base + rva)?;
        Ok(self.regs.eax)
    }

    pub fn execute(&mut self, entry: u32) -> Result<(), VmError> {
        if self.memory.is_empty() {
            return Err(VmError::NoImage);
        }
        self.regs.eip = entry;
        self.push(0)?;

        let limit = self.config.execution_limit_value();
        let mut steps = 0u64;
        while self.regs.eip != 0 {
            if limit != 0 && steps > limit {
                if std::env::var("PE_VM_TRACE").is_ok() {
                    let eip = self.regs.eip;
                    let start = eip.wrapping_sub(8);
                    let mut bytes = [0u8; 40];
                    for (idx, slot) in bytes.iter_mut().enumerate() {
                        *slot = self.read_u8(start.wrapping_add(idx as u32)).unwrap_or(0);
                    }
                    let hex = bytes
                        .iter()
                        .map(|value| format!("{value:02X}"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    eprintln!(
                        "[pe_vm] execution limit at eip=0x{eip:08X} eax=0x{:08X} ecx=0x{:08X} edx=0x{:08X} edi=0x{:08X} bytes@0x{start:08X}={hex}",
                        self.regs.eax,
                        self.regs.ecx,
                        self.regs.edx,
                        self.regs.edi
                    );
                }
                return Err(VmError::ExecutionLimit);
            }
            let executor = self.executor;
            if let Err(err) = executor.step(self) {
                if std::env::var("PE_VM_TRACE_UNSUPPORTED").is_ok() {
                    let eip = self.regs.eip;
                    let start = eip.wrapping_sub(8);
                    let mut bytes = [0u8; 32];
                    for (idx, slot) in bytes.iter_mut().enumerate() {
                        *slot = self.read_u8(start.wrapping_add(idx as u32)).unwrap_or(0);
                    }
                    let hex = bytes
                        .iter()
                        .map(|value| format!("{value:02X}"))
                        .collect::<Vec<_>>()
                        .join(" ");
                    eprintln!(
                        "[pe_vm] step error at eip=0x{eip:08X} err={err:?} bytes@0x{start:08X}={hex}"
                    );
                }
                return Err(err);
            }
            steps += 1;
        }

        Ok(())
    }

    pub(crate) fn queue_thread(&mut self, entry: u32, param: u32) -> u32 {
        let handle = self.next_thread_handle;
        self.next_thread_handle = self.next_thread_handle.wrapping_add(1);
        self.pending_threads.push(PendingThread { entry, param });
        handle
    }

    pub(crate) fn run_pending_threads(&mut self) -> usize {
        if self.pending_threads.is_empty() {
            return 0;
        }
        let tasks = std::mem::take(&mut self.pending_threads);
        let count = tasks.len();
        for task in tasks {
            let _ = self.execute_at_with_stack(task.entry, &[Value::U32(task.param)]);
        }
        count
    }

    pub(crate) fn execute_at_with_stack(
        &mut self,
        entry: u32,
        values: &[Value],
    ) -> Result<u32, VmError> {
        let saved_regs = self.regs.clone();
        let saved_flags = self.flags;
        let saved_xmm = self.xmm;
        let saved_stack_top = self.stack_top;
        let saved_stack_depth = self.stack_depth;

        let result = (|| {
            if self.stack_top == 0 {
                return Err(VmError::NoImage);
            }
            let depth = saved_stack_depth.checked_add(1).ok_or(VmError::OutOfMemory)?;
            let slice_offset = NESTED_STACK_SLICE_SIZE
                .checked_mul(depth)
                .ok_or(VmError::OutOfMemory)?;
            let stack_top = saved_stack_top
                .checked_sub(slice_offset)
                .ok_or(VmError::OutOfMemory)?;
            let stack_bottom = stack_top
                .checked_sub(NESTED_STACK_SLICE_SIZE)
                .ok_or(VmError::OutOfMemory)?;
            let stack_base = self.base + self.heap_end as u32;
            if stack_bottom < stack_base {
                return Err(VmError::OutOfMemory);
            }
            self.stack_depth = depth;
            self.regs = Registers {
                esp: stack_top,
                ..Registers::default()
            };
            self.xmm = [[0u8; 16]; 8];
            self.flags = Flags::default();

            self.apply_values(values)?;
            if std::env::var("PE_VM_TRACE_STACK").is_ok() {
                let mut line = format!("[pe_vm] stack prep esp=0x{:08X}", self.regs.esp);
                for idx in 0..6 {
                    let addr = self.regs.esp.wrapping_add((idx * 4) as u32);
                    let value = self.read_u32(addr).unwrap_or(0);
                    line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
                }
                eprintln!("{line}");
            }
            self.execute(entry)?;
            Ok(self.regs.eax)
        })();

        self.regs = saved_regs;
        self.flags = saved_flags;
        self.xmm = saved_xmm;
        self.stack_top = saved_stack_top;
        self.stack_depth = saved_stack_depth;

        result
    }

    pub(crate) fn execute_at_with_stack_with_ecx(
        &mut self,
        entry: u32,
        ecx: u32,
        values: &[Value],
    ) -> Result<u32, VmError> {
        let saved_regs = self.regs.clone();
        let saved_flags = self.flags;
        let saved_xmm = self.xmm;
        let saved_stack_top = self.stack_top;
        let saved_stack_depth = self.stack_depth;

        let result = (|| {
            if self.stack_top == 0 {
                return Err(VmError::NoImage);
            }
            let depth = saved_stack_depth.checked_add(1).ok_or(VmError::OutOfMemory)?;
            let slice_offset = NESTED_STACK_SLICE_SIZE
                .checked_mul(depth)
                .ok_or(VmError::OutOfMemory)?;
            let stack_top = saved_stack_top
                .checked_sub(slice_offset)
                .ok_or(VmError::OutOfMemory)?;
            let stack_bottom = stack_top
                .checked_sub(NESTED_STACK_SLICE_SIZE)
                .ok_or(VmError::OutOfMemory)?;
            let stack_base = self.base + self.heap_end as u32;
            if stack_bottom < stack_base {
                return Err(VmError::OutOfMemory);
            }
            self.stack_depth = depth;
            self.regs = Registers {
                ecx,
                esp: stack_top,
                ..Registers::default()
            };
            self.xmm = [[0u8; 16]; 8];
            self.flags = Flags::default();

            self.apply_values(values)?;
            if std::env::var("PE_VM_TRACE_STACK").is_ok() {
                let mut line = format!("[pe_vm] stack prep esp=0x{:08X}", self.regs.esp);
                for idx in 0..6 {
                    let addr = self.regs.esp.wrapping_add((idx * 4) as u32);
                    let value = self.read_u32(addr).unwrap_or(0);
                    line.push_str(&format!(" +0x{:02X}=0x{value:08X}", idx * 4));
                }
                eprintln!("{line}");
            }
            self.execute(entry)?;
            Ok(self.regs.eax)
        })();

        self.regs = saved_regs;
        self.flags = saved_flags;
        self.xmm = saved_xmm;
        self.stack_top = saved_stack_top;
        self.stack_depth = saved_stack_depth;

        result
    }

    pub(crate) fn call_host(
        &mut self,
        host: HostFunction,
        return_eip: u32,
    ) -> Result<(), VmError> {
        self.push(return_eip)?;
        let stack_ptr = self.regs.esp;
        let ret = (host.func)(self, stack_ptr);
        self.regs.eax = ret;
        let ret_addr = self.pop()?;
        self.regs.esp = self.regs.esp.wrapping_add(host.stack_cleanup);
        self.regs.eip = ret_addr;
        Ok(())
    }

    pub(crate) fn call_host_tail(&mut self, host: HostFunction) -> Result<(), VmError> {
        let stack_ptr = self.regs.esp;
        let ret = (host.func)(self, stack_ptr);
        self.regs.eax = ret;
        let ret_addr = self.pop()?;
        self.regs.esp = self.regs.esp.wrapping_add(host.stack_cleanup);
        self.regs.eip = ret_addr;
        Ok(())
    }

    fn reset_stack(&mut self) {
        if self.stack_top != 0 {
            self.regs.esp = self.stack_top;
        }
    }

    fn apply_values(&mut self, values: &[Value]) -> Result<(), VmError> {
        let mut args = Vec::new();
        for value in values {
            match value {
                Value::Env(env) => self.set_env(env.clone()),
                _ => args.push(value),
            }
        }

        for value in args.into_iter().rev() {
            self.push_value(value)?;
        }
        Ok(())
    }

    fn push_value(&mut self, value: &Value) -> Result<(), VmError> {
        match value {
            Value::U32(v) => self.push(*v),
            Value::U64(v) => {
                let low = *v as u32;
                let high = (*v >> 32) as u32;
                self.push(high)?;
                self.push(low)
            }
            Value::String(text) => {
                let mut bytes = text.as_bytes().to_vec();
                bytes.push(0);
                let addr = self.alloc_bytes(&bytes, 1)?;
                self.push(addr)
            }
            Value::Env(_) => Ok(()),
        }
    }
}
