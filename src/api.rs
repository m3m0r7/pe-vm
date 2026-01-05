//! High-level API wrapper for PE loading and execution.

use std::path::Path;

use crate::pe::{ExportSymbol, PeFile, ResourceDirectory};
use crate::vm::{ExecuteOptions, Value, Vm, VmError};

#[derive(Debug, Clone)]
pub struct Pe {
    file: PeFile,
    image: Vec<u8>,
}

impl Pe {
    pub fn load(vm: &mut Vm, path: impl AsRef<Path>) -> Result<Self, VmError> {
        let guest_path = path.as_ref().to_string_lossy();
        let host_path = vm.map_path(&guest_path);
        let image = std::fs::read(&host_path)?;
        let file = PeFile::parse(&image)?;
        vm.load_image(&file, &image)?;
        vm.set_image_path(guest_path.to_string());
        vm.resolve_imports(&file)?;
        Ok(Self { file, image })
    }

    pub fn symbols(&self) -> &[ExportSymbol] {
        &self.file.exports
    }

    pub fn resources(&self) -> Option<&ResourceDirectory> {
        self.file.directories.resource.as_ref()
    }

    pub fn file(&self) -> &PeFile {
        &self.file
    }

    pub fn image(&self) -> &[u8] {
        &self.image
    }

    pub fn default_path_mapping() -> crate::vm::PathMapping {
        std::collections::BTreeMap::new()
    }
}

pub struct SymbolExecutor<'a> {
    vm: &'a mut Vm,
    pe: &'a Pe,
    symbol: String,
}

impl<'a> SymbolExecutor<'a> {
    pub fn new(vm: &'a mut Vm, pe: &'a Pe) -> Self {
        Self {
            vm,
            pe,
            symbol: String::new(),
        }
    }

    pub fn load(self, name: &str) -> Self {
        let mut executor = self;
        executor.symbol = name.to_string();
        executor
    }

    pub fn execute(&mut self, values: &[Value], options: ExecuteOptions) -> Result<u32, VmError> {
        self.vm
            .execute_export_with_values(self.pe.file(), &self.symbol, values, options)
    }
}
