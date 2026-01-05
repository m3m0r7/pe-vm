//! Registry key parsing and hive definitions.

use super::RegistryError;

// Represents a registry root hive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegistryHive {
    ClassesRoot,
    LocalMachine,
    CurrentUser,
    Users,
    CurrentConfig,
}

impl RegistryHive {
    pub(crate) fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_uppercase().as_str() {
            "HKCR" | "HKEY_CLASSES_ROOT" => Some(Self::ClassesRoot),
            "HKLM" | "HKEY_LOCAL_MACHINE" => Some(Self::LocalMachine),
            "HKCU" | "HKEY_CURRENT_USER" => Some(Self::CurrentUser),
            "HKU" | "HKEY_USERS" => Some(Self::Users),
            "HKCC" | "HKEY_CURRENT_CONFIG" => Some(Self::CurrentConfig),
            _ => None,
        }
    }
}

// Parsed registry key plus optional value name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryKey {
    pub hive: RegistryHive,
    pub path: Vec<String>,
    pub value_name: Option<String>,
}

impl RegistryKey {
    pub fn parse(input: &str) -> Result<Self, RegistryError> {
        let normalized = input.trim().replace('/', "\\");
        let parts: Vec<&str> = normalized
            .split('\\')
            .filter(|part| !part.is_empty())
            .collect();
        let Some((hive_raw, rest)) = parts.split_first() else {
            return Err(RegistryError::InvalidKey("empty registry key".to_string()));
        };
        let hive = RegistryHive::parse(hive_raw)
            .ok_or_else(|| RegistryError::InvalidHive(hive_raw.to_string()))?;
        let mut path: Vec<String> = rest.iter().map(|part| part.to_string()).collect();
        let mut value_name = None;
        if let Some(last) = path.pop() {
            if last == "@" || last.eq_ignore_ascii_case("(default)") {
                value_name = None;
            } else if let Some((key, name)) = last.split_once('@') {
                if !key.is_empty() {
                    path.push(key.to_string());
                }
                if !name.is_empty() {
                    value_name = Some(name.to_string());
                }
            } else {
                path.push(last);
            }
        }
        Ok(Self {
            hive,
            path,
            value_name,
        })
    }
}
