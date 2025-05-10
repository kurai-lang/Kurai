use crate::{parse::bin_op::BinOp, value::Value};

#[derive(Debug)]
pub enum Expr {
    Var(String),                 // refers to variable name
    Id(String),
    Literal(Value),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}
