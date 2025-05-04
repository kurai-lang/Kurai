use crate::{eat::eat, token::token::Token, value::Value};

use super::stmt::Stmt;

pub fn parse_var_assign(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Id(id)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::Equal, tokens, pos) {
            return None;
        }

        let value: Option<Value> = match tokens.get(*pos) {
            Some(Token::Number(v)) => {
                *pos += 1;
                // Lets use .into() cuz im too lazy to use Some()
                Value::Int(*v).into()
            }
            Some(Token::Float(v)) => {
                *pos += 1;
                Value::Float(*v as f64).into()
            }
            Some(Token::Id(id)) => {
                *pos += 1;
                Value::Str(id.clone()).into()
            }
            _ => return None
        };

        if !eat(&Token::Semicolon, tokens, pos) {
            return None;
        }

        Some(Stmt::Assign {
            name: id.to_string(),
            value: value?,
        })
    } else {
        return None;
    }
}
