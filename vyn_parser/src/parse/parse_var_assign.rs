use colored::Colorize;
use vyn_token::eat::eat;
use vyn_token::token::token::Token;
use vyn_ast::stmt::Stmt;

use crate::parse::Parser;

impl Parser {
    pub fn parse_var_assign(
        &mut self,
    ) -> Result<Stmt, String> {
        let id = match self.tokens.get(self.pos) {
            Some(Token::Id(id)) => id.clone(),
            _ => return Err("Where identifier".to_string()),
        };
        self.pos += 1;

        if !eat(&Token::Equal, &self.tokens, &mut self.pos) {
            return Err(format!("Expected an equal sign `=` after `{id}`"));
        }

        let expr = self.parse_arithmetic(0);
        let value = &expr.unwrap();

        #[cfg(debug_assertions)]
        { println!("{} name = {}, value = {:?}", "[parse_var_assign()]".green().bold(), id.clone(), value); }

        if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
            return Err(format!("Expected a semicolon `;` after `{value:?}`"));
        }

        Ok(Stmt::Assign {
            name: id.to_string(),
            value: value.clone(),
        })
    }
}
