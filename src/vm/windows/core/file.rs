use std::path::Path;

use crate::vm::*;

impl Vm {
    pub(crate) fn file_open(
        &mut self,
        guest_path: &str,
        readable: bool,
        writable: bool,
        create: bool,
        truncate: bool,
    ) -> Result<u32, VmError> {
        let host_path = self.map_path(guest_path);
        let path = Path::new(&host_path);
        if create {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path);
        }
        if truncate {
            let _ = std::fs::OpenOptions::new().write(true).truncate(true).open(path);
        }
        let mut data = Vec::new();
        if readable {
            if let Ok(bytes) = std::fs::read(path) {
                data = bytes;
            }
        }
        self.virtual_files.insert(host_path.clone(), data);
        let handle = self.file_next_handle;
        self.file_next_handle = self.file_next_handle.wrapping_add(1);
        self.file_handles.insert(
            handle,
            FileHandle {
                path: host_path,
                cursor: 0,
                readable,
                writable,
            },
        );
        Ok(handle)
    }

    pub(crate) fn file_close(&mut self, handle: u32) -> bool {
        self.file_handles.remove(&handle).is_some()
    }

    pub(crate) fn file_exists(&self, guest_path: &str) -> bool {
        let host_path = self.map_path(guest_path);
        let is_dir_hint = guest_path.ends_with(['\\', '/']);
        if is_dir_hint
            && !Path::new(&host_path).exists()
            && std::fs::create_dir_all(&host_path).is_ok()
        {
            return true;
        }
        if let Some(data) = self.virtual_files.get(&host_path) {
            !data.is_empty() || Path::new(&host_path).exists()
        } else {
            Path::new(&host_path).exists()
        }
    }

    pub(crate) fn file_delete(&mut self, guest_path: &str) -> bool {
        let host_path = self.map_path(guest_path);
        self.virtual_files.remove(&host_path);
        std::fs::remove_file(host_path).is_ok()
    }

    pub(crate) fn file_read(&mut self, handle: u32, len: usize) -> Option<Vec<u8>> {
        let file = self.file_handles.get_mut(&handle)?;
        if !file.readable {
            return Some(Vec::new());
        }
        let data = self.virtual_files.get(&file.path)?;
        let start = file.cursor.min(data.len());
        let end = (start + len).min(data.len());
        file.cursor = end;
        Some(data[start..end].to_vec())
    }

    pub(crate) fn file_write(&mut self, handle: u32, bytes: &[u8]) -> Option<usize> {
        let file = self.file_handles.get_mut(&handle)?;
        if !file.writable {
            return Some(0);
        }
        let data = self.virtual_files.entry(file.path.clone()).or_default();
        let start = file.cursor;
        let end = start + bytes.len();
        if data.len() < end {
            data.resize(end, 0);
        }
        data[start..end].copy_from_slice(bytes);
        file.cursor = end;
        Some(bytes.len())
    }

    pub(crate) fn file_size(&self, handle: u32) -> Option<u32> {
        let file = self.file_handles.get(&handle)?;
        let data = self.virtual_files.get(&file.path)?;
        u32::try_from(data.len()).ok()
    }

    pub(crate) fn file_seek(&mut self, handle: u32, offset: i64, method: u32) -> Option<u64> {
        let file = self.file_handles.get_mut(&handle)?;
        let len = self
            .virtual_files
            .get(&file.path)
            .map(|data| data.len() as i64)
            .unwrap_or(0);
        let base = match method {
            0 => 0,
            1 => file.cursor as i64,
            2 => len,
            _ => return None,
        };
        let mut next = base + offset;
        if next < 0 {
            next = 0;
        }
        file.cursor = next as usize;
        Some(next as u64)
    }
}
