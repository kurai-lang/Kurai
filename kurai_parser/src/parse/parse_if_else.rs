use kurai_ast::expr::{Expr, IfBranch};
use kurai_core::scope::Scope;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use crate::parse::parse_block::parse_expr_block;
use crate::GroupedParsers;

use super::parse::parse_expr;

pub fn parse_if_else(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Expr, String> {
    if !eat(&Token::If, tokens, pos) {
        return Err("Expected keyword `if`".to_string());
    }
    // check if ( exists; if it does, parse inside it; otherwise, parse the next expression directly.

    let condition = if eat(&Token::OpenParenthese, tokens, pos) {
        let cond = parse_expr(tokens, pos, true, discovered_modules, parsers, scope)
            .ok_or_else(|| format!("Failed to parse expression inside `if (...)` at token {}", pos))?;

        if !eat(&Token::CloseParenthese, tokens, pos) {
            return Err("Expected `)` after `if` condition".to_string());
        }

        cond
    } else {
        parse_expr(tokens, pos, true, discovered_modules, parsers, scope)
            .ok_or_else(|| format!("Failed to parse expression after `if` at token {}", pos))?
    };


    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` at start of block".to_string());
    // }

    let then_branch = parse_expr_block(
        tokens,
        pos,
        discovered_modules,
        parsers,
        scope,
    )?;

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` at start of block".to_string());
    // }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` at start of block".to_string());
    // }

    let else_body = if eat(&Token::Else, tokens, pos) {
        Some(parse_expr_block(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope,
        )?)
    } else {
        None
    };

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` at start of block".to_string());
    // }

    Ok(Expr::If {
        branches: vec![IfBranch {
            condition,
            body: then_branch,
        }],
        else_body,
    })
}
