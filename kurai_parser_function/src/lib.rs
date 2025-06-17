pub mod parse_fn_call;
pub mod parse_fn_decl;
use kurai_core::scope::Scope;
use kurai_parser::{BlockParser, FunctionParser, ImportParser, LoopParser};
use kurai_token::token::token::Token;
use crate::{parse_fn_call::parse_fn_call, parse_fn_decl::parse_fn_decl};

pub struct FunctionParserStruct;

impl FunctionParser for FunctionParserStruct {
    fn parse_fn_decl(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_decl(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }

    fn parse_fn_call(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_call(tokens, pos)
    }
}
