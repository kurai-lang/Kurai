use kurai_parser::{FunctionParser, ImportParser, StmtParser};
use kurai_parser_import_file::parse_imported_file::parse_imported_file;
use kurai_token::token::token::Token;

use crate::parse_import_decl::parse_import_decl;

pub mod parse_import_decl;

pub struct ImportParserStruct;

impl ImportParser for ImportParserStruct {
    fn parse_import_decl(&self, tokens: &[kurai_token::token::token::Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_import_decl(tokens, pos, discovered_modules)
    }

    fn parse_imported_file(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        stmt_parser: &dyn StmtParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        ) -> Result<kurai_stmt::stmt::Stmt, String> {
        parse_imported_file(tokens, pos, discovered_modules, stmt_parser, fn_parser, import_parser)
    }
}
