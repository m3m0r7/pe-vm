use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::super::*;

impl Vm {
    pub fn stdout_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
        self.stdout.clone()
    }

    pub fn write_stdout(&self, text: &str) {
        if let Ok(mut buffer) = self.stdout.lock() {
            buffer.extend_from_slice(text.as_bytes());
        }
    }

    pub fn set_env(&mut self, env: BTreeMap<String, String>) {
        self.env = env;
    }

    pub(crate) fn env_value(&self, key: &str) -> Option<&str> {
        self.env.get(key).map(|value| value.as_str())
    }

    pub(crate) fn set_env_entry(&mut self, key: String, value: Option<String>) {
        match value {
            Some(value) => {
                self.env.insert(key, value);
            }
            None => {
                self.env.remove(&key);
            }
        }
    }
}
