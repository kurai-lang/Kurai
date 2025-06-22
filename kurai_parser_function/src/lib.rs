pub mod parse_fn_call;
pub mod parse_fn_decl;
use kurai_core::scope::Scope;
use kurai_parser::{BlockParser, FunctionParser, GroupedParsers, ImportParser, LoopParser};
use kurai_token::token::token::Token;
use crate::{parse_fn_call::parse_fn_call, parse_fn_decl::parse_fn_decl};

pub struct FunctionParserStruct;

impl FunctionParser for FunctionParserStruct {
    fn parse_fn_decl(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_decl(tokens, pos, discovered_modules, parsers, scope)
    }

    fn parse_fn_call(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_call(tokens, pos)
    }
}
