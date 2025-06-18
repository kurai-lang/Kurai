use kurai_core::scope::Scope;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use crate::{BlockParser, FunctionParser, ImportParser, LoopParser};

use super::parse::parse_expr;
use super::parse_block::parse_block;
use kurai_stmt::stmt::{ IfBranch, Stmt };

pub fn parse_if_else(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::If, tokens, pos) {
        return Err("Expected keyword `if`".to_string());
    }

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return Err("Expected `(` after `if`".to_string());
    }

    let condition = parse_expr(tokens, pos, true)
        .ok_or_else(|| format!("Failed to parse expression inside `if (...)` at token {}", pos))?;

    if !eat(&Token::CloseParenthese, tokens, pos) {
        return Err("Expected `)` after `if` condition".to_string());
    }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` at start of block".to_string());
    // }

    println!("Before {{");

    let then_branch = block_parser.parse_block(
        tokens,
        pos,
        discovered_modules,
        block_parser,
        fn_parser,
        import_parser,
        loop_parser,
        scope,
    )?;

    println!("After }}");

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` at start of block".to_string());
    // }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` at start of block".to_string());
    // }

    println!("works");
    let mut else_body: Option<Vec<Stmt>> = None;
    if eat(&Token::Else, tokens, pos) {
        println!("[before parse_block()] token[{}] = {:?}", *pos, tokens.get(*pos));

        else_body = Some(block_parser.parse_block(
            tokens,
            pos,
            discovered_modules, 
            block_parser,
            fn_parser,
            import_parser,
            loop_parser,
            scope
        )?);
    }

    // let else_body = if eat(&Token::Else, tokens, pos) {
    //     Some(block_parser.parse_block(
    //         tokens,
    //         pos,
    //         discovered_modules,
    //         block_parser,
    //         fn_parser,
    //         import_parser,
    //         loop_parser,
    //         scope,
    //     )?)
    // } else {
    //     None
    // };

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` at start of block".to_string());
    // }

    Ok(Stmt::If {
        branches: vec![IfBranch {
            condition,
            body: then_branch,
        }],
        else_body,
    })
}
