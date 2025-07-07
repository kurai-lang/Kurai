use kurai_attr::attribute::{AttrArg, Attribute};
use crate::parse::Parser;
use kurai_token::{eat::eat, token::token::Token};
use kurai_types::value::Value;

impl Parser {
    pub fn parse_attrs(&mut self) -> Result<Vec<Attribute>, String> {
        let mut attrs = Vec::new();

        while let Some(Token::Hash) = self.tokens.get(self.pos) {
            self.pos += 1;

            if !eat(&Token::OpenSquareBracket, &self.tokens, &mut self.pos) {
                return Err("Expected `[` after `#` in attribute".to_string());
            }

            let attr_name = match self.tokens.get(self.pos) {
                Some(Token::Id(name)) => {
                    self.pos += 1;
                    name.clone()
                }
                _ => return Err("Expected attribute".to_string())
            };

            // attribute args
            let mut args = Vec::new();
            self.parse_attr_args(&mut args).unwrap();

            if !eat(&Token::CloseSquareBracket, &self.tokens, &mut self.pos) {
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

    fn parse_attr_args(&mut self, args: &mut Vec<AttrArg>) -> Result<(), String> {
        if eat(&Token::OpenParenthese, &self.tokens, &mut self.pos) {
            while !eat(&Token::CloseParenthese, &self.tokens, &mut self.pos) {
                match self.tokens.get(self.pos) {
                    Some(Token::Id(key)) => {
                        self.pos += 1;

                        // Check if this key is equal to something (basically, like a variable decl)
                        if eat(&Token::Equal, &self.tokens, &mut self.pos) {
                            let value = match self.tokens.get(self.pos) {
                                Some(Token::Id(s)) => { self.pos += 1; s.clone() },
                                Some(Token::StringLiteral(s)) => { self.pos += 1; s.clone() },
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
                        self.pos += 1;
                    }

                    Some(t) => return Err(format!("Unexpected token in attribute args: {:?}", t)),
                    None => return Err("Unexpected end in attribute args".to_string())
                }

                eat(&Token::Comma, &self.tokens, &mut self.pos);
            }
        }

        Ok(())
    }
}
