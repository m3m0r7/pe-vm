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
            cursor = cursor
                .children
                .entry(segment.to_string())
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
            cursor = cursor
                .children
                .entry((*segment).to_string())
                .or_default();
        }
    }

    fn get_key_node(&self, hive: RegistryHive, path: &[String]) -> Option<&RegistryNode> {
        let mut cursor = self.hives.get(&hive)?;
        for segment in path {
            cursor = cursor.children.get(segment)?;
        }
        Some(cursor)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

fn normalize_value_name(name: Option<&str>) -> String {
    name.unwrap_or("").to_string()
}
