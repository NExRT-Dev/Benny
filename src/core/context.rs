use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub values: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }
}
