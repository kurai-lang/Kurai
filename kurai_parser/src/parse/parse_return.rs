use kurai_ast::stmt::Stmt;
use kurai_core::scope::Scope;
use kurai_token::{eat::eat, token::token::Token};

use crate::{parse::parse::parse_expr, GroupedParsers};

pub fn parse_return(
    tokens: &[Token], 
    pos: &mut usize, 
    discovered_modules: &mut Vec<String>, 
    parsers: &GroupedParsers, 
    scope: &mut Scope,
    src: &str
) -> Result<Stmt, String> {
    if !eat(&Token::Return, tokens, pos) {
        return Err("Expected `return` keyword".to_string());
    }

    let expr = parse_expr(tokens, pos, true, discovered_modules, parsers, scope, src);

    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected a semicolon".to_string());
    }

    // NOTE: well... that was easier than i expected

    Ok(Stmt::Return(expr))
}
