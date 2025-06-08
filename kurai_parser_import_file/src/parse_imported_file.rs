use std::collections::HashMap;

use kurai_token::token::token::Token;
use kurai_parser::parse::parse::parse_stmt;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;

pub fn parse_imported_file(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    parse_stmt(tokens, pos, discovered_modules)
        .map_err(|_| "Failed to parse imported file content".to_string())
}
