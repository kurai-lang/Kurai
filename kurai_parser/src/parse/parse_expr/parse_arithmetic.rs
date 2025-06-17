use kurai_binop::bin_op::BinOp;
use kurai_expr::expr::Expr;
use kurai_token::token::token::Token;

use crate::parse::parse::parse_expr;

pub fn parse_arithmetic(tokens: &[Token], pos: &mut usize, min_prec: u8) -> Option<Expr> {
    let mut left = parse_expr(tokens, pos, false).unwrap();

    println!("{:?}", left);

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

        let mut right = parse_arithmetic(tokens, pos, op.1+1).unwrap();

        left = Expr::Binary { 
            op: op.0, 
            left: Box::new(left), 
            right: Box::new(right),
        };
    }

    Some(left)
}
