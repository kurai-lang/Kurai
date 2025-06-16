use kurai_parser::{parse::parse_block::parse_block, BlockParser, LoopParser};

pub mod parse_loop;

pub struct BlockParserStruct;
impl BlockParser for BlockParserStruct {
    fn parse_block(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn kurai_parser::FunctionParser,
        import_parser: &dyn kurai_parser::ImportParser,
        loop_parser: &dyn LoopParser,
    ) -> Result<Vec<kurai_stmt::stmt::Stmt>, String> {
        parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser)
    }
}
