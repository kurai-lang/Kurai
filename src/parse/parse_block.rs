use crate::{eat::eat, token::token::Token};

use super::{parse::parse_stmt, stmt::Stmt};

pub fn parse_block(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Vec<Stmt>, String> {
    if !eat(&Token::OpenBracket, tokens, pos) {
        return Err("Expected `{` at start of block".to_string());
    }

    let mut stmts = Vec::new();
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::CloseBracket => {
                *pos += 1;
                return Ok(stmts);
            }
            _ => {
                let stmt = parse_stmt(tokens, pos, discovered_modules)
                    .expect(&format!("Failed to parse statement at token {}", *pos));
                stmts.push(stmt)
            }
        }
    }

    Err("Expected `}`".to_string())
}
