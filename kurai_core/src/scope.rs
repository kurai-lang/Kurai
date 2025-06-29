use std::collections::HashMap;
use kurai_expr::expr::Expr;
use kurai_types::value::Value;

#[derive(Debug)]
pub struct Scope(pub HashMap<String, Expr>);

impl Scope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn declare_var(&mut self, name: String, expr: Expr) {
        self.0.insert(name, expr);
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
