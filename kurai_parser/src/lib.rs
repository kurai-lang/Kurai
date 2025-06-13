use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;

pub mod parse;

pub trait ImportParser {
    fn parse_import_decl(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
    fn parse_imported_file(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>, stmt_parser: &dyn StmtParser) -> Result<Stmt, String>;
}

pub trait FunctionParser {
    fn parse_fn_decl(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
    fn parse_fn_call(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
}

pub trait StmtParser {
    fn parse_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        // fn_parser: &dyn Any,
        // parser: &dyn Any,
    ) -> Result<Stmt, String>;
}
