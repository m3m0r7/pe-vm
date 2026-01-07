use crate::vm::*;

impl Vm {
    pub(crate) fn set_last_com_out_params(&mut self, params: Vec<ComOutParam>) {
        self.last_com_out_params = params;
    }

    pub fn last_com_out_params(&self) -> &[ComOutParam] {
        &self.last_com_out_params
    }

    pub fn clear_last_com_out_params(&mut self) {
        self.last_com_out_params.clear();
    }

    pub fn take_last_com_out_params(&mut self) -> Vec<ComOutParam> {
        std::mem::take(&mut self.last_com_out_params)
    }

    pub fn read_bstr(&self, ptr: u32) -> Result<String, VmError> {
        // Delegate to the OLEAUT32 BSTR reader for convenience in examples.
        windows::oleaut32::read_bstr(self, ptr)
    }
}
