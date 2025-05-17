use crate::token::token::Token;
use crate::parse::stmt::Stmt;
use crate::value::Value;
use crate::eat::eat;

pub fn parse_var_decl(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    if !eat(&Token::Let, tokens, pos) {
        return Err("Expected keyword `let`".to_string());
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return Err("Expected an identifier name after keyword `let`".to_string()),
    };

    if !eat(&Token::Equal, tokens, pos) {
        return Err(format!("Expected an equal sign after `{}`", name));
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
        Some(Token::Bool(v)) => {
            *pos += 1;
            Value::Bool(*v).into()
        }
        _ => return Err(format!("Unsupported value {:?}", tokens.get(*pos)))
    };

    // No semicolon, no ending
    // no ending, no food
    // no food, ded
    if !eat(&Token::Semicolon, tokens, pos) {
        if let Some(value) = value {
            return Err(format!("Expected a semicolon after `{:?}`", value));
        } else {
            return Err(format!("Expected a semicolon after `{:?}`", value));
        }
    }

    // stands for... i forgot
    // oh btw typ does nothing, at least for now 
    Ok(Stmt::VarDecl {
        name,
        typ: "int".to_string(), 
        value,
    })
}
