use kurai_types::typ::Type;
use crate::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct TypedArg {
    pub name: String,
    pub typ: Type,
    pub value: Option<Expr>,
}
