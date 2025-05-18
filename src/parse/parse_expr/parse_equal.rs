use crate::{parse::{bin_op::BinOp, expr::Expr, parse::parse_expr}, token::token::Token};

pub fn parse_equal(tokens: &[Token], pos: &mut usize) -> Option<Expr> {
    let mut left = parse_expr(tokens, pos)?;

    while let Some(op) = parse_comparison_op(tokens, pos) {
        *pos += 1;

        let right = parse_expr(tokens, pos)?;
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
