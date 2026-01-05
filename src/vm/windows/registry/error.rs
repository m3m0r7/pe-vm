//! Registry error types.

use std::fmt;

#[derive(Debug)]
pub enum RegistryError {
    Io(std::io::Error),
    InvalidKey(String),
    InvalidHive(String),
    InvalidValue(String),
    Yaml(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::Io(err) => write!(f, "io error: {err}"),
            RegistryError::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
            RegistryError::InvalidHive(msg) => write!(f, "invalid hive: {msg}"),
            RegistryError::InvalidValue(msg) => write!(f, "invalid value: {msg}"),
            RegistryError::Yaml(msg) => write!(f, "yaml error: {msg}"),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        RegistryError::Io(err)
    }
}
