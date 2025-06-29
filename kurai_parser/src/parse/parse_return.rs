use kurai_core::scope::Scope;
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

use crate::{parse::parse::parse_expr, GroupedParsers};

pub fn parse_return(tokens: &[Token], pos: &mut usize, scope: &mut Scope, parsers: &GroupedParsers) -> Result<Stmt, String> {
    if !eat(&Token::Return, tokens, pos) {
        return Err("Expected `return` keyword".to_string());
    }

    let expr = Some(Box::new(parse_expr(tokens, pos, scope, parsers).unwrap()));

    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected a semicolon".to_string());
    }

    // NOTE: well... that was easier than i expected

    Ok(Stmt::Return(expr))
}
