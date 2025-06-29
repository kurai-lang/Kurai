use kurai_core::scope::Scope;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use crate::GroupedParsers;

use super::parse::parse_expr;
use kurai_stmt::stmt::{ IfBranch, Stmt };

pub fn parse_if_else(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::If, tokens, pos) {
        return Err("Expected keyword `if`".to_string());
    }
    // check if ( exists; if it does, parse inside it; otherwise, parse the next expression directly.

    let condition = if eat(&Token::OpenParenthese, tokens, pos) {
        let cond = parse_expr(tokens, pos, scope, parsers).unwrap();

        if !eat(&Token::CloseParenthese, tokens, pos) {
            return Err("Expected `)` after `if` condition".to_string());
        }

        cond
    } else {
        parse_expr(tokens, pos, scope, parsers).unwrap()
    };

    let then_block = Box::new(Stmt::Block(
        parsers.block_parser.parse_block(
            tokens, pos, discovered_modules, parsers, scope
        )?
    ));

    let else_block = if eat(&Token::Else, tokens, pos) {
        Some(Box::new(Stmt::Block(
            parsers.block_parser.parse_block(
                tokens, pos, discovered_modules, parsers, scope
            )?
        )))
    } else {
        None
    };

    Ok(Stmt::If {
        branches: vec![IfBranch {
            condition: Box::new(condition),
            body: then_block,
        }],
        else_: else_block,
    })
}
