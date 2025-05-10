use crate::{eat::eat, token::token::Token, typedArg::TypedArg, value::Value};

use super::{expr::Expr, stmt::Stmt};

pub fn parse_fn_call(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Id(id)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::OpenParenthese, tokens, pos) {
            panic!("Expected '(' after function name");
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
                Token::Comma => {
                    *pos += 1;  // skips comma by moving the cursor (pos) hehe
                    continue;
                }

                // This marks the end of args
                Token::CloseParenthese => break,
                _ => panic!("Unexpected token in function call args: {:?}", token)
            }
        }

        if !eat(&Token::CloseParenthese, tokens, pos) {
            panic!("Expected '(' after ')'");
        }

        if !eat(&Token::Semicolon, tokens, pos) {
            return None;
        }

        Some(Stmt::FnCall {
            name: id.to_string(),
            args,
        })
    } else {
        None
    }
}
