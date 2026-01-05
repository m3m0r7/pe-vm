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
        self.register(dispid, Arc::new(move |vm, args| {
            handler(vm, args)?;
            Ok(ComValue::Void)
        }))
    }

    pub fn set_fallback<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&mut Vm, &[ComArg]) -> Result<ComValue, VmError> + Send + Sync + 'static,
    {
        self.fallback = Some(Arc::new(handler));
        self
    }

    pub fn invoke(
        &self,
        vm: &mut Vm,
        dispid: u32,
        args: &[ComArg],
    ) -> Result<ComValue, VmError> {
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
