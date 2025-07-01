use kurai_core::scope::Scope;
use kurai_ast::expr::Expr;
use kurai_ast::stmt::Stmt;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;
use crate::{ parse::parse::parse_expr, BlockParser, GroupedParsers };

pub struct BlockParserStruct;
impl BlockParser for BlockParserStruct {
    fn parse_block(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Vec<Stmt>, String> {
        parse_block(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope
        )
    }

    fn parse_block_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_block_stmt(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope
        )
    }
}

pub fn parse_block(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Vec<Stmt>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err(format!("Expected `{{` at start of block, found {:?}", tokens.get(*pos)));
    }

    let mut stmts = Vec::new();
    while *pos < tokens.len() {
        match tokens.get(*pos) {
            Some(Token::CloseBracket) => {
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
) -> Result<Vec<Expr>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err(format!("Expected `{{` at start of block, found {:?}", tokens.get(*pos)));
    }

    let mut exprs = Vec::new();
    while *pos < tokens.len() {
        match tokens.get(*pos) {
            Some(Token::CloseBracket) => {
                *pos += 1;
                return Ok(exprs);
            }
            Some(Token::Else) => {
                // Don't parse 'else' inside a block, let parse_if_else handle it
                break;
            }
            Some(Token::Semicolon) => {
                *pos += 1; // just skip semicolon, it’s not an expression
                continue;
            }
            Some(_) => {
                #[cfg(debug_assertions)]
                { println!(">> calling parse_expr_block at pos {}: {:?}", *pos, tokens.get(*pos)); }

                let expr = parse_expr(
                    tokens,
                    pos,
                    false,
                    discovered_modules,
                    parsers,
                    scope
                ).unwrap();

                exprs.push(expr);
            }
            None => return Err("Unexpected end of token stream while parsing block.".to_string()),
        }
    }

    Ok(exprs)
}

pub fn parse_block_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    parse_block(tokens, pos, discovered_modules, parsers,scope)
        .map(Stmt::Block)
}
