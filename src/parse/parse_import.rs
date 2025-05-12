use crate::{eat::eat, token::token::Token};

use super::stmt::Stmt;

pub fn parse_import(tokens: &[Token], pos: &mut usize) -> Option<Stmt>{
    if !eat(&Token::Import, tokens, pos) {
        return None;
    }
    
    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return None
    };

    if eat(&Token::As, tokens, pos) {
        let nickname = match tokens.get(*pos) {
            Some(Token::Id(nickname)) => {
                *pos += 1;
                Some(nickname.clone())
            }
            _ => return None
        };

        if !eat(&Token::Semicolon, tokens, pos) {
            return None;
        }

        Some(Stmt::Import {
            name,
            nickname,
        })
    } else {
        if !eat(&Token::Semicolon, tokens, pos) {
            return None;
        }

        let nickname = None;

        Some(Stmt::Import {
            name,
            nickname, // NOTE: Nickname is initialized as None here
        })
    }
}
