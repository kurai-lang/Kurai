use kurai_attr::attribute::Attribute;
use kurai_binop::bin_op::BinOp;
// use crate::scope::Scope;
use kurai_typedArg::typedArg::TypedArg;
use kurai_expr::expr::Expr;
use kurai_types::{typ::Type, value::Value};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    // ─────────────────────────────────────────────────────────────────────────────
    // 🧟 OLD STMT VARIANTS (to be replaced with Expr-style)
    // ─────────────────────────────────────────────────────────────────────────────

    /// TEMP: Replace with Expr::Let during parser lowering
    // VarDecl {
    //     name: String,
    //     typ: String,
    //     value: Option<Stmt>, // should become Box<Stmt> eventually
    // },
    //
    // /// TEMP: Replace with Expr::AssignExpr
    // Assign {
    //     name: String,
    //     value: Stmt,
    // },
    //
    // /// TEMP: Replace with Expr::NewFnCall (with callee = Ident)
    // FnCall {
    //     name: String,
    //     args: Vec<TypedArg>, // eventually migrate to Vec<Stmt>
    // },
    //
    // /// KEEP: Top-level construct (not runtime expression)
    // FnDecl {
    //     name: String,
    //     args: Vec<TypedArg>,
    //     body: Vec<Stmt>,
    //     attributes: Vec<Attribute>,
    //     ret_type: Type,
    // },
    //
    // /// KEEP: Top-level import, not an expression
    // Import {
    //     path: Vec<String>,
    //     nickname: Option<String>,
    //     is_glob: bool,
    // },
    //
    // /// TEMP: Replace with nested Expr::IfExpr
    // If {
    //     branches: Vec<IfBranch>,
    //     else_body: Option<Vec<Stmt>>,
    // },
    //
    // /// TEMP: Replace with Expr::LoopExpr
    // Loop {
    //     body: Vec<Stmt>,
    // },
    //
    // /// TEMP: Replace with Expr::BreakExpr
    // Break,
    //
    // /// TEMP: Replace with Expr::ReturnExpr
    // Return(Option<Stmt>),
    //
    // /// TEMP: Replace with Expr::BlockExpr
    // Block(Vec<Stmt>),
    //
    // /// TEMP: Just a wrapper. Eventually all top-levels will be Expr directly.
    // Expr(Stmt),

    // ─────────────────────────────────────────────────────────────────────────────
    // ✅ NEW EXPR VARIANTS (target structure)
    // ─────────────────────────────────────────────────────────────────────────────

    /// Merged: replaces both Var(String) and Id(String)
    Id(String),

    /// Literal value (number, string, etc.)
    Literal(Value),

    /// Binary operation (a + b, x * y, etc.)
    Binary {
        op: BinOp,
        left: Box<Stmt>,
        right: Box<Stmt>,
    },

    /// Call ANY expression, not just named functions (e.g. foo(), getFunc()())
    FnCall {
        callee: Box<Stmt>,
        args: Vec<Stmt>,
    },

    /// let <name>: <typ>? = <value> in <then>
    /// Replaces VarDecl (fully expression-based)
    Let {
        name: String,
        typ: Option<String>,
        value: Option<Box<Stmt>>,
        then: Option<Box<Stmt>>,
    },

    /// assignment as an expression (e.g. x = y + 1)
    Assign {
        target: Box<Stmt>,
        value: Box<Stmt>,
    },

    /// if expr with then/else returning values
    If {
        branches: Vec<IfBranch>,
        else_: Option<Box<Stmt>>,
    },

    /// block of expressions, value = last expr
    Block(Vec<Stmt>),

    /// return as expression (optional value)
    Return(Option<Box<Stmt>>),

    /// loop as expression (might return value in future)
    Loop(Box<Stmt>),

    /// break from loop as expression
    Break,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub condition: Box<Stmt>,
    pub body: Box<Stmt>,
}

// impl Stmt {
//     pub fn has_attr(&self, name: &str) -> bool {
//         match self {
//             Stmt::FnDecl { attributes, .. } => {
//                 attributes.iter().any(|attribute| match attribute {
//                     Attribute::Simple(id) => id == name,
//                     Attribute::WithArgs { name: attr_name, ..} => attr_name == name,
//                     _ => false
//                 })
//             }
//             _ => false,
//         }
//     }
//
//     pub fn get_attr(&self, target: &str) -> Option<&Attribute> {
//         match self {
//             Stmt::FnDecl { attributes, ..} => {
//                 attributes.iter().find(|attribute| match attribute {
//                     Attribute::Simple(name) => name == target,
//                     Attribute::WithArgs { name, .. } => name == target,
//                     _ => false
//                 })
//             }
//             _ => None,
//         }
//     }
// }

// This is only for debugging purposes.
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Id(name) => write!(f, "Id({})", name),

            Stmt::Literal(val) => write!(f, "Literal({:?})", val),

            Stmt::Binary { op, left, right } => {
                write!(f, "Binary({:?} {:?} {:?})", left, op, right)
            }

            Stmt::FnCall { callee, args } => {
                write!(f, "FnCall(callee: {:?}, args: {:?})", callee, args)
            }

            Stmt::Let { name, typ, value, then } => {
                let typ_str = typ.as_deref().unwrap_or("_");
                write!(
                    f,
                    "Let(name: {}, type: {}, value: {:?}, then: {:?})",
                    name, typ_str, value, then
                )
            }

            Stmt::Assign { target, value } => {
                write!(f, "Assign(target: {:?}, value: {:?})", target, value)
            }

            Stmt::If { else_, branches } => {
                match else_ {
                    Some(else_expr) => {
                        write!(
                            f,
                            "If(IfBranch: {:?} else: {:?})",
                            branches, else_expr
                        )
                    }
                    None => {
                        write!(
                            f,
                            "If(IfBranch: {:?})",
                            branches
                        )
                    }
                }
            }

            Stmt::Block(exprs) => {
                write!(f, "Block([")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, "])")
            }

            Stmt::Return(expr_opt) => {
                match expr_opt {
                    Some(expr) => write!(f, "Return({})", expr),
                    None => write!(f, "Return"),
                }
            }

            Stmt::Loop(body) => write!(f, "Loop({})", body),

            Stmt::Break => write!(f, "Break"),
        }
    }
}

// This is literally just interpreter shit
// pls dont use this
// NOTE: DISCONTINUED.
// impl Stmt {
//     pub fn execute(&self, scope: &mut Scope) {
//         match self {
//             Stmt::VarDecl { name, typ, value } => {
//                 let val = value.clone().unwrap_or(Value::Int(0));
//                 scope.declare_var(name.clone(), val);
//             }
//             Stmt::FnCall { name, args } => {
//                 println!("FnCall testing");
//             }
//             _ => println!("OK")
//         }
//     }
// }
