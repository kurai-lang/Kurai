use crate::{eat::eat, token::token::Token, value::Value};

use super::stmt::Stmt;

pub fn parse_var_assign(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    if let Some(Token::Id(id)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::Equal, tokens, pos) {
            return Err(format!("Expected an equal sign `=` after `{}`", id));
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
            _ => return Err(format!("Unsupported value: {:?}", tokens.get(*pos)))
        };

        if !eat(&Token::Semicolon, tokens, pos) {
            return Err(format!("Expected a semicolon `;` after `{:?}`", value));
        }

        Ok(Stmt::Assign {
            name: id.to_string(),
            value: value.unwrap(),
        })
    } else {
        return Err("Where identifier".to_string());
    }
}
