use kurai_binop::bin_op::BinOp;
use kurai_core::scope::Scope;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;
use crate::{parse::parse::parse_expr, GroupedParsers};

pub fn parse_equal(tokens: &[Token], pos: &mut usize, scope: &mut Scope, parsers: &GroupedParsers) -> Option<Stmt> {
    let mut left = parse_expr(tokens, pos, scope, parsers).unwrap();

    while let Some(op) = parse_comparison_op(tokens, pos) {
        *pos += 1;
        let right = parse_expr(tokens, pos, scope, parsers).unwrap();

        left = Stmt::Binary { 
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
