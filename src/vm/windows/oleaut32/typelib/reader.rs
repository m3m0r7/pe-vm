use crate::vm::VmError;

#[derive(Debug, Clone)]
pub(super) struct SegEntry {
    pub(super) offset: u32,
    pub(super) length: u32,
}

#[derive(Debug, Clone)]
pub(super) struct SegDir {
    pub(super) typeinfo_tab: SegEntry,
    pub(super) guid_tab: SegEntry,
    pub(super) typdesc_tab: SegEntry,
}

impl SegDir {
    pub(super) fn read(reader: &Reader, offset: usize) -> Result<Self, VmError> {
        let entry_size = 8usize;
        let mut entries = Vec::with_capacity(12);
        if std::env::var("PE_VM_TRACE_COM").is_ok() {
            for idx in 0..12usize {
                let base = offset + idx * entry_size;
                let seg_offset = reader.read_u32(base)?;
                let seg_length = reader.read_u32(base + 4)?;
                entries.push(SegEntry {
                    offset: seg_offset,
                    length: seg_length,
                });
                eprintln!(
                    "[pe_vm] typelib seg[{idx}] offset=0x{seg_offset:08X} len=0x{seg_length:08X}"
                );
                if seg_offset != 0xFFFF_FFFF && seg_length != 0 && seg_length % 16 == 0 {
                    let mut bytes = [0u8; 16];
                    for (bidx, slot) in bytes.iter_mut().enumerate() {
                        *slot = reader.read_u8(seg_offset as usize + bidx).unwrap_or(0);
                    }
                    eprintln!(
                        "[pe_vm] typelib seg[{idx}] first16={:02X?}",
                        bytes
                    );
                }
            }
        } else {
            for idx in 0..12usize {
                let base = offset + idx * entry_size;
                let seg_offset = reader.read_u32(base)?;
                let seg_length = reader.read_u32(base + 4)?;
                entries.push(SegEntry {
                    offset: seg_offset,
                    length: seg_length,
                });
            }
        }
        let typeinfo_tab = entries
            .get(0)
            .cloned()
            .ok_or(VmError::InvalidConfig("typelib typeinfo seg missing"))?;
        let typdesc_tab = entries
            .iter()
            .find(|entry| {
                entry.offset != 0xFFFF_FFFF
                    && entry.length != 0
                    && entry.length % 8 == 0
                    && entry.length % 16 != 0
            })
            .cloned()
            .or_else(|| entries.get(10).cloned())
            .unwrap_or(SegEntry { offset: 0, length: 0 });
        let guid_tab = entries
            .iter()
            .filter(|entry| {
                entry.offset != 0xFFFF_FFFF
                    && entry.length != 0
                    && entry.length % 16 == 0
                    && !(entry.offset == typeinfo_tab.offset && entry.length == typeinfo_tab.length)
                    && !(entry.offset == typdesc_tab.offset && entry.length == typdesc_tab.length)
            })
            .max_by_key(|entry| entry.length)
            .cloned()
            .or_else(|| entries.get(5).cloned())
            .unwrap_or(SegEntry { offset: 0, length: 0 });
        Ok(Self {
            typeinfo_tab,
            guid_tab,
            typdesc_tab,
        })
    }
}

#[derive(Clone)]
pub(super) struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub(super) fn read_u8(&self, offset: usize) -> Result<u8, VmError> {
        self.data.get(offset).copied().ok_or(VmError::MemoryOutOfRange)
    }

    pub(super) fn read_u16(&self, offset: usize) -> Result<u16, VmError> {
        if offset + 2 > self.data.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u16::from_le_bytes([self.data[offset], self.data[offset + 1]]))
    }

    pub(super) fn read_u32(&self, offset: usize) -> Result<u32, VmError> {
        if offset + 4 > self.data.len() {
            return Err(VmError::MemoryOutOfRange);
        }
        Ok(u32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ]))
    }

    pub(super) fn read_i32(&self, offset: usize) -> Result<i32, VmError> {
        Ok(self.read_u32(offset)? as i32)
    }
}
