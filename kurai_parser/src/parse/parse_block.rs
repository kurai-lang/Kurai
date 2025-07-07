use kurai_core::scope::Scope;
use kurai_ast::expr::Expr;
use kurai_ast::stmt::Stmt;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;
use crate::{ parse::{parse::parse_expr, parse_stmt::parse_stmt}, BlockParser, GroupedParsers };
use colored::Colorize;

pub struct BlockParserStruct;
impl BlockParser for BlockParserStruct {
    fn parse_block(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
        src: &str,
    ) -> Result<Vec<Stmt>, String> {
        parse_block(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope,
            src
        )
    }

    fn parse_block_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
        src: &str,
    ) -> Result<Stmt, String> {
        parse_block_stmt(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope,
            src
        )
    }
}

pub fn parse_block(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    src: &str,
) -> Result<Vec<Stmt>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err(format!("Expected `{{` at start of block, found {:?}", tokens.get(*pos)));
    }

    let mut stmts = Vec::new();
    while *pos < tokens.len() {
        match tokens.get(*pos) {
            Some(Token::CloseBracket) => {
                #[cfg(debug_assertions)]
                println!("{}: closing bracket detected, bumping pos", "debug".cyan().bold());

                *pos += 1;
                return Ok(stmts);
            }
            Some(Token::Else) => {
                // Don't parse 'else' inside a block, let parse_if_else handle it
                break;
            }
            Some(_) => {
                #[cfg(debug_assertions)]
                { println!(">> calling parse_stmt at pos {}: {:?}", *pos, tokens.get(*pos)); }

                let stmt = parsers.stmt_parser.parse_stmt(
                    tokens,
                    pos,
                    discovered_modules,
                    parsers,
                    scope,
                    src,
                )?;

                stmts.push(stmt);
            }
            None => return Err("Unexpected end of token stream while parsing block.".to_string()),
        }
    }

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` at end of block.".to_string());
    // }

    Ok(stmts)
}

pub fn parse_expr_block(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    src: &str,
) -> Result<Vec<Expr>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err(format!("Expected `{{` at start of block, found {:?}", tokens.get(*pos)));
    }

    let mut stmts: Vec<Stmt> = Vec::new();
    let mut final_expr: Option<Box<Expr>> = None;

    while let Some(token) = tokens.get(*pos) {
        match token {
            Token::CloseBracket => {
                #[cfg(debug_assertions)]
                println!("{}: found end of block at pos {}, stopping.", "debug".cyan().bold(), pos);

                *pos += 1;
                return Ok(vec![Expr::Block {
                    stmts,
                    final_expr,
                }]);
            }
            Token::Else => {
                // Don't parse 'else' inside a block, let parse_if_else handle it
                break;
            }
            Token::Semicolon => {
                *pos += 1; // just skip semicolon, it’s not an expression
                continue;
            }
            _ => {
                #[cfg(debug_assertions)]
                { 
                    println!("{}: calling parse_expr_block at pos {}: {:?}", "debug".cyan().bold(), *pos, tokens.get(*pos)); 
                    println!("{}: parsing statements", "debug".cyan().bold());
                }
                let old_pos = *pos;
                let stmt = parse_stmt(tokens, pos, discovered_modules, parsers, scope, src).unwrap();
                match stmt {
                    Stmt::Expr(expr) => {
                        #[cfg(debug_assertions)]
                        println!("{}: parsing final expressions", "debug".cyan().bold());
                        final_expr = Some(Box::new(expr));
                    }
                    _ => stmts.push(stmt)
                }

                if *pos == old_pos {
                    return Err(format!("⚠️ parse_stmt made no progress at pos = {}, token = {:?}", pos, token));
                }
            }
        }
    }

    Ok(vec![Expr::Block {
        stmts, final_expr
    }])
}

pub fn parse_block_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    src: &str,
) -> Result<Stmt, String> {
    parse_block(tokens, pos, discovered_modules, parsers, scope, src)
        .map(Stmt::Block)
}
