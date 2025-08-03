use vyn_core::scope::Scope;
use vyn_ast::expr::{Expr, IfBranch};
use vyn_ast::stmt::Stmt;
use vyn_token::{eat::eat, token::token::Token};

use crate::parse::Parser;

impl Parser {
    pub fn parse_while_loop(
        &mut self,
    ) -> Result<Stmt, String> {
        if !eat(&Token::While, &self.tokens, &mut self.pos) {    
            return Err("Expected `while`".to_string());
        }

        // if !eat(&Token::OpenParenthese, tokens, pos) {
        //     return Err("Expected `(` after `while`".to_string());
        // }

        println!("parsing conditions...");
        let condition = self.parse_expr(true)
            .ok_or_else(|| format!("Failed to parse expression inside `while (...)` at token {}", self.pos))?;
        println!("parsing conditions succeed");

        // if !eat(&Token::CloseParenthese, tokens, pos) {
        //     return Err("Expected `)` after `while` condition".to_string());
        // }

        let body = self.parse_expr_block()?;

        Ok(Stmt::Block(vec![
            Stmt::Loop { 
                body: vec![
                    Stmt::Expr(Expr::If {
                        branches: vec![IfBranch {
                            condition,
                            body,
                        }],
                        else_body: Some(
                                vec![
                                    Expr::Block {
                                        stmts: vec![Stmt::Break],
                                        final_expr: None 
                                    }
                                ]
                        ),
                    })
                ]
            }]
        ))
    }
}
