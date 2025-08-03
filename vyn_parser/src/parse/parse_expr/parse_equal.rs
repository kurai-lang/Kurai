use vyn_binop::bin_op::BinOp;
use vyn_core::scope::Scope;
use vyn_token::token::token::Token;
use vyn_ast::expr::Expr;

use crate::parse::Parser;

impl Parser {
    pub fn parse_equal(
        &mut self,
    ) -> Option<Expr> {
        let mut left = self.parse_expr(true)?;

        while let Some(op) = self.parse_comparison_op() {
            self.pos += 1;
            let right = self.parse_expr(true)?;

            left = Expr::Binary { 
                op,
                left: Box::new(left),
                right: Box::new(right)
            }
        }

        Some(left)
    }

    fn parse_comparison_op(&self) -> Option<BinOp> {
        let op = match self.tokens.get(self.pos)? {
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
}
