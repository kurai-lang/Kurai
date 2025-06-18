use kurai_core::scope::Scope;
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
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Vec<Stmt>, String> {
        parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }

    fn parse_block_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_block_stmt(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }
}

pub fn parse_block(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
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
                println!(">> calling parse_stmt at pos {}: {:?}", *pos, tokens.get(*pos));

                let stmt = parse_stmt(
                    tokens,
                    pos,
                    discovered_modules,
                    block_parser,
                    fn_parser,
                    import_parser,
                    loop_parser,
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

pub fn parse_block_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
        .map(Stmt::Block)
}
