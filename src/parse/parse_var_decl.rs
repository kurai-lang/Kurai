use crate::token::token::Token;
use crate::parse::stmt::Stmt;
use crate::value::Value;
use crate::eat::eat;

pub fn parse_var_decl(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if !eat(&Token::Let, tokens, pos) {
        return None;
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return None,
    };

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

    // No semicolon, no ending
    // no ending, no food
    // no food, ded
    if !eat(&Token::Semicolon, tokens, pos) {
        return None;
    }

    // stands for... i forgot
    // oh btw typ does nothing, at least for now 
    Some(Stmt::VarDecl {
        name,
        typ: "int".to_string(), 
        value,
    })
}
