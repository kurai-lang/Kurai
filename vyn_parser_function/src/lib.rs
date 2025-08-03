// pub mod parse_fn_call;
// pub mod parse_fn_decl;
// pub mod parse_attrs;
//
// use vyn_ast::{expr::Expr, stmt::Stmt};
// use vyn_attr::attribute::Attribute;
// use vyn_core::scope::Scope;
// use vyn_parser::{FunctionParser, GroupedParsers};
// use vyn_token::token::token::Token;
// use crate::{parse_attrs::parse_attrs, parse_fn_call::parse_fn_call, parse_fn_decl::parse_fn_decl};
//
// pub struct FunctionParserStruct;

// impl FunctionParser for FunctionParserStruct {
//     fn parse_fn_decl(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         attrs: Vec<Attribute>,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_fn_decl(tokens, pos, discovered_modules, parsers, scope, attrs, src)
//     }
//
//     fn parse_fn_call(&self, tokens: &[vyn_token::token::token::Token], pos: &mut usize) -> Result<Expr, String> {
//         parse_fn_call(tokens, pos)
//     }
//
//     fn parse_attrs(&self, tokens: &[Token], pos: &mut usize) -> Result<Vec<vyn_attr::attribute::Attribute>, String> {
//         parse_attrs(tokens, pos)
//     }
// }
