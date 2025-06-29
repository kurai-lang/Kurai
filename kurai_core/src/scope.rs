use std::collections::HashMap;
use kurai_stmt::stmt::Stmt;

#[derive(Debug)]
pub struct Scope(pub HashMap<String, Stmt>);

impl Scope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn declare_var(&mut self, name: String, expr: Stmt) {
        self.0.insert(name, expr);
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
