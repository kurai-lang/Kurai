use crate::{eat::eat, token::token::Token};

use super::{parse::parse_stmt, stmt::Stmt};

pub fn parse_fn_decl(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if !eat(&Token::Function, tokens, pos) {
        return None;
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return None
    };

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return None;
    }

    // TODO: Add arguments passing here later 
    
    if !eat(&Token::CloseParenthese, tokens, pos) {
        return None;
    }

    let mut body = Vec::new();
    if eat(&Token::OpenBracket, tokens, pos) {
        while *pos < tokens.len() {
            if let Some(Token::CloseBracket) = tokens.get(*pos) {
                *pos += 1;
                break;
            }

            if let Some(stmt) = parse_stmt(tokens, pos) {
                body.push(stmt);
            } else {
                return None;
            }
        }
    } else {
        return None;
    }

    Some(Stmt::FnDecl { 
        name,
        args: vec![], // bro got skipped 💀
        body,
    })
}
