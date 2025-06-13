use kurai_parser::StmtParser;
use kurai_token::token::token::Token;
use kurai_stmt::stmt::Stmt;

pub fn parse_imported_file(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>, stmt_parser: &dyn StmtParser) -> Result<Stmt, String> {
    stmt_parser.parse_stmt(tokens, pos, discovered_modules)
        .map_err(|_| "Failed to parse imported file content".to_string())
}
