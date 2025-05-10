use crate::{parse::expr::Expr, value::Value};

#[derive(Debug)]
pub struct TypedArg {
    pub name: String,
    pub typ: String,
    pub value: Option<Expr>,
}
