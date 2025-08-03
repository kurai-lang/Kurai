// use std::sync::Arc;
//
// use vyn_attr::attribute::Attribute;
// use vyn_core::scope::Scope;
// use vyn_ast::{expr::Expr, stmt::Stmt};
// use vyn_token::token::token::Token;
//
pub mod parse;
//
// pub trait ImportParser {
//     fn parse_import_decl(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
//     fn parse_imported_file(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
// }
//
// pub trait StmtParser {
//     fn parse_stmt(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
// }
//
// pub trait BlockParser {
//     fn parse_block(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Vec<Stmt>, String>;
//
//     fn parse_block_stmt(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
// }
//
// pub trait LoopParser {
//     fn parse_loop(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
//
//     fn parse_for_loop(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
//
//     fn parse_while_loop(
//         &self,
//         tokens: &[vyn_token::token::token::Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String>;
// }
//
// pub trait FunctionParser {
//     fn parse_fn_decl(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         attrs: Vec<Attribute>,
//         src: &str,
//     ) -> Result<Stmt, String>;
//     fn parse_fn_call(&self, tokens: &[Token], pos: &mut usize) -> Result<Expr, String>;
//     fn parse_attrs(&self, tokens: &[Token], pos: &mut usize) -> Result<Vec<Attribute>, String>;
// }
//
// #[derive(Clone)]
// pub struct GroupedParsers{
//     pub stmt_parser: Arc<dyn StmtParser + Send + Sync>,
//     pub fn_parser: Arc<dyn FunctionParser + Send + Sync>,
//     pub import_parser: Arc<dyn ImportParser + Send + Sync>,
//     pub block_parser: Arc<dyn BlockParser + Send + Sync>,
//     pub loop_parser: Arc<dyn LoopParser + Send + Sync>,
// }
//
// impl GroupedParsers {
//     pub fn new(
//         stmt_parser: Arc<dyn StmtParser + Send + Sync>,    
//         fn_parser: Arc<dyn FunctionParser + Send + Sync>,  
//         import_parser: Arc<dyn ImportParser + Send + Sync>,
//         block_parser: Arc<dyn BlockParser + Send + Sync>,  
//         loop_parser: Arc<dyn LoopParser + Send + Sync>,    
//     ) -> Self {
//         GroupedParsers {
//             stmt_parser,
//             fn_parser,
//             import_parser,
//             block_parser,
//             loop_parser,
//         }
//     }
// }
