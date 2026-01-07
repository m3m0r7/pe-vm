use crate::vm::*;

impl Vm {
    pub(crate) fn unhandled_exception_filter(&self) -> u32 {
        self.unhandled_exception_filter
    }

    pub(crate) fn set_unhandled_exception_filter(&mut self, value: u32) {
        self.unhandled_exception_filter = value;
    }

    pub fn message_box_mode(&self) -> MessageBoxMode {
        self.message_box_mode
    }

    pub fn set_message_box_mode(&mut self, mode: MessageBoxMode) {
        self.message_box_mode = mode;
    }

    pub fn supported_opcodes(&self) -> (Vec<u8>, Vec<u8>) {
        self.executor.supported_opcodes()
    }

    pub(crate) fn onexit_table_mut(&mut self, table_ptr: u32) -> &mut Vec<u32> {
        self.onexit_tables.entry(table_ptr).or_default()
    }

    pub(crate) fn take_onexit_table(&mut self, table_ptr: u32) -> Vec<u32> {
        self.onexit_tables.remove(&table_ptr).unwrap_or_default()
    }

    pub(crate) fn default_onexit_table(&self) -> u32 {
        self.default_onexit_table
    }

    pub(crate) fn set_default_onexit_table(&mut self, value: u32) {
        self.default_onexit_table = value;
    }
}
