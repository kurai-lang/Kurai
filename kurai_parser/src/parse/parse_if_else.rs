use colored::Colorize;
use kurai_ast::expr::{Expr, IfBranch};
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use crate::parse::Parser;

impl Parser {
    pub fn parse_if_else(
        &mut self,
    ) -> Result<Expr, String> {
        #[cfg(debug_assertions)]
        println!("{}: parsing if/else expression", "debug".cyan().bold());

        if !eat(&Token::If, &self.tokens, &mut self.pos) {
            return Err("Expected keyword `if`".to_string());
        }
        // check if ( exists; if it does, parse inside it; otherwise, parse the next expression directly.

        let condition = if eat(&Token::OpenParenthese, &self.tokens, &mut self.pos) {
            #[cfg(debug_assertions)]
            println!("{}: parsing conditions", "debug".cyan().bold());

            let cond = self.parse_expr(true)
                .ok_or_else(|| panic!("Failed to parse expression inside `if (...)` at token {}", self.pos)).unwrap();

            if !eat(&Token::CloseParenthese, &self.tokens, &mut self.pos) {
                return Err("Expected `)` after `if` condition".to_string());
            }

            cond
        } else {
            #[cfg(debug_assertions)]
            println!("{}: parsing conditions", "debug".cyan().bold());
            self.parse_expr(true)
                .ok_or_else(|| panic!("Failed to parse expression after `if` at token {}", self.pos)).unwrap()
        };

        #[cfg(debug_assertions)]
        println!("{}: parsing conditions successful. condition: {:?}", "debug".cyan().bold(), condition);


        // if !eat(&Token::OpenBracket, tokens, pos) {
        //     return Err("Expected `{` at start of block".to_string());
        // }

        #[cfg(debug_assertions)]
        println!("{}: parsing then branch", "debug".cyan().bold());
        let then_branch = self.parse_expr_block()?;
        #[cfg(debug_assertions)]
        println!("{}: parsing then branch successful. then_branch: {:?}", "debug".cyan().bold(), then_branch);

        // if !eat(&Token::CloseBracket, tokens, pos) {
        //     return Err("Expected `}` at start of block".to_string());
        // }

        // if !eat(&Token::OpenBracket, tokens, pos) {
        //     return Err("Expected `{` at start of block".to_string());
        // }

        #[cfg(debug_assertions)]
        println!("{}: parsing else body", "debug".cyan().bold());
        let else_body = if eat(&Token::Else, &self.tokens, &mut self.pos) {
            Some(self.parse_expr_block()?)
        } else {
            None
        };
        #[cfg(debug_assertions)]
        println!("{}: parsing else body successful. else_body: {:?}", "debug".cyan().bold(), else_body);

        // if !eat(&Token::CloseBracket, tokens, pos) {
        //     return Err("Expected `}` at start of block".to_string());
        // }

        println!("{}: parsing whole if/else expression successful. Returning the expression for codegen to handle", "debug".cyan().bold());
        Ok(Expr::If {
            branches: vec![IfBranch {
                condition,
                body: then_branch,
            }],
            else_body,
        })
    }
}
