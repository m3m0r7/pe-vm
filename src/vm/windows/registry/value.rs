//! Registry value representations.

use serde_yaml::Value as YamlValue;

use super::RegistryError;

// Minimal set of registry value types needed for COM setup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryValue {
    String(String),
    Dword(u32),
    MultiString(Vec<String>),
    Binary(Vec<u8>),
}

impl RegistryValue {
    pub(crate) fn from_yaml(value: &YamlValue) -> Result<Self, RegistryError> {
        match value {
            YamlValue::String(s) => Ok(Self::String(s.clone())),
            YamlValue::Number(num) => num
                .as_u64()
                .map(|val| Self::Dword(val as u32))
                .ok_or_else(|| RegistryError::InvalidValue("invalid number".to_string())),
            YamlValue::Sequence(seq) => {
                if seq.iter().all(|item| item.as_str().is_some()) {
                    let values = seq
                        .iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect();
                    Ok(Self::MultiString(values))
                } else if seq.iter().all(|item| item.as_i64().is_some()) {
                    let mut bytes = Vec::new();
                    for item in seq {
                        let byte = item.as_i64().unwrap_or(0);
                        if !(0..=255).contains(&byte) {
                            return Err(RegistryError::InvalidValue(
                                "binary values must be 0-255".to_string(),
                            ));
                        }
                        bytes.push(byte as u8);
                    }
                    Ok(Self::Binary(bytes))
                } else {
                    Err(RegistryError::InvalidValue(
                        "unsupported sequence value".to_string(),
                    ))
                }
            }
            _ => Err(RegistryError::InvalidValue(
                "unsupported registry value".to_string(),
            )),
        }
    }
}
