use crate::{parse::{bin_op::BinOp, expr::Expr, parse::parse_expr}, token::token::Token};

pub fn parse_equal(tokens: &[Token], pos: &mut usize) -> Option<Expr> {
    let mut left = parse_expr(tokens, pos)?;

    loop {
        let op = if let Some(op) = parse_equality(tokens, pos) {
            op
        } else if let Some(op) = parse_comparison(tokens, pos) {
            op
        } else {
            break;
        };
        *pos += 1;

        let right = parse_expr(tokens, pos)?;

        left = Expr::Binary { 
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Some(left)
}

fn parse_comparison(tokens: &[Token], pos: &mut usize) -> Option<BinOp> {
    let op = match tokens.get(*pos)? {
        Token::Less => BinOp::Lt,
        Token::LessEqual => BinOp::Le,
        Token::Greater => BinOp::Gt,
        Token::GreaterEqual => BinOp::Ge,
        _ => panic!("Dude-")
    };

    Some(op)
}

fn parse_equality(tokens: &[Token], pos: &mut usize) -> Option<BinOp> {
    let op = match tokens.get(*pos)? {
        Token::EqualEqual => BinOp::Eq,
        Token::BangEqual => BinOp::Ne,
        _ => panic!("Istg-")
    };

    Some(op)
}
