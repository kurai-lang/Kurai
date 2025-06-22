use kurai_core::scope::Scope;
use kurai_parser:: GroupedParsers;
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

pub fn parse_loop(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::Loop, tokens, pos) {
        return Err("Expected `loop`".to_string());
    }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` after `loop`".to_string());
    // }

    let body = parsers.block_parser.parse_block(
        tokens,
        pos,
        discovered_modules,
        parsers,
        scope
    )?;
    // *pos += 1;

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` after body".to_string());
    // }

    Ok(Stmt::Loop { 
        body
    })
}
