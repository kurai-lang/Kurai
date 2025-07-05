use kurai_core::scope::Scope;
// use kurai_codegen::codegen::codegen::CodeGen;
use kurai_types::value::Value;
use kurai_ast::stmt::Stmt;
use kurai_ast::expr::Expr;
use kurai_ast::typedArg::TypedArg;
use kurai_binop::bin_op::BinOp;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;

use crate::parse::parse_expr::parse_arithmetic::parse_arithmetic;
use crate::parse::parse_if_else::parse_if_else;
use crate::parse::parse_stmt::parse_stmt;
use crate::GroupedParsers;

pub fn parse_expr(tokens: &[Token], pos: &mut usize, in_condition: bool, discovered_modules: &mut Vec<String>, parsers: &GroupedParsers, scope: &mut Scope) -> Option<Expr> {
    // parse_equal(tokens, pos)
    // match tokens.get(*pos) {
    //     Some(Token::If) => parse_if_else(tokens, pos, discovered_modules, parsers, scope).ok(),
    //     _ => None
    // };
    let mut left = match tokens.get(*pos)? {
        Token::If => parse_if_else(tokens, pos, discovered_modules, parsers, scope).ok(),
        Token::Number(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Int(*v)))
        }
        Token::Float(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Float(*v)))
        }
        Token::StringLiteral(v) => {
            *pos += 1;
            let v = v.clone();
            Some(Expr::Literal(Value::Str(v)))
        }
        Token::Bool(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Bool(*v)))
        }
        Token::Id(id) => {
            let name = id.clone();
            *pos += 1;

            if eat(&Token::OpenParenthese, tokens, pos) {
                let mut args = Vec::new();
                while !eat(&Token::CloseParenthese, tokens, pos) {
                    if let Some(arg) = parse_arithmetic(tokens, pos, 0, discovered_modules, parsers, scope) {
                        args.push(arg);
                        eat(&Token::Comma, tokens, pos);
                    } else {
                        return None;
                    }
                }
                Some(Expr::FnCall { 
                    name,
                    args
                })
            } else {
                Some(Expr::Id(name))
            }
        }
        Token::OpenParenthese => {
            println!("yay");
            *pos += 1;
            let expr = match parse_arithmetic(tokens, pos, 0, discovered_modules, parsers, scope) {
                Some(e) => e,
                None => {
                    panic!("Failed to parse expression inside parentheses at pos {pos}");
                }
            };

            if !eat(&Token::CloseParenthese, tokens, pos) {
                return None;
            }

            Some(expr)
        }
        // Token::CloseBracket => {
        //     // A standalone block is a valid statement in some languages, or maybe error here
        //     println!("Unexpected `}}` without a control structure");
        //     None
        // }
        _ => {
            // NOTE: this is def correct
            *pos += 1;
            None
        }
    }?;

    if in_condition {
        while let Some(token) = tokens.get(*pos) {
            let op = match token {
                Token::Less => BinOp::Lt,
                Token::LessEqual => BinOp::Le,
                Token::EqualEqual => BinOp::Eq,
                Token::Greater => BinOp::Gt,
                Token::GreaterEqual => BinOp::Ge,
                // Token::Plus => BinOp::Add,
                // Token::Dash => BinOp::Sub,
                // Token::Star => BinOp::Mul,
                // Token::Slash => BinOp::Div,
                _ => break,
            };

            *pos += 1;

            let right_start = *pos;
            let right = parse_arithmetic(tokens, pos, 0, discovered_modules, parsers, scope)?;
            if *pos == right_start {
                return None;
            }

            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right)
            }
        }
    }
    Some(left)
}

pub fn parse_typed_arg(tokens: &[Token], pos: &mut usize) -> Option<TypedArg> {
    todo!()
}

pub fn parse_out_vec_expr(tokens: &[Token], discovered_modules: &mut Vec<String>, parsers: &GroupedParsers, scope: &mut Scope) -> Result<Vec<Expr>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        if let Some(expr) = parse_expr(tokens, &mut pos, false, discovered_modules, parsers, scope) {
            exprs.push(expr);
            if eat(&Token::Comma, tokens, &mut pos) { continue; }
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

    while let Some(token) = tokens.get(pos) {
        match parse_stmt(
            tokens,
            &mut pos,
            discovered_modules, 
            parsers,
            scope
        ) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => panic!("Parse error at token {:?}: {}\n {:?}", token, e, tokens)
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("TOKENS: {:?}", tokens);

        for stmt in &stmts {
            println!("Parsed stmt: {:?}", stmt);
        }
    }
    stmts
}
