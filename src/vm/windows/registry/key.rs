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

    /// Returns the root hive name (e.g., "HKEY_LOCAL_MACHINE").
    pub fn root(&self) -> &'static str {
        match self.hive {
            RegistryHive::ClassesRoot => "HKEY_CLASSES_ROOT",
            RegistryHive::LocalMachine => "HKEY_LOCAL_MACHINE",
            RegistryHive::CurrentUser => "HKEY_CURRENT_USER",
            RegistryHive::Users => "HKEY_USERS",
            RegistryHive::CurrentConfig => "HKEY_CURRENT_CONFIG",
        }
    }

    /// Returns the value name if present.
    pub fn value_name(&self) -> Option<&str> {
        self.value_name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_hive_parse_hklm() {
        assert_eq!(
            RegistryHive::parse("HKLM"),
            Some(RegistryHive::LocalMachine)
        );
        assert_eq!(
            RegistryHive::parse("HKEY_LOCAL_MACHINE"),
            Some(RegistryHive::LocalMachine)
        );
        assert_eq!(
            RegistryHive::parse("hklm"),
            Some(RegistryHive::LocalMachine)
        );
    }

    #[test]
    fn test_registry_hive_parse_hkcu() {
        assert_eq!(RegistryHive::parse("HKCU"), Some(RegistryHive::CurrentUser));
        assert_eq!(
            RegistryHive::parse("HKEY_CURRENT_USER"),
            Some(RegistryHive::CurrentUser)
        );
    }

    #[test]
    fn test_registry_hive_parse_hkcr() {
        assert_eq!(RegistryHive::parse("HKCR"), Some(RegistryHive::ClassesRoot));
        assert_eq!(
            RegistryHive::parse("HKEY_CLASSES_ROOT"),
            Some(RegistryHive::ClassesRoot)
        );
    }

    #[test]
    fn test_registry_hive_parse_invalid() {
        assert_eq!(RegistryHive::parse("INVALID"), None);
        assert_eq!(RegistryHive::parse(""), None);
    }

    #[test]
    fn test_registry_key_parse_simple() {
        let key = RegistryKey::parse("HKLM\\Software\\Test").unwrap();
        assert_eq!(key.hive, RegistryHive::LocalMachine);
        assert_eq!(key.path, vec!["Software", "Test"]);
        assert_eq!(key.value_name, None);
    }

    #[test]
    fn test_registry_key_parse_with_value() {
        let key = RegistryKey::parse("HKLM\\Software\\Test@Version").unwrap();
        assert_eq!(key.hive, RegistryHive::LocalMachine);
        assert_eq!(key.path, vec!["Software", "Test"]);
        assert_eq!(key.value_name, Some("Version".to_string()));
    }

    #[test]
    fn test_registry_key_parse_default_value() {
        let key = RegistryKey::parse("HKLM\\Software\\Test@").unwrap();
        assert_eq!(key.hive, RegistryHive::LocalMachine);
        assert_eq!(key.path, vec!["Software", "Test"]);
        assert_eq!(key.value_name, None);
    }

    #[test]
    fn test_registry_key_parse_forward_slash() {
        let key = RegistryKey::parse("HKLM/Software/Test").unwrap();
        assert_eq!(key.hive, RegistryHive::LocalMachine);
        assert_eq!(key.path, vec!["Software", "Test"]);
    }

    #[test]
    fn test_registry_key_parse_empty() {
        let result = RegistryKey::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_key_parse_invalid_hive() {
        let result = RegistryKey::parse("INVALID\\Software\\Test");
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_key_root() {
        let key = RegistryKey::parse("HKLM\\Software").unwrap();
        assert_eq!(key.root(), "HKEY_LOCAL_MACHINE");

        let key = RegistryKey::parse("HKCU\\Software").unwrap();
        assert_eq!(key.root(), "HKEY_CURRENT_USER");
    }

    #[test]
    fn test_registry_key_value_name() {
        let key = RegistryKey::parse("HKLM\\Software\\Test@Value").unwrap();
        assert_eq!(key.value_name(), Some("Value"));

        let key = RegistryKey::parse("HKLM\\Software\\Test").unwrap();
        assert_eq!(key.value_name(), None);
    }
}
