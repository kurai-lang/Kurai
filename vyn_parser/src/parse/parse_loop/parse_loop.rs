use vyn_ast::stmt::Stmt;
use vyn_token::{eat::eat, token::token::Token};

use crate::parse::Parser;

impl Parser {
    pub fn parse_loop(
        &mut self
    ) -> Result<Stmt, String> {
        if !eat(&Token::Loop, &self.tokens, &mut self.pos) {
            return Err("Expected `loop`".to_string());
        }

        // if !eat(&Token::OpenBracket, tokens, pos) {
        //     return Err("Expected `{` after `loop`".to_string());
        // }

        let body = self.parse_block()?; 
        // *pos += 1;

        // if !eat(&Token::CloseBracket, tokens, pos) {
        //     return Err("Expected `}` after body".to_string());
        // }

        Ok(Stmt::Loop { 
            body
        })
    }
}
