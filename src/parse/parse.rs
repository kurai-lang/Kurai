use std::fmt::Debug;

use crate::codegen::codegen::CodeGen;
use crate::eat::eat;
use crate::token::token::Token;
use crate::parse::stmt::Stmt;
use crate::parse::parse_var_decl::parse_var_decl;
use crate::typedArg::TypedArg;
use crate::value::Value;

use super::bin_op::BinOp;
use super::expr::{self, Expr};
use super::parse_block::{self, parse_block};
use super::parse_expr::parse_equal::parse_equal;
use super::parse_fn_call::parse_fn_call;
use super::parse_fn_decl::parse_fn_decl;
use super::parse_if_else::parse_if_else;
use super::parse_import::parse_import_decl::parse_import_decl;
use super::parse_var_assign::parse_var_assign;

// this function just wants to return stmt
// this function practically just runs whatever function here whenever the program encounters
// one of the tokens
pub fn parse_stmt(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    match tokens.get(*pos) {
        Some(Token::Function) => parse_fn_decl(tokens, pos, discovered_modules),
        Some(Token::Let) => parse_var_decl(tokens, pos),
        Some(Token::Import) => parse_import_decl(tokens, pos, discovered_modules),
        Some(Token::If) => parse_if_else(tokens, pos, discovered_modules),
        Some(Token::Id(_)) => {
            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => parse_fn_call(tokens, pos),
                Some(Token::Equal) => parse_var_assign(tokens, pos),
                _ => Err("Identifier expected, is this supposed to be a function call or variable assignment?".to_string())
            }
        }
        _ => match parse_expr(tokens, pos) {
            Some(Expr::FnCall { name, args }) => {
                let typed_args = args.into_iter().map(|arg|
                    TypedArg {
                        name: name.clone(),
                        typ: "any".to_string(),
                        value: Some(arg),
                    }).collect();

                Ok(Stmt::FnCall { name, args: typed_args })
            }
            Some(expr) => Err(format!("Standalone expressions not allowed: {:?}", expr)),
            None => Err(format!("Invalid statement: {:?}", tokens.get(*pos)))
        }
    }
}

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Option<Expr> {
    // parse_equal(tokens, pos)
    let mut left = match tokens.get(*pos)? {
        Token::Number(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Int(*v)))
        }
        Token::StringLiteral(v) => {
            *pos += 1;
            let v = v.clone();
            Some(Expr::Literal(Value::Str(v)))
        }
        Token::Bool(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Bool(*v)))
        }
        Token::Id(id) => {
            let name = id.clone();
            *pos += 1;

            if eat(&Token::OpenParenthese, tokens, pos) {
                let mut args = Vec::new();
                while !eat(&Token::CloseParenthese, tokens, pos) {
                    if let Some(arg) = parse_expr(tokens, pos) {
                        args.push(arg);
                        eat(&Token::Comma, tokens, pos);
                    } else {
                        return None;
                    }
                }
                Some(Expr::FnCall { 
                    name,
                    args
                })
            } else {
                Some(Expr::Var(name))
            }
        }
        Token::OpenParenthese => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            eat(&Token::CloseParenthese, tokens, pos);
            Some(expr)
        }
        _ => {
            *pos += 1;
            None
        }
    }?;

    while let Some(Token::EqualEqual) = tokens.get(*pos) {
        *pos += 1;
        let right = parse_expr(tokens, pos)?;

        left = Expr::Binary { 
            op: BinOp::Eq,
            left: Box::new(left),
            right: Box::new(right)
        };
    }
    Some(left)
}

pub fn parse_typed_arg(tokens: &[Token], pos: &mut usize) -> Option<TypedArg> {
    todo!()
}

pub fn parse_out_vec_expr(tokens: &[Token]) -> Result<Vec<Expr>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        println!("{}", pos);
        if let Some(expr) = parse_expr(tokens, &mut pos) {
            exprs.push(expr);
            if eat(&Token::Comma, tokens, &mut pos) { continue; }
        } else {
            eprintln!("Parse error at expression {:?} {}", exprs.get(pos), pos);
        }
    }

    Ok(exprs)
}

pub fn parse_out_vec_stmt(tokens: &[Token], discovered_modules: &mut Vec<String>) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        match parse_stmt(tokens, &mut pos, discovered_modules) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => panic!("Parse error at token {:?}: {}\n {:?}", tokens.get(pos), e, tokens)
        }
    }

    println!("{:?}", tokens);
    stmts
}
