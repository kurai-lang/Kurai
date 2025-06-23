use std::io;

use kurai_attr::attribute::{AttrArg, Attribute};
use kurai_token::{eat::eat, token::token::Token};
use kurai_types::value::Value;

pub fn parse_attrs(tokens: &[Token], pos: &mut usize) -> Result<Vec<Attribute>, String> {
    let mut attrs = Vec::new();

    while let Some(Token::Hash) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::OpenSquareBracket, tokens, pos) {
            return Err("Expected `[` after `#` in attribute".to_string());
        }

        let attr_name = match tokens.get(*pos) {
            Some(Token::Id(name)) => {
                *pos += 1;
                name.clone()
            }
            _ => return Err("Expected attribute".to_string())
        };

        // attribute args
        let mut args = Vec::new();
        parse_attr_args(tokens, pos, &mut args).unwrap();

        if !eat(&Token::CloseSquareBracket, tokens, pos) {
            return Err("Expecetd `]` to close attribute".to_string());
        }

        if args.is_empty() {
            attrs.push(Attribute::Simple(attr_name));
        } else {
            attrs.push(Attribute::WithArgs { name: attr_name, args });
        }
    }

    Ok(attrs)
}

fn parse_attr_args(tokens: &[Token], pos: &mut usize, args: &mut Vec<AttrArg>) -> Result<(), String> {
    if eat(&Token::OpenParenthese, tokens, pos) {
        while !eat(&Token::CloseParenthese, tokens, pos) {
            match tokens.get(*pos) {
                Some(Token::Id(key)) => {
                    *pos += 1;

                    // Check if this key is equal to something (basically, like a variable decl)
                    if eat(&Token::Equal, tokens, pos) {
                        let value = match tokens.get(*pos) {
                            Some(Token::Id(s)) => { *pos += 1; s.clone() },
                            Some(Token::StringLiteral(s)) => { *pos += 1; s.clone() },
                            _ => return Err("Expected value after `=` in attribute arguments".to_string()),
                        };

                        args.push(AttrArg::Named(key.clone(), Value::Str(value)));
                    } else {
                        // Arguments that behaves more like functions, like #[route("yes")]
                        args.push(AttrArg::Positional(key.clone()));
                    }
                }

                Some(Token::StringLiteral(s)) => {
                    args.push(AttrArg::Positional(s.clone()));
                    *pos += 1;
                }

                Some(t) => return Err(format!("Unexpected token in attribute args: {:?}", t)),
                None => return Err("Unexpected end in attribute args".to_string())
            }

            eat(&Token::Comma, tokens, pos);
        }
    }

    Ok(())
}
