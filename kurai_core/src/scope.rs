use std::collections::HashMap;
use kurai_ast::expr::Expr;

#[derive(Debug, Default)]
pub struct Scope(pub HashMap<String, Expr>);

impl Scope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn declare_var(&mut self, name: String, expr: Expr) {
        self.0.insert(name, expr);
    }
}
