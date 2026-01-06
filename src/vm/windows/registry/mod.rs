//! Windows registry model for COM lookups and configuration.

mod error;
mod key;
mod reg_file;
mod store;
mod value;
mod yaml;

pub use error::RegistryError;
pub use key::{RegistryHive, RegistryKey};
pub use store::{Registry, RegistryMergeMode, RegistryStats};
pub use value::RegistryValue;

use std::path::Path;

pub fn load_from_registry(path: impl AsRef<Path>) -> Result<Registry, RegistryError> {
    reg_file::load_from_registry(path)
}

pub fn load_from_yml(path: impl AsRef<Path>) -> Result<Registry, RegistryError> {
    let mut registry = Registry::with_defaults();
    registry.merge_yaml_path(path, RegistryMergeMode::Overwrite)?;
    Ok(registry)
}
