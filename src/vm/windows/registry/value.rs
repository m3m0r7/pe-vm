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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_value_string() {
        let value = RegistryValue::String("test".to_string());
        if let RegistryValue::String(s) = value {
            assert_eq!(s, "test");
        } else {
            panic!("Expected String");
        }
    }

    #[test]
    fn test_registry_value_dword() {
        let value = RegistryValue::Dword(42);
        if let RegistryValue::Dword(d) = value {
            assert_eq!(d, 42);
        } else {
            panic!("Expected Dword");
        }
    }

    #[test]
    fn test_registry_value_multistring() {
        let value = RegistryValue::MultiString(vec!["one".to_string(), "two".to_string()]);
        if let RegistryValue::MultiString(v) = value {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], "one");
            assert_eq!(v[1], "two");
        } else {
            panic!("Expected MultiString");
        }
    }

    #[test]
    fn test_registry_value_binary() {
        let value = RegistryValue::Binary(vec![0x01, 0x02, 0x03]);
        if let RegistryValue::Binary(b) = value {
            assert_eq!(b, vec![0x01, 0x02, 0x03]);
        } else {
            panic!("Expected Binary");
        }
    }

    #[test]
    fn test_registry_value_from_yaml_string() {
        let yaml = YamlValue::String("hello".to_string());
        let value = RegistryValue::from_yaml(&yaml).unwrap();
        assert_eq!(value, RegistryValue::String("hello".to_string()));
    }

    #[test]
    fn test_registry_value_from_yaml_number() {
        let yaml = YamlValue::Number(serde_yaml::Number::from(123u64));
        let value = RegistryValue::from_yaml(&yaml).unwrap();
        assert_eq!(value, RegistryValue::Dword(123));
    }

    #[test]
    fn test_registry_value_from_yaml_string_sequence() {
        let yaml = YamlValue::Sequence(vec![
            YamlValue::String("a".to_string()),
            YamlValue::String("b".to_string()),
        ]);
        let value = RegistryValue::from_yaml(&yaml).unwrap();
        assert_eq!(
            value,
            RegistryValue::MultiString(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_registry_value_from_yaml_binary_sequence() {
        let yaml = YamlValue::Sequence(vec![
            YamlValue::Number(serde_yaml::Number::from(1i64)),
            YamlValue::Number(serde_yaml::Number::from(2i64)),
            YamlValue::Number(serde_yaml::Number::from(255i64)),
        ]);
        let value = RegistryValue::from_yaml(&yaml).unwrap();
        assert_eq!(value, RegistryValue::Binary(vec![1, 2, 255]));
    }

    #[test]
    fn test_registry_value_from_yaml_invalid_binary() {
        let yaml = YamlValue::Sequence(vec![YamlValue::Number(serde_yaml::Number::from(256i64))]);
        let result = RegistryValue::from_yaml(&yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_value_from_yaml_unsupported() {
        let yaml = YamlValue::Bool(true);
        let result = RegistryValue::from_yaml(&yaml);
        assert!(result.is_err());
    }
}
