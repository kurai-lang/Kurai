use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug)]
pub struct Scope {
    a: HashMap<String, Value>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            a: HashMap::new()
        }
    }

    pub fn declare_var(&mut self, name: String, value: Value) {
        self.a.insert(name, value);
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
