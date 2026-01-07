//! Windows .reg file parsing helpers.

mod decode;
mod parse;
mod string;
mod value;

use std::path::Path;

use super::{Registry, RegistryError, RegistryMergeMode};

pub(super) fn load_from_registry(path: impl AsRef<Path>) -> Result<Registry, RegistryError> {
    let bytes = std::fs::read(path)?;
    let contents = decode::decode_registry_text(&bytes);
    let mut registry = Registry::with_defaults();
    parse::merge_reg_str(&mut registry, &contents, RegistryMergeMode::Overwrite)?;
    Ok(registry)
}
