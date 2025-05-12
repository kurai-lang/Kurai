use crate::token::token::Token;
use crate::parse::stmt::Stmt;
use crate::parse::parse_var_decl::parse_var_decl;

use super::parse_fn_call::parse_fn_call;
use super::parse_fn_decl::parse_fn_decl;
use super::parse_import::parse_import;
use super::parse_var_assign::parse_var_assign;

// this function just wants to return stmt
pub fn parse_stmt(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    match tokens.get(*pos) {
        Some(Token::Function) => {
            parse_fn_decl(tokens, pos)
        }
        Some(Token::Let) => {
            parse_var_decl(tokens, pos)
        }
        Some(Token::Import) => {
            parse_import(tokens, pos)
        }
        Some(Token::Id(_)) => {
            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => {
                    parse_fn_call(tokens, pos)
                }
                Some(Token::Equal) => {
                    parse_var_assign(tokens, pos)
                }
                _ => None
            }
        }
        _ => None
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        if let Some(stmt) = parse_stmt(tokens, &mut pos) {
            stmts.push(stmt)
        } else {
            // if it returns None (from the parse_stmt function), we return an error 
            // indirect, but it gets the job done.. for now
            panic!("Parse error at token {:?}", tokens.get(pos));
        }
    }

    println!("{:?}", tokens);
    stmts
}
