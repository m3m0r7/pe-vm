use super::super::*;

impl Vm {
    pub(crate) fn set_last_error(&mut self, value: u32) {
        self.last_error = value;
    }

    pub(crate) fn last_error(&self) -> u32 {
        self.last_error
    }

    pub(crate) fn registry_open_handle(&mut self, path: String) -> u32 {
        let handle = self.registry_next_handle;
        self.registry_next_handle = self.registry_next_handle.wrapping_add(4);
        self.registry_handles.insert(handle, path);
        handle
    }

    pub(crate) fn registry_handle_path(&self, handle: u32) -> Option<&str> {
        self.registry_handles.get(&handle).map(|value| value.as_str())
    }

    pub(crate) fn registry_close_handle(&mut self, handle: u32) {
        self.registry_handles.remove(&handle);
    }
}
