use colored::Colorize;
use vyn_binop::bin_op::BinOp;
use vyn_ast::expr::Expr;
use vyn_core::scope::Scope;
use vyn_token::token::token::Token;

use crate::parse::Parser;

impl Parser {
    pub fn parse_arithmetic(
        &mut self,
        min_prec: u8, 
    ) -> Option<Expr> {
        if let Some(mut left) = self.parse_expr(true) {
            #[cfg(debug_assertions)]
            { println!("{}: {:?}", "[parse_arithmetic()]".green().bold(), left); }

            loop {
                // Format of op: (Operation, precedence)
                let op = match self.tokens.get(self.pos)? {
                    Token::Plus => (BinOp::Add, 1),
                    Token::Dash => (BinOp::Sub, 1),
                    Token::Star => (BinOp::Mul, 2),
                    Token::Slash => (BinOp::Div, 2),
                    _ => break,
                };

                if op.1 < min_prec {
                    break;
                }

                self.pos += 1;

                let right = self.parse_arithmetic(op.1+1).unwrap();

                left = Expr::Binary { 
                    op: op.0, 
                    left: Box::new(left), 
                    right: Box::new(right),
                };
            }

            Some(left)
        } else { None }
    }
}
