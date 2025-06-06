use kurai_binop::bin_op::BinOp;
use kurai_types::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Var(String),                 // refers to variable name
    Id(String),
    Literal(Value),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FnCall {
        name: String,
        args: Vec<Expr>,
    },
}
