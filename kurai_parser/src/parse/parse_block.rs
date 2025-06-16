use kurai_token::token::token::Token;
use kurai_token::eat::eat;
use kurai_stmt::stmt::Stmt;
use crate::parse::parse_stmt::parse_stmt;
use crate::{BlockParser, FunctionParser, ImportParser, LoopParser};

pub struct BlockParserStruct;
impl BlockParser for BlockParserStruct {
    fn parse_block(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser
    ) -> Result<Vec<Stmt>, String> {
        parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser)
    }
}

pub fn parse_block(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser
) -> Result<Vec<Stmt>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err("Expected `{` at start of block".to_string());
    }

    let mut stmts = Vec::new();
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::CloseBracket => {
                *pos += 1;
                return Ok(stmts);
            }
            _ => {
                let stmt = parse_stmt(
                        tokens,
                        pos,
                        discovered_modules,
                        block_parser,
                        fn_parser,
                        import_parser,
                        loop_parser
                    )
                    .expect(&format!("Failed to parse statement at token {}", *pos));
                stmts.push(stmt)
            }
        }
    }

    Err("Expected `}`".to_string())
}
