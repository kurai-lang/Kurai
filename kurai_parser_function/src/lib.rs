pub mod parse_fn_call;
pub mod parse_fn_decl;
use kurai_parser_stmt::parse_stmt::FunctionParser;
use crate::parse_fn_decl::parse_fn_decl;

pub struct FunctionParserStruct;

impl FunctionParser for FunctionParserStruct {
    fn parse_fn_decl(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_decl(tokens, pos, discovered_modules)
    }
}
