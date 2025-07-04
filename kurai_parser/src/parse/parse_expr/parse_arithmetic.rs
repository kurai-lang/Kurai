use colored::Colorize;
use kurai_binop::bin_op::BinOp;
use kurai_ast::expr::Expr;
use kurai_core::scope::Scope;
use kurai_token::token::token::Token;

use crate::{parse::parse::parse_expr, GroupedParsers};

pub fn parse_arithmetic(tokens: &[Token], pos: &mut usize, min_prec: u8, discovered_modules: &mut Vec<String>, parsers: &GroupedParsers, scope: &mut Scope) -> Option<Expr> {
    if let Some(mut left) = parse_expr(tokens, pos, false, discovered_modules, parsers, scope) {
        #[cfg(debug_assertions)]
        { println!("{}: {:?}", "[parse_arithmetic()]".green().bold(), left); }

        loop {
            // Format of op: (Operation, precedence)
            let op = match tokens.get(*pos)? {
                Token::Plus => (BinOp::Add, 1),
                Token::Dash => (BinOp::Sub, 1),
                Token::Star => (BinOp::Mul, 2),
                Token::Slash => (BinOp::Div, 2),
                _ => break,
            };

            if op.1 < min_prec {
                break;
            }

            *pos += 1;

            let right = parse_arithmetic(tokens, pos, op.1+1, discovered_modules, parsers, scope).unwrap();

            left = Expr::Binary { 
                op: op.0, 
                left: Box::new(left), 
                right: Box::new(right),
            };
        }

        Some(left)
    } else { None }
}
