use kurai_core::scope::Scope;
use kurai_parser::{parse::parse_block, BlockParser, FunctionParser, ImportParser, LoopParser};
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

pub struct LoopParserStruct;
impl LoopParser for LoopParserStruct {
    fn parse_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        block_parser: &dyn BlockParser,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<Stmt, String> {
        parse_loop(tokens, pos, block_parser, discovered_modules, fn_parser, import_parser, loop_parser, scope)
    }
}

pub fn parse_loop(
    tokens: &[Token],
    pos: &mut usize,
    block_parser: &dyn BlockParser,
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::Loop, tokens, pos) {
        return Err("Expected `loop`".to_string());
    }

    // if !eat(&Token::OpenBracket, tokens, pos) {
    //     return Err("Expected `{` after `loop`".to_string());
    // }

    let body = block_parser.parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope).unwrap();
    // *pos += 1;

    // if !eat(&Token::CloseBracket, tokens, pos) {
    //     return Err("Expected `}` after body".to_string());
    // }

    Ok(Stmt::Loop { 
        body
    })
}
