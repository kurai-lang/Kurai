use vyn_attr::attribute::Attribute;
use crate::typedArg::TypedArg;
use vyn_types::typ::Type;
use std::fmt;

use crate::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl { 
        name: String,
        typ: Option<String>,
        value: Option<Expr>,
    },
    Assign {
        name: String,
        value: Expr,
    },
    FnCall {
        name: String,
        args: Vec<TypedArg>,
    },
    FnDecl {
        name: String,
        args: Vec<TypedArg>,
        body: Vec<Stmt>,
        attributes: Vec<Attribute>,
        ret_type: Type,
        is_extern: bool,
    },
    Import {
        path: Vec<String>, // Originally String lol, its turned into vector to support "directory
                           // joining" stuff
        nickname: Option<String>,
        is_glob: bool,
    },
    Loop {
        body: Vec<Stmt>,
    },
    Break,
    Expr(Expr),
    Block(Vec<Stmt>),
    Return(Option<Expr>),
}

impl Stmt {
    pub fn has_attr(&self, name: &str) -> bool {
        match self {
            Stmt::FnDecl { attributes, .. } => {
                attributes.iter().any(|attribute| match attribute {
                    Attribute::Simple(id) => id == name,
                    Attribute::WithArgs { name: attr_name, ..} => attr_name == name,
                    _ => false
                })
            }
            _ => false,
        }
    }

    pub fn get_attr(&self, target: &str) -> Option<&Attribute> {
        match self {
            Stmt::FnDecl { attributes, ..} => {
                attributes.iter().find(|attribute| match attribute {
                    Attribute::Simple(name) => name == target,
                    Attribute::WithArgs { name, .. } => name == target,
                    _ => false
                })
            }
            _ => None,
        }
    }
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
                        write!(f, "VarDecl(name: {}, type: {:?}, value: {})", name, typ, value_str)
                    }
            Stmt::Assign { name, value } => {
                        let value_str = format!("{:?}", value);
                        write!(f, "Assign(name: {}, value: {})", name, value_str)
                    }
            Stmt::FnCall { name, args } => {
                        write!(f, "FnCall(name: {}, args: {:?})", name, args)
                    }
            Stmt::FnDecl { name, args, body, attributes, ret_type, is_extern } => {
                        write!(f, "FnDecl(name: {}, args: {:?}, body: {:?}, attributes: {:?}, ret_type: {:?}, is_extern: {})", name, args, body, attributes, ret_type, is_extern)
                    }
            Stmt::Import { path, nickname, is_glob } => {
                        if let Some(nickname) = nickname {
                            write!(f, "Import(name: {:?}, nickname: {})", path, nickname)
                        } else {
                            write!(f, "Import(name: {:?}, nickname: {:?})", path, nickname)
                        }
                    }
            Stmt::Loop { body } => {
                        write!(f, "Loop(body: {:?})", body)
                    }
            Stmt::Break => {
                        write!(f, "Break")
                    }
            Stmt::Expr(expr) => {
                        write!(f, "Expr(Expr: {:?})", expr)
                    }
            Stmt::Block(stmts) => {
                write!(f, "Block(stmts: {:?}", stmts)
            }
            Stmt::Return(expr) => {
                write!(f, "Return(expr: {:?})", expr)
            }
        }
    }
}
