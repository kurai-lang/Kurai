use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

use crate::parse::parse::parse_expr;

pub fn parse_return(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    if !eat(&Token::Return, tokens, pos) {
        return Err("Expected `return` keyword".to_string());
    }

    let expr = parse_expr(tokens, pos, true);

    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected a semicolon".to_string());
    }

    // NOTE: well... that was easier than i expected

    Ok(Stmt::Return(expr))
}
