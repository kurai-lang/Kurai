use kurai_expr::expr::Expr;
use kurai_types::typ::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct TypedArg {
    pub name: String,
    pub typ: Type,
    pub value: Option<Expr>,
}
