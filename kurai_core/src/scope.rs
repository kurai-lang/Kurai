use std::collections::HashMap;
use kurai_types::value::Value;

#[derive(Debug)]
pub struct Scope(pub HashMap<String, Value>);

impl Scope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn declare_var(&mut self, name: String, value: Value) {
        self.0.insert(name, value);
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
