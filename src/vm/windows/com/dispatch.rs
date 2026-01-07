//! Dispatch table for COM-like method invocation.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::vm::{Vm, VmError};

use super::{ComArg, ComValue};

type DispatchFn = Arc<dyn Fn(&mut Vm, &[ComArg]) -> Result<ComValue, VmError> + Send + Sync>;

// Maps DISPIDs to handlers that can be invoked from COM wrappers.
#[derive(Default)]
pub struct DispatchTable {
    handlers: BTreeMap<u32, DispatchFn>,
    fallback: Option<DispatchFn>,
}

impl DispatchTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, dispid: u32, handler: DispatchFn) -> &mut Self {
        self.handlers.insert(dispid, handler);
        self
    }

    pub fn register_i4<F>(&mut self, dispid: u32, handler: F) -> &mut Self
    where
        F: Fn(&mut Vm, &[ComArg]) -> Result<i32, VmError> + Send + Sync + 'static,
    {
        self.register(
            dispid,
            Arc::new(move |vm, args| handler(vm, args).map(ComValue::I4)),
        )
    }

    pub fn register_bstr<F>(&mut self, dispid: u32, handler: F) -> &mut Self
    where
        F: Fn(&mut Vm, &[ComArg]) -> Result<String, VmError> + Send + Sync + 'static,
    {
        self.register(
            dispid,
            Arc::new(move |vm, args| handler(vm, args).map(ComValue::BStr)),
        )
    }

    pub fn register_void<F>(&mut self, dispid: u32, handler: F) -> &mut Self
    where
        F: Fn(&mut Vm, &[ComArg]) -> Result<(), VmError> + Send + Sync + 'static,
    {
        self.register(
            dispid,
            Arc::new(move |vm, args| {
                handler(vm, args)?;
                Ok(ComValue::Void)
            }),
        )
    }

    pub fn set_fallback<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&mut Vm, &[ComArg]) -> Result<ComValue, VmError> + Send + Sync + 'static,
    {
        self.fallback = Some(Arc::new(handler));
        self
    }

    pub fn invoke(&self, vm: &mut Vm, dispid: u32, args: &[ComArg]) -> Result<ComValue, VmError> {
        if let Some(handler) = self.handlers.get(&dispid) {
            return handler(vm, args);
        }
        if let Some(handler) = &self.fallback {
            return handler(vm, args);
        }
        Err(VmError::InvalidConfig("dispatch handler not registered"))
    }
}

// Handle that pairs a CLSID with its dispatch table.
#[derive(Clone)]
pub struct DispatchHandle {
    clsid: String,
    dispatch: Arc<DispatchTable>,
}

impl DispatchHandle {
    pub(crate) fn new(clsid: String, table: DispatchTable) -> Self {
        Self {
            clsid,
            dispatch: Arc::new(table),
        }
    }

    pub fn clsid(&self) -> &str {
        &self.clsid
    }

    pub(crate) fn dispatch(&self) -> Arc<DispatchTable> {
        self.dispatch.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Architecture, VmConfig};

    fn create_test_vm() -> Vm {
        let mut vm = Vm::new(VmConfig::new().architecture(Architecture::X86)).expect("vm");
        vm.memory = vec![0u8; 0x10000];
        vm.base = 0x1000;
        vm.stack_top = 0x1000 + 0x10000 - 4;
        vm.regs.esp = vm.stack_top;
        vm.heap_start = 0x2000;
        vm.heap_end = 0x8000;
        vm.heap_cursor = vm.heap_start;
        vm
    }

    #[test]
    fn test_dispatch_table_new() {
        let table = DispatchTable::new();
        assert!(table.handlers.is_empty());
        assert!(table.fallback.is_none());
    }

    #[test]
    fn test_dispatch_table_register_i4() {
        let mut table = DispatchTable::new();
        table.register_i4(1, |_vm, _args| Ok(42));
        assert!(table.handlers.contains_key(&1));
    }

    #[test]
    fn test_dispatch_table_register_bstr() {
        let mut table = DispatchTable::new();
        table.register_bstr(2, |_vm, _args| Ok("hello".to_string()));
        assert!(table.handlers.contains_key(&2));
    }

    #[test]
    fn test_dispatch_table_register_void() {
        let mut table = DispatchTable::new();
        table.register_void(3, |_vm, _args| Ok(()));
        assert!(table.handlers.contains_key(&3));
    }

    #[test]
    fn test_dispatch_table_invoke_i4() {
        let mut table = DispatchTable::new();
        table.register_i4(1, |_vm, _args| Ok(42));
        let mut vm = create_test_vm();
        let result = table.invoke(&mut vm, 1, &[]);
        assert!(matches!(result, Ok(ComValue::I4(42))));
    }

    #[test]
    fn test_dispatch_table_invoke_bstr() {
        let mut table = DispatchTable::new();
        table.register_bstr(2, |_vm, _args| Ok("test".to_string()));
        let mut vm = create_test_vm();
        let result = table.invoke(&mut vm, 2, &[]);
        if let Ok(ComValue::BStr(s)) = result {
            assert_eq!(s, "test");
        } else {
            panic!("Expected BStr");
        }
    }

    #[test]
    fn test_dispatch_table_invoke_void() {
        let mut table = DispatchTable::new();
        table.register_void(3, |_vm, _args| Ok(()));
        let mut vm = create_test_vm();
        let result = table.invoke(&mut vm, 3, &[]);
        assert!(matches!(result, Ok(ComValue::Void)));
    }

    #[test]
    fn test_dispatch_table_invoke_missing() {
        let table = DispatchTable::new();
        let mut vm = create_test_vm();
        let result = table.invoke(&mut vm, 999, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_table_fallback() {
        let mut table = DispatchTable::new();
        table.set_fallback(|_vm, _args| Ok(ComValue::I4(0)));
        let mut vm = create_test_vm();
        let result = table.invoke(&mut vm, 999, &[]);
        assert!(matches!(result, Ok(ComValue::I4(0))));
    }

    #[test]
    fn test_dispatch_table_chaining() {
        let mut table = DispatchTable::new();
        table
            .register_i4(1, |_vm, _args| Ok(1))
            .register_i4(2, |_vm, _args| Ok(2))
            .register_bstr(3, |_vm, _args| Ok("three".to_string()));
        assert!(table.handlers.contains_key(&1));
        assert!(table.handlers.contains_key(&2));
        assert!(table.handlers.contains_key(&3));
    }

    #[test]
    fn test_dispatch_handle_clsid() {
        let table = DispatchTable::new();
        let handle =
            DispatchHandle::new("{12345678-1234-1234-1234-123456789ABC}".to_string(), table);
        assert_eq!(handle.clsid(), "{12345678-1234-1234-1234-123456789ABC}");
    }
}
