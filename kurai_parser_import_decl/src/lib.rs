use kurai_core::scope::Scope;
use kurai_parser::{GroupedParsers, ImportParser};
use kurai_parser_import_file::parse_imported_file::parse_imported_file;
use kurai_ast::stmt::Stmt;
use kurai_token::token::token::Token;

use crate::parse_import_decl::parse_import_decl;

pub mod parse_import_decl;

pub struct ImportParserStruct;

impl ImportParser for ImportParserStruct {
    fn parse_import_decl(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
        parse_import_decl(tokens, pos, discovered_modules)
    }

    fn parse_imported_file(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_imported_file(tokens, pos, discovered_modules, parsers, scope)
    }
}
