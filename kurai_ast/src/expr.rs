use kurai_binop::bin_op::BinOp;
use kurai_types::value::Value;

use crate::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
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
    If {
        branches: Vec<IfBranch>,
        else_body: Option<Vec<Expr>>,
    },
    Block {
        stmts: Vec<Stmt>,
        final_expr: Option<Box<Expr>>,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Expr>
}
