pub mod parse_fn_call;
pub mod parse_fn_decl;
use kurai_parser::{FunctionParser, ImportParser};
use crate::{parse_fn_call::parse_fn_call, parse_fn_decl::parse_fn_decl};

pub struct FunctionParserStruct;

impl FunctionParser for FunctionParserStruct {
    fn parse_fn_decl(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize, discovered_modules: &mut Vec<String>, fn_parser: &dyn FunctionParser, import_parser: &dyn ImportParser) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_decl(tokens, pos, discovered_modules, fn_parser, import_parser)
    }

    fn parse_fn_call(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_call(tokens, pos)
    }
}
