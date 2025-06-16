// use crate::scope::Scope;
use kurai_typedArg::typedArg::TypedArg;
use kurai_expr::expr::Expr;
use kurai_types::value::Value;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl { 
        name: String,
        typ: String,
        value: Option<Value>,
    },
    Assign {
        name: String,
        value: Value,
    },
    FnCall {
        name: String,
        args: Vec<TypedArg>,
    },
    FnDecl {
        name: String,
        args: Vec<TypedArg>,
        body: Vec<Stmt>,
    },
    Import {
        path: Vec<String>, // Originally String lol, its turned into vector to support "directory
                           // joining" stuff
        nickname: Option<String>,
        is_glob: bool,
    },
    If {
        branches: Vec<IfBranch>,
        else_body: Option<Vec<Stmt>>,
    },
    Loop {
        body: Vec<Stmt>,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Stmt>
}

// This is only for debugging purposes.
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::VarDecl { name, typ, value } => {
                // converts value to string datatype
                let value_str = match value {
                    Some(v) => format!("{:?}", v),
                    None => "None".to_string(),
                };
                write!(f, "VarDecl(name: {}, type: {}, value: {})", name, typ, value_str)
            }
            Stmt::Assign { name, value } => {
                let value_str = format!("{:?}", value);
                write!(f, "Assign(name: {}, value: {})", name, value_str)
            }
            Stmt::FnCall { name, args } => {
                write!(f, "FnCall(name: {}, args: {:?})", name, args)
            }
            Stmt::FnDecl { name, args, body } => {
                    write!(f, "FnDecl(name: {}, args: {:?}, body: {:?})", name, args, body)
            }
            Stmt::Import { path, nickname, is_glob } => {
                if let Some(nickname) = nickname {
                    write!(f, "Import(name: {:?}, nickname: {})", path, nickname)
                } else {
                    write!(f, "Import(name: {:?}, nickname: {:?})", path, nickname)
                }
            }
            Stmt::If { branches, else_body } => {
                write!(f, "If(branches: {:?}, else_body: {:?}", branches, else_body)
            }
            Stmt::Loop { body } => {
                write!(f, "Loop(body: {:?})", body)
            }
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
