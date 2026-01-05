//! YAML merge helpers for the registry.

use std::path::Path;

use serde_yaml::Value as YamlValue;

use super::{Registry, RegistryError, RegistryHive, RegistryMergeMode, RegistryValue};

impl Registry {
    pub fn merge_yaml_str(&mut self, yaml: &str, mode: RegistryMergeMode) -> Result<(), RegistryError> {
        let doc: YamlValue =
            serde_yaml::from_str(yaml).map_err(|err| RegistryError::Yaml(err.to_string()))?;
        self.merge_yaml_value(&doc, mode)
    }

    pub fn merge_yaml_path(
        &mut self,
        path: impl AsRef<Path>,
        mode: RegistryMergeMode,
    ) -> Result<(), RegistryError> {
        let contents = std::fs::read_to_string(path)?;
        self.merge_yaml_str(&contents, mode)
    }

    fn merge_yaml_value(
        &mut self,
        doc: &YamlValue,
        mode: RegistryMergeMode,
    ) -> Result<(), RegistryError> {
        let map = doc
            .as_mapping()
            .ok_or_else(|| RegistryError::Yaml("root must be a mapping".to_string()))?;
        for (key, value) in map {
            let hive_name = key
                .as_str()
                .ok_or_else(|| RegistryError::Yaml("hive name must be string".to_string()))?;
            let hive = RegistryHive::parse(hive_name)
                .ok_or_else(|| RegistryError::InvalidHive(hive_name.to_string()))?;
            let mut path = Vec::new();
            self.merge_yaml_node(hive, &mut path, value, mode)?;
        }
        Ok(())
    }

    fn merge_yaml_node(
        &mut self,
        hive: RegistryHive,
        path: &mut Vec<String>,
        node: &YamlValue,
        mode: RegistryMergeMode,
    ) -> Result<(), RegistryError> {
        match node {
            YamlValue::Mapping(map) => {
                for (key, value) in map {
                    let key_str = key
                        .as_str()
                        .ok_or_else(|| RegistryError::Yaml("key must be string".to_string()))?;
                    if value.is_mapping() {
                        path.push(key_str.to_string());
                        self.merge_yaml_node(hive, path, value, mode)?;
                        path.pop();
                    } else {
                        let value_name = normalize_yaml_value_name(key_str);
                        let registry_value = RegistryValue::from_yaml(value)?;
                        self.apply_value(hive, path, value_name.as_deref(), registry_value, mode);
                    }
                }
            }
            _ => {
                let registry_value = RegistryValue::from_yaml(node)?;
                self.apply_value(hive, path, None, registry_value, mode);
            }
        }
        Ok(())
    }
}

fn normalize_yaml_value_name(name: &str) -> Option<String> {
    if name == "@" || name.eq_ignore_ascii_case("(default)") {
        None
    } else {
        Some(name.to_string())
    }
}
