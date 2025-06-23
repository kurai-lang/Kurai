pub mod parse_fn_call;
pub mod parse_fn_decl;
pub mod parse_attrs;

use kurai_attr::attribute::Attribute;
use kurai_core::scope::Scope;
use kurai_parser::{FunctionParser, GroupedParsers};
use kurai_token::token::token::Token;
use crate::{parse_attrs::parse_attrs, parse_fn_call::parse_fn_call, parse_fn_decl::parse_fn_decl};

pub struct FunctionParserStruct;

impl FunctionParser for FunctionParserStruct {
    fn parse_fn_decl(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
        attrs: Vec<Attribute>,
    ) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_decl(tokens, pos, discovered_modules, parsers, scope, attrs)
    }

    fn parse_fn_call(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_fn_call(tokens, pos)
    }

    fn parse_attrs(&self, tokens: &[Token], pos: &mut usize) -> Result<Vec<kurai_attr::attribute::Attribute>, String> {
        parse_attrs(tokens, pos)
    }
}
