use crate::vm::*;

impl Vm {
    pub(crate) fn tls_alloc(&mut self) -> u32 {
        let index = self.tls_next_index;
        self.tls_next_index = self.tls_next_index.wrapping_add(1);
        self.tls_values.insert(index, 0);
        index
    }

    pub(crate) fn tls_set(&mut self, index: u32, value: u32) -> bool {
        if let Some(slot) = self.tls_values.get_mut(&index) {
            *slot = value;
            true
        } else {
            false
        }
    }

    pub(crate) fn tls_get(&self, index: u32) -> u32 {
        self.tls_values.get(&index).copied().unwrap_or(0)
    }

    pub(crate) fn tls_free(&mut self, index: u32) -> bool {
        self.tls_values.remove(&index).is_some()
    }
}
