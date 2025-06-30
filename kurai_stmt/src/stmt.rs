// NOTE: OK MAYBE THE WHOLE THING IS DISCONTINUED.
// use kurai_attr::attribute::Attribute;
// use crate::scope::Scope;
// use kurai_typedArg::typedArg::TypedArg;
// use kurai_expr::expr::Expr;
// use kurai_types::typ::Type;
// use std::fmt;

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
