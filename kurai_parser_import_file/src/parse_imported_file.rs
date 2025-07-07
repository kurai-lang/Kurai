use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_token::token::token::Token;
use kurai_ast::stmt::Stmt;

pub fn parse_imported_file(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    src: &str,
) -> Result<Stmt, String> {
    parsers.stmt_parser.parse_stmt(tokens, pos, discovered_modules, parsers, scope, src)
        .map_err(|_| "Failed to parse imported file content".to_string())
}
