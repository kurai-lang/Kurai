use crate::{eat::eat, token::token::Token, typedArg::TypedArg, value::Value};

use super::{expr::Expr, stmt::Stmt};

pub fn parse_fn_call(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    if let Some(Token::Id(id)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::OpenParenthese, tokens, pos) {
            return Err(format!("Expected an opening paranthese `(` after `{}`", id));
        }

        let mut args: Vec<TypedArg> = Vec::new();

        // Check for function calls args
        // for example, printf(1) would check for Token::Number(n)
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

        if !eat(&Token::CloseParenthese, tokens, pos) {
            return Err("Expected a closing paranthese `)` after passing in arguments".to_string());
        }

        if !eat(&Token::Semicolon, tokens, pos) {
            return Err(format!("Expected semicolon after attempting to call function `{}`", id));
        }

        Ok(Stmt::FnCall {
            name: id.to_string(),
            args,
        })
    } else {
        Err("Expected an identifier name".to_string())
    }
}
