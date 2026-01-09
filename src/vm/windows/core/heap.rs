use crate::vm::*;

impl Vm {
    // Allocate bytes in the VM heap for host-side helpers (COM/BSTR/etc).
    pub fn alloc_bytes(&mut self, bytes: &[u8], align: usize) -> Result<u32, VmError> {
        if self.memory.is_empty() {
            return Err(VmError::NoImage);
        }
        let align = align.max(1);
        let mask = align - 1;
        let mut offset = self.heap_cursor;
        if offset & mask != 0 {
            offset = (offset + mask) & !mask;
        }
        let end = offset + bytes.len();
        if end > self.heap_end {
            return Err(VmError::OutOfMemory);
        }
        self.trace_write(
            "alloc_bytes",
            self.base + offset as u32,
            bytes.len(),
            Some(bytes),
        );
        self.memory[offset..end].copy_from_slice(bytes);
        self.heap_cursor = end;
        Ok(self.base + offset as u32)
    }

    pub(crate) fn heap_alloc(&mut self, size: usize) -> u32 {
        let ptr = self.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0);
        if ptr != 0 {
            self.heap_allocs.insert(ptr, size);
        }
        ptr
    }

    pub(crate) fn heap_realloc(&mut self, ptr: u32, size: usize) -> u32 {
        if ptr == 0 {
            return self.heap_alloc(size);
        }
        let new_ptr = self.alloc_bytes(&vec![0u8; size], 8).unwrap_or(0);
        if new_ptr == 0 {
            return 0;
        }
        if let Some(old_size) = self.heap_allocs.get(&ptr).copied() {
            let copy_len = old_size.min(size);
            for offset in 0..copy_len {
                if let Ok(value) = self.read_u8(ptr.wrapping_add(offset as u32)) {
                    let _ = self.write_u8(new_ptr.wrapping_add(offset as u32), value);
                }
            }
        }
        self.heap_allocs.remove(&ptr);
        self.heap_allocs.insert(new_ptr, size);
        new_ptr
    }

    pub(crate) fn heap_free(&mut self, ptr: u32) -> bool {
        self.heap_allocs.remove(&ptr).is_some()
    }

    pub(crate) fn heap_size(&self, ptr: u32) -> Option<usize> {
        self.heap_allocs.get(&ptr).copied()
    }
}
