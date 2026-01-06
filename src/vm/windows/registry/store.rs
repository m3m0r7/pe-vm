//! Registry storage and mutation.

use std::collections::BTreeMap;

use super::{RegistryError, RegistryHive, RegistryKey, RegistryValue};

// Merge behavior when applying YAML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryMergeMode {
    Append,
    Overwrite,
}

#[derive(Debug, Default, Clone)]
struct RegistryNode {
    values: BTreeMap<String, RegistryValue>,
    children: BTreeMap<String, RegistryNode>,
}

// In-memory registry representation.
#[derive(Debug, Clone)]
pub struct Registry {
    hives: BTreeMap<RegistryHive, RegistryNode>,
}

// Aggregated registry stats for RegQueryInfoKey.
#[derive(Debug, Default, Clone, Copy)]
pub struct RegistryStats {
    pub subkey_count: u32,
    pub max_subkey_len: u32,
    pub value_count: u32,
    pub max_value_name_len: u32,
    pub max_value_len: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self::with_defaults()
    }

    pub fn with_defaults() -> Self {
        // Seed root hives to match common Windows layouts.
        let mut hives = BTreeMap::new();
        hives.insert(RegistryHive::ClassesRoot, RegistryNode::default());
        hives.insert(RegistryHive::LocalMachine, RegistryNode::default());
        hives.insert(RegistryHive::CurrentUser, RegistryNode::default());
        hives.insert(RegistryHive::Users, RegistryNode::default());
        hives.insert(RegistryHive::CurrentConfig, RegistryNode::default());
        let mut registry = Self { hives };
        registry.ensure_path(RegistryHive::LocalMachine, &["Software", "Classes"]);
        registry.ensure_path(RegistryHive::CurrentUser, &["Software", "Classes"]);
        registry
    }

    // Parse a key string and set the value, overwriting if present.
    pub fn set(&mut self, key: &str, value: RegistryValue) -> Result<(), RegistryError> {
        let key = RegistryKey::parse(key)?;
        self.set_key(&key, value);
        Ok(())
    }

    // Parse a key string and append the value if it is not already present.
    pub fn append(&mut self, key: &str, value: RegistryValue) -> Result<bool, RegistryError> {
        let key = RegistryKey::parse(key)?;
        Ok(self.append_key(&key, value))
    }

    pub fn set_key(&mut self, key: &RegistryKey, value: RegistryValue) {
        let node = self.ensure_key_mut(key.hive, &key.path);
        let name = normalize_value_name(key.value_name.as_deref());
        node.values.insert(name, value);
    }

    pub fn append_key(&mut self, key: &RegistryKey, value: RegistryValue) -> bool {
        let node = self.ensure_key_mut(key.hive, &key.path);
        let name = normalize_value_name(key.value_name.as_deref());
        if let std::collections::btree_map::Entry::Vacant(entry) = node.values.entry(name) {
            entry.insert(value);
            true
        } else {
            false
        }
    }

    // Retrieve a value by key string.
    pub fn get(&self, key: &str) -> Result<Option<&RegistryValue>, RegistryError> {
        let key = RegistryKey::parse(key)?;
        Ok(self.get_key(&key))
    }

    pub fn get_key(&self, key: &RegistryKey) -> Option<&RegistryValue> {
        let node = self.get_key_node(key.hive, &key.path)?;
        let name = normalize_value_name(key.value_name.as_deref());
        node.values.get(&name)
    }

    // Enumerate immediate subkeys under a key.
    pub fn subkeys(&self, key: &str) -> Result<Vec<String>, RegistryError> {
        let key = RegistryKey::parse(key)?;
        Ok(self.subkeys_key(&key))
    }

    // Return stats for RegQueryInfoKeyA/W.
    pub fn stats(&self, key: &str, wide: bool) -> Result<RegistryStats, RegistryError> {
        let key = RegistryKey::parse(key)?;
        self.stats_key(&key, wide)
    }

    pub(crate) fn apply_value(
        &mut self,
        hive: RegistryHive,
        path: &[String],
        value_name: Option<&str>,
        value: RegistryValue,
        mode: RegistryMergeMode,
    ) {
        let key = RegistryKey {
            hive,
            path: path.to_vec(),
            value_name: value_name.map(|name| name.to_string()),
        };
        match mode {
            RegistryMergeMode::Append => {
                let _ = self.append_key(&key, value);
            }
            RegistryMergeMode::Overwrite => {
                self.set_key(&key, value);
            }
        }
    }

    fn ensure_key_mut(&mut self, hive: RegistryHive, path: &[String]) -> &mut RegistryNode {
        let root = self
            .hives
            .entry(hive)
            .or_default();
        let mut cursor = root;
        for segment in path {
            let key = normalize_segment(segment);
            cursor = cursor
                .children
                .entry(key)
                .or_default();
        }
        cursor
    }

    fn ensure_path(&mut self, hive: RegistryHive, path: &[&str]) {
        let root = self
            .hives
            .entry(hive)
            .or_default();
        let mut cursor = root;
        for segment in path {
            let key = normalize_segment(segment);
            cursor = cursor
                .children
                .entry(key)
                .or_default();
        }
    }

    fn get_key_node(&self, hive: RegistryHive, path: &[String]) -> Option<&RegistryNode> {
        let mut cursor = self.hives.get(&hive)?;
        for segment in path {
            let key = normalize_segment(segment);
            cursor = cursor.children.get(&key)?;
        }
        Some(cursor)
    }

    fn subkeys_key(&self, key: &RegistryKey) -> Vec<String> {
        let Some(node) = self.get_key_node(key.hive, &key.path) else {
            return Vec::new();
        };
        let mut names = node.children.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    fn stats_key(&self, key: &RegistryKey, wide: bool) -> Result<RegistryStats, RegistryError> {
        let node = self
            .get_key_node(key.hive, &key.path)
            .ok_or_else(|| RegistryError::InvalidKey("missing registry key".to_string()))?;
        let mut stats = RegistryStats::default();
        stats.subkey_count = node.children.len() as u32;
        stats.value_count = node.values.len() as u32;
        stats.max_subkey_len = node
            .children
            .keys()
            .map(|name| name.len())
            .max()
            .unwrap_or(0) as u32;
        let mut max_value_len = 0usize;
        let mut max_value_name_len = 0usize;
        for (name, value) in &node.values {
            if !name.is_empty() {
                max_value_name_len = max_value_name_len.max(name.len());
            }
            let value_len = registry_value_len(value, wide);
            if value_len > max_value_len {
                max_value_len = value_len;
            }
        }
        stats.max_value_name_len = max_value_name_len as u32;
        stats.max_value_len = max_value_len as u32;
        Ok(stats)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

fn normalize_value_name(name: Option<&str>) -> String {
    normalize_segment(name.unwrap_or(""))
}

// Registry keys and value names are case-insensitive.
fn normalize_segment(segment: &str) -> String {
    segment.to_ascii_uppercase()
}

fn registry_value_len(value: &RegistryValue, wide: bool) -> usize {
    match value {
        RegistryValue::String(text) => {
            if wide {
                (text.encode_utf16().count() + 1) * 2
            } else {
                text.as_bytes().len() + 1
            }
        }
        RegistryValue::Dword(_) => 4,
        RegistryValue::MultiString(values) => {
            let mut len = 0usize;
            for value in values {
                if wide {
                    len += (value.encode_utf16().count() + 1) * 2;
                } else {
                    len += value.as_bytes().len() + 1;
                }
            }
            if wide {
                len + 2
            } else {
                len + 1
            }
        }
        RegistryValue::Binary(bytes) => bytes.len(),
    }
}
