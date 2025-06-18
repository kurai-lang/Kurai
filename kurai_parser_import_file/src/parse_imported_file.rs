use kurai_core::scope::Scope;
use kurai_parser::{BlockParser, FunctionParser, ImportParser, LoopParser, StmtParser};
use kurai_token::token::token::Token;
use kurai_stmt::stmt::Stmt;

pub fn parse_imported_file(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    stmt_parser: &dyn StmtParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    block_parser: &dyn BlockParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    stmt_parser.parse_stmt(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
        .map_err(|_| "Failed to parse imported file content".to_string())
}
