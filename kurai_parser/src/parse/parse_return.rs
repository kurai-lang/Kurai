use kurai_ast::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};

use crate::parse::Parser;

impl Parser {
    pub fn parse_return(
        &mut self,
    ) -> Result<Stmt, String> {
        if !eat(&Token::Return, &self.tokens, &mut self.pos) {
            return Err("Expected `return` keyword".to_string());
        }

        let expr = self.parse_expr(true);

        if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
            return Err("Expected a semicolon".to_string());
        }

        // NOTE: well... that was easier than i expected

        Ok(Stmt::Return(expr))
    }
}
