use kurai_core::scope::Scope;
use kurai_expr::expr::{Expr, IfBranch};
use kurai_parser::{parse::{parse::parse_expr, parse_block::parse_expr_block}, GroupedParsers};
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

pub fn parse_while_loop(
    tokens: &[kurai_token::token::token::Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::While, tokens, pos) {    
        return Err("Expected `while`".to_string());
    }

    // if !eat(&Token::OpenParenthese, tokens, pos) {
    //     return Err("Expected `(` after `while`".to_string());
    // }

    println!("parsing conditions...");
    let condition = parse_expr(tokens, pos, true)
        .ok_or_else(|| format!("Failed to parse expression inside `while (...)` at token {}", pos))?;
    println!("parsing conditions succeed");

    // if !eat(&Token::CloseParenthese, tokens, pos) {
    //     return Err("Expected `)` after `while` condition".to_string());
    // }

    let body = parse_expr_block(
        tokens,
        pos,
        discovered_modules,
        parsers,
        scope,
    )?;

    Ok(Stmt::Block(vec![
        Stmt::Loop { 
            body: vec![
                Stmt::Expr(Expr::If {
                    branches: vec![IfBranch {
                        condition,
                        body,
                    }],
                    else_body: Some(vec![Stmt::Break]),
                })
            ]
        }]
    ))
}
