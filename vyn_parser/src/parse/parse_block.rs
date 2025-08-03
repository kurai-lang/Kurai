use vyn_core::scope::Scope;
use vyn_ast::expr::Expr;
use vyn_ast::stmt::Stmt;
use vyn_token::token::token::Token;
use vyn_token::eat::eat;
use colored::Colorize;

use crate::parse::Parser;

// pub struct BlockParserStruct;
// impl BlockParser for BlockParserStruct {
//     fn parse_block(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Vec<Stmt>, String> {
//         parse_block(
//             tokens,
//             pos,
//             discovered_modules,
//             parsers,
//             scope,
//             src
//         )
//     }
//
//     fn parse_block_stmt(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         parse_block_stmt(
//             tokens,
//             pos,
//             discovered_modules,
//             parsers,
//             scope,
//             src
//         )
//     }
// }

impl Parser {
    pub fn parse_block(
        &mut self,
    ) -> Result<Vec<Stmt>, String> {
        if !eat(&Token::OpenBracket, &self.tokens, &mut self.pos) {
            return Err(format!("Expected `{{` at start of block, found {:?}", self.tokens.get(self.pos)));
        }

        let mut stmts = Vec::new();
        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(Token::CloseBracket) => {
                    #[cfg(debug_assertions)]
                    println!("{}: closing bracket detected, bumping self.pos", "debug".cyan().bold());

                    self.pos += 1;
                    return Ok(stmts);
                }
                Some(Token::Else) => {
                    // Don't parse 'else' inside a block, let parse_if_else handle it
                    break;
                }
                Some(_) => {
                    #[cfg(debug_assertions)]
                    { println!(">> calling parse_stmt at pos {}: {:?}", self.pos, self.tokens.get(self.pos)); }

                    let stmt = self.parse_stmt()?;

                    stmts.push(stmt);
                }
                None => return Err("Unexpected end of token stream while parsing block.".to_string()),
            }
        }

        // if !eat(&Token::CloseBracket, tokens, pos) {
        //     return Err("Expected `}` at end of block.".to_string());
        // }

        Ok(stmts)
    }

    pub fn parse_expr_block(
        &mut self,
    ) -> Result<Vec<Expr>, String> {
        if !eat(&Token::OpenBracket, &self.tokens, &mut self.pos) {
            return Err(format!("Expected `{{` at start of block, found {:?}", self.tokens.get(self.pos)));
        }

        let mut stmts: Vec<Stmt> = Vec::new();
        let mut final_expr: Option<Box<Expr>> = None;

        while let Some(token) = self.tokens.get(self.pos) {
            match token {
                Token::CloseBracket => {
                    #[cfg(debug_assertions)]
                    println!("{}: found end of block at pos {}, stopping.", "debug".cyan().bold(), self.pos);

                    self.pos += 1;
                    return Ok(vec![Expr::Block {
                        stmts,
                        final_expr,
                    }]);
                }
                Token::Else => {
                    // Don't parse 'else' inside a block, let parse_if_else handle it
                    break;
                }
                Token::Semicolon => {
                    self.pos += 1; // just skip semicolon, it’s not an expression
                    continue;
                }
                _ => {
                    #[cfg(debug_assertions)]
                    { 
                        println!("{}: calling parse_expr_block at pos {}: {:?}", "debug".cyan().bold(), self.pos, self.tokens.get(self.pos)); 
                        println!("{}: parsing statements", "debug".cyan().bold());
                    }
                    let old_pos = self.pos;
                    let stmt = self.parse_stmt().unwrap();
                    match stmt {
                        Stmt::Expr(expr) => {
                            #[cfg(debug_assertions)]
                            println!("{}: parsing final expressions", "debug".cyan().bold());
                            final_expr = Some(Box::new(expr));
                        }
                        _ => stmts.push(stmt)
                    }

                    if self.pos == old_pos {
                        return Err(format!("⚠️ parse_stmt made no progress at pos = {}, token = {:?}", self.pos, self.tokens.get(self.pos)));
                    }
                }
            }
        }

        Ok(vec![Expr::Block {
            stmts, final_expr
        }])
    }

    pub fn parse_block_stmt(
        &mut self,
    ) -> Result<Stmt, String> {
        self.parse_block()
            .map(Stmt::Block)
    }
}
