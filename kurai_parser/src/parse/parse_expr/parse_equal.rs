use kurai_binop::bin_op::BinOp;
use kurai_core::scope::Scope;
use kurai_token::token::token::Token;
use kurai_ast::expr::Expr;
use crate::{parse::parse::parse_expr, GroupedParsers};

pub fn parse_equal(
    tokens: &[Token], 
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers, 
    scope: &mut Scope,
    src: &str,
) -> Option<Expr> {
    let mut left = parse_expr(tokens, pos, true, discovered_modules, parsers, scope, src)?;

    while let Some(op) = parse_comparison_op(tokens, pos) {
        *pos += 1;
        let right = parse_expr(tokens, pos, true, discovered_modules, parsers, scope, src)?;

        left = Expr::Binary { 
            op,
            left: Box::new(left),
            right: Box::new(right)
        }
    }

    Some(left)
}

fn parse_comparison_op(tokens: &[Token], pos: &mut usize) -> Option<BinOp> {
    let op = match tokens.get(*pos)? {
        Token::Less => BinOp::Lt,
        Token::LessEqual => BinOp::Le,
        Token::Greater => BinOp::Gt,
        Token::GreaterEqual => BinOp::Ge,
        Token::EqualEqual => BinOp::Eq,
        Token::BangEqual => BinOp::Ne,
        _ => panic!("Dude-")
    };

    Some(op)
}
