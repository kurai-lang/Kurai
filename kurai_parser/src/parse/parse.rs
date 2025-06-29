use kurai_core::print_error;
use kurai_core::scope::Scope;
// use kurai_codegen::codegen::codegen::CodeGen;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::value::Value;
use kurai_stmt::stmt::{IfBranch, Stmt};
use kurai_token::token::token::Token;
use kurai_token::eat::eat;

use colored::Colorize;

use crate::GroupedParsers;

pub fn parse_expr(tokens: &[Token], pos: &mut usize, scope: &mut Scope, parsers: &GroupedParsers) -> Result<Stmt, String> {
    let token = tokens.get(*pos).ok_or("Unexpected end of tokens")?;

    match token {
        Token::Number(v) => {
            *pos += 1;
            Ok(Stmt::Literal(Value::Int(*v)))
        }

        Token::Float(v) => {
            *pos += 1;
            Ok(Stmt::Literal(Value::Float(*v)))
        }

        Token::StringLiteral(s) => {
            *pos += 1;
            Ok(Stmt::Literal(Value::Str(s.clone())))
        }

        Token::Bool(b) => {
            *pos += 1;
            Ok(Stmt::Literal(Value::Bool(*b)))
        }

        Token::Id(id) => {
            let name = id.clone();
            *pos += 1;

            if eat(&Token::OpenParenthese, tokens, pos) {
                let mut args = Vec::new();
                while !eat(&Token::CloseParenthese, tokens, pos) {
                    let arg = parse_expr(tokens, pos, scope, parsers)?;
                    args.push(arg);
                    eat(&Token::Comma, tokens, pos);
                }

                Ok(Stmt::FnCall {
                    callee: Box::new(Stmt::Id(name)),
                    args
                })
            } else {
                Ok(Stmt::Id(name))
            }
        }

        Token::OpenParenthese => {
            *pos += 1;
            let expr = parse_expr(tokens, pos, scope, parsers)?;

            if !eat(&Token::CloseParenthese, tokens, pos) {
                return Err("Expected `)`".to_string());
            }

            Ok(expr)
        }

        Token::OpenBracket => {
            // block expression
            let body = parsers.block_parser.parse_block(tokens, pos, &mut vec![], parsers, scope)?;
            Ok(Stmt::Block(body))
        }

        Token::Loop => {
            *pos += 1;
            let body = parse_expr(tokens, pos, scope, parsers)?;
            Ok(Stmt::Loop(Box::new(body)))
        }

        Token::Break => {
            *pos += 1;
            Ok(Stmt::Break)
        }

        Token::Return => {
            *pos += 1;
            let expr = if let Some(Token::Semicolon) = tokens.get(*pos) {
                *pos += 1;
                None
            } else {
                Some(Box::new(parse_expr(tokens, pos, scope, parsers)?))
            };
            Ok(Stmt::Return(expr))
        }

        Token::Let => {
            *pos += 1;
            let name = if let Some(Token::Id(id)) = tokens.get(*pos) {
                *pos += 1;
                id.clone()
            } else {
                return Err("Expected identifier after `let`".to_string());
            };

            let typ = if eat(&Token::Colon, tokens, pos) {
                if let Some(Token::Id(ty)) = tokens.get(*pos) {
                    *pos += 1;
                    Some(ty.clone())
                } else {
                    return Err("Expected type name after `:`".to_string());
                }
            } else {
                None
            };

            if !eat(&Token::Equal, tokens, pos) {
                return Err("Expected `=` in let expression".to_string());
            }

            let value = parse_expr(tokens, pos, scope, parsers)?;
            let then = if eat(&Token::Semicolon, tokens, pos) {
                Box::new(Stmt::Block(vec![])) // No `then`, just placeholder
            } else {
                Box::new(parse_expr(tokens, pos, scope, parsers)?)
            };

            Ok(Stmt::Let {
                name,
                typ,
                value: Some(Box::new(value)),
                then: Some(then),
            })
        }

        Token::If => {
            *pos += 1;
            let cond = parse_expr(tokens, pos, scope, parsers)?;
            let then = parse_expr(tokens, pos, scope, parsers)?;

            let else_branch = if eat(&Token::Else, tokens, pos) {
                Some(Box::new(parse_expr(tokens, pos, scope, parsers)?))
            } else {
                None
            };

            Ok(Stmt::If {
                else_: else_branch,
                branches: vec![IfBranch {
                    condition: Box::new(cond),
                    body: Box::new(then),
                }],
            })
        }

        _ => Err(format!("Unexpected token at pos {}: {:?}", pos, token)),
    }
}

pub fn parse_typed_arg(tokens: &[Token], pos: &mut usize) -> Option<TypedArg> {
    todo!()
}

pub fn parse_out_vec_expr(tokens: &[Token], scopes: &mut Scope, parsers: &GroupedParsers) -> Result<Vec<Stmt>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        let expr = parse_expr(tokens, &mut pos, scopes, parsers)
            .map_err(|e| format!("Failed to parse expression at pos {}: {}", pos, e))?;

        exprs.push(expr);

        // Eat comma if there's one, otherwise break
        if !eat(&Token::Comma, tokens, &mut pos) {
            break;
        }
    }

    Ok(exprs)
}

pub fn parse_out_vec_stmt(
    tokens: &[Token],
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        match parsers.stmt_parser.parse_stmt(
            tokens,
            &mut pos,
            discovered_modules, 
            parsers,
            scope
        ) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => {
                let token = tokens.get(pos).unwrap_or_else(|| panic!("EOF or invalid token"));
                print_error!("Parsing error at token {:?}: {}", token, e);
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("TOKENS: {:?}", tokens);
    }
    stmts
}
