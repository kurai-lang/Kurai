use kurai_attr::attribute::Attribute;
use kurai_token::{eat::eat, token::token::Token};

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

        if !eat(&Token::CloseSquareBracket, tokens, pos) {
            return Err("Expecetd `]` to close attribute".to_string());
        }

        attrs.push(Attribute::Simple(attr_name));
    }

    Ok(attrs)
}
