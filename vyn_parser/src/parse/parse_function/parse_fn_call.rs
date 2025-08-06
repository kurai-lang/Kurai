use vyn_ast::expr::Expr;
use vyn_token::eat::eat;
use vyn_token::token::token::Token;
use vyn_types::value::Value;

use crate::parse::Parser;

impl Parser {
    pub fn parse_fn_call(&mut self) -> Result<Expr, String> {
        let mut path = Vec::new();

        // step 1: parse full path, like foo::bar()
        loop {
            match &self.tokens.get(self.pos) {
                Some(Token::Id(name)) => {
                    path.push(name.clone());
                    self.pos += 1;
                }
                _ => break
            }

            if eat(&Token::Colon, &self.tokens, &mut self.pos)
            && eat(&Token::Colon, &self.tokens, &mut self.pos) {
                continue;
            } else {
                break;
            }
        }

        if path.is_empty() {
            return Err("Expected function name".to_string());
        }

        // if !eat(&Token::OpenParenthese, &self.tokens, self.pos) {
        //     return Err("Expected `(` after function name".to_string());
        // }

        let args = self.parse_args()?;

        if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
            return Err("Expected `;` after function call".to_string());
        }

        Ok(Expr::FnCall {
            name: path.join("::"),
            args,
        })
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if !eat(&Token::OpenParenthese, &self.tokens, &mut self.pos) {
            return Err("Expected `(` in function call".to_string());
        }

        loop {
            match self.tokens.get(self.pos) {
                Some(Token::Number(v)) => {
                    args.push(Expr::Literal(Value::Int(*v)));
                    self.pos += 1;
                }
                Some(Token::Float(v)) => {
                    args.push(Expr::Literal(Value::Float(*v)));
                    self.pos += 1;
                }
                Some(Token::StringLiteral(s)) => {
                    args.push(Expr::Literal(Value::Str(s.clone())));
                    self.pos += 1;
                }
                Some(Token::Id(id)) => {
                    args.push(Expr::Id(id.clone()));
                    self.pos += 1;
                }
                Some(Token::Comma) => {
                    self.pos += 1; // skip commas
                    continue;
                }
                Some(Token::CloseParenthese) => {
                    self.pos += 1; // consume `)`
                    break;
                }
                Some(unexpected) => {
                    return Err(format!("Unexpected token in args: {unexpected:?}"));
                }
                None => {
                    return Err("Unexpected end of tokens while parsing args".to_string());
                }
            }
        }

        Ok(args)
    }

    // Reference:
    /*
    ```
    while let Some(token) = tokens.get(*pos) {
        match token {
            Token::Number(v) => {
                args.push(TypedArg {
                    name: id.to_string(),
                    typ: "int".to_string(),
                    value: Some(Expr::Literal(Value::Int(*v)))
                });
                *pos += 1;
            }
            Token::StringLiteral(s) => {
                args.push(TypedArg {
                    name: "_".to_string(), // Using `_` as a placeholder fr
                    typ: "str".to_string(),
                    value: Some(Expr::Literal(Value::Str(s.clone()))),
                });
                *pos += 1; // now it skips the string and moves on to the next thing in
                            // token
            }
            Token::Id(id) => {
                args.push(TypedArg {
                    name: id.to_string(),
                    typ: "id".to_string(),
                    value: None,
                });
                *pos += 1;
            }
            Token::Comma => {
                *pos += 1;  // skips comma by moving the cursor (pos) hehe
                continue;
            }

            // This marks the end of args
            Token::CloseParenthese => break,
            // _ => panic!("Unexpected token in function call args: {:?}", token)
            _ => eprintln!("Unexpected token at pos: {}, {:?} \n {:?}", *pos, tokens.get(*pos),tokens)
        }
    }
    ```
    */
}
