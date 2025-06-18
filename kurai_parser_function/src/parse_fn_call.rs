use std::net::SocketAddr;

use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::typ::Type;
use kurai_types::value::Value;

pub fn parse_fn_call(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    let mut path = Vec::new();

    // step 1: parse full path, like foo::bar()
    loop {
        match tokens.get(*pos) {
            Some(Token::Id(name)) => {
                path.push(name.clone());
                *pos += 1;
            }
            _ => break
        }

        if eat(&Token::Colon, tokens, pos)
        && eat(&Token::Colon, tokens, pos) {
            continue;
        } else {
            break;
        }
    }

    if path.is_empty() {
        return Err("Expected function name".to_string());
    }

    // if !eat(&Token::OpenParenthese, tokens, pos) {
    //     return Err("Expected `(` after function name".to_string());
    // }

    let args = parse_args(tokens, pos)?;

    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected `;` after function call".to_string());
    }

    Ok(Stmt::FnCall {
        name: path.join("::"),
        args,
    })
}

fn parse_args(tokens: &[Token], pos: &mut usize) -> Result<Vec<TypedArg>, String> {
    let mut args = Vec::new();

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return Err("Expected `(` in function call".to_string());
    }

    loop {
        match tokens.get(*pos) {
            Some(Token::Number(v)) => {
                args.push(TypedArg {
                    name: "_".to_string(),
                    typ: Type::Int,
                    value: Some(Expr::Literal(Value::Int(*v))),
                });
                *pos += 1;
            }
            Some(Token::StringLiteral(s)) => {
                args.push(TypedArg {
                    name: "_".to_string(),
                    typ: Type::Str,
                    value: Some(Expr::Literal(Value::Str(s.clone()))),
                });
                *pos += 1;
            }
            Some(Token::Id(id)) => {
                args.push(TypedArg {
                    name: id.to_string(),
                    typ: Type::Var,
                    value: Some(Expr::Var(id.clone())),
                });
                *pos += 1;
            }
            Some(Token::Comma) => {
                *pos += 1; // skip commas
                continue;
            }
            Some(Token::CloseParenthese) => {
                *pos += 1; // consume `)`
                break;
            }
            Some(unexpected) => {
                return Err(format!("Unexpected token in args: {:?}", unexpected));
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
