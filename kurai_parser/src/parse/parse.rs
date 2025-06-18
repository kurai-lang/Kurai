use kurai_core::scope::Scope;
// use kurai_codegen::codegen::codegen::CodeGen;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::value::Value;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;
use kurai_binop::bin_op::BinOp;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;

use crate::parse::parse_expr::parse_arithmetic::parse_arithmetic;
use crate::parse::parse_stmt::parse_stmt;
use crate::{BlockParser, FunctionParser, ImportParser, LoopParser};

pub fn parse_expr(tokens: &[Token], pos: &mut usize, in_condition: bool) -> Option<Expr> {
    // parse_equal(tokens, pos)
    let mut left = match tokens.get(*pos)? {
        Token::Number(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Int(*v)))
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
                    if let Some(arg) = parse_arithmetic(tokens, pos, 0) {
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
                Some(Expr::Var(name))
            }
        }
        Token::OpenParenthese => {
            println!("yay");
            *pos += 1;
            let expr = match parse_arithmetic(tokens, pos, 0) {
                Some(e) => e,
                None => {
                    panic!("Failed to parse expression inside parentheses at pos {pos}");
                }
            };
            eat(&Token::CloseParenthese, tokens, pos);
            Some(expr)
        }
        // Token::OpenBracket => {
        //     // A standalone block is a valid statement in some languages, or maybe error here
        //     println!("Unexpected `{{` without a control structure");
        //     None
        // }
        _ => {
            *pos += 1;
            for i in (*pos).saturating_sub(3)..(*pos+4).min(tokens.len()) {
                println!(
                    "token[{}] = {:?}{}",
                    i,
                    tokens.get(i),
                    if i == *pos { "   <== current" } else { "" }
                );
            }
            // println!("{:?}", tokens.get(*pos));
            None
        }
    }?;

    // while let Some(Token::EqualEqual) = tokens.get(*pos) {
    //     *pos += 1;
    //     let right = parse_expr(tokens, pos, false).unwrap();
    //
    //     left = Expr::Binary { 
    //         op: BinOp::Eq,
    //         left: Box::new(left),
    //         right: Box::new(right)
    //     };
    // }
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
            let right = parse_arithmetic(tokens, pos, 0)?;
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

pub fn parse_out_vec_expr(tokens: &[Token]) -> Result<Vec<Expr>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        if let Some(expr) = parse_expr(tokens, &mut pos, false) {
            exprs.push(expr);
            if eat(&Token::Comma, tokens, &mut pos) { continue; }
        }
    }

    Ok(exprs)
}

pub fn parse_out_vec_stmt(
    tokens: &[Token],
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser, fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        match parse_stmt(tokens, &mut pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => panic!("Parse error at token {:?}: {}\n {:?}", tokens.get(pos), e, tokens)
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("TOKENS: {:?}", tokens);
    }
    stmts
}
