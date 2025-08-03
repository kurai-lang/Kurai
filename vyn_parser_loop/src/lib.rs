// use vyn_core::scope::Scope;
// use vyn_parser::{parse::parse_block::{parse_block, parse_block_stmt}, BlockParser, GroupedParsers, LoopParser};
// use vyn_ast::stmt::Stmt;
// use vyn_token::token::token::Token;
//
// use crate::parse_for_loop::parse_for_loop;
// use crate::parse_loop::parse_loop;
// use crate::parse_while_loop::parse_while_loop;
//
// pub mod parse_loop;
// pub mod parse_for_loop;
// pub mod parse_while_loop;
//
// pub struct BlockParserStruct;
// impl BlockParser for BlockParserStruct {
//     fn parse_block(
//         &self,
//         tokens: &[vyn_token::token::token::Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Vec<Stmt>, String> {
//         parse_block(tokens, pos, discovered_modules, parsers, scope, src)
//     }
//
//     fn parse_block_stmt(
//         &self,
//         tokens: &[vyn_token::token::token::Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_block_stmt(tokens, pos, discovered_modules, parsers, scope, src)
//     }
// }
//
// pub struct LoopParserStruct;
// impl LoopParser for LoopParserStruct {
//     fn parse_loop(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_loop(tokens, pos, discovered_modules, parsers, scope, src)
//     }
//
//     fn parse_for_loop(
//         &self,
//         tokens: &[vyn_token::token::token::Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_for_loop(tokens, pos, discovered_modules, parsers, scope, src)
//     }
//
//     fn parse_while_loop(
//         &self,
//         tokens: &[vyn_token::token::token::Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_while_loop(tokens, pos, discovered_modules, parsers, scope, src)
//     }
// }
