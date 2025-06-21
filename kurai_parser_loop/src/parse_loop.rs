use kurai_core::scope::Scope;
use kurai_parser::{parse::parse_block, BlockParser, FunctionParser, ImportParser, LoopParser};
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

use crate::parse_for_loop::parse_for_loop;

pub fn parse_loop(
    tokens: &[Token],
    pos: &mut usize,
    block_parser: &dyn BlockParser,
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::Loop, tokens, pos) {
        return Err("Expected `loop`".to_string());
    }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` after `loop`".to_string());
    // }

    let body = block_parser.parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)?;
    // *pos += 1;

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` after body".to_string());
    // }

    Ok(Stmt::Loop { 
        body
    })
}
