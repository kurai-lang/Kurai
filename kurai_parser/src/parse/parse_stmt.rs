use colored::Colorize;
use kurai_ast::expr::Expr;
use kurai_ast::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};
use kurai_ast::typedArg::TypedArg;
use kurai_types::typ::Type;

use crate::parse::Parser;

// pub struct StmtParserStruct;
// impl StmtParser for StmtParserStruct {
//     fn parse_stmt(
//         &self,
//         tokens: &[Token],
//         pos: &mut usize,
//         discovered_modules: &mut Vec<String>,
//         
//         scope: &mut Scope,
//         src: &str,
//     ) -> Result<Stmt, String> {
//         // Parser::parse_stmt(tokens, pos, discovered_modules, parsers, scope, src)
//     }
// }

impl Parser {
    pub fn parse_stmt(
        &mut self,
    ) -> Result<Stmt, String> {
        // println!("[parse_stmt] Entering at pos = {}, token = {:?}", *pos, tokens.get(*pos));
        println!("{}: At parse_stmt entry: pos = {}, len = {}", "sanity check".cyan().bold(), self.pos, self.tokens.len());

        let mut attrs = if let Some(Token::Hash) = self.tokens.get(self.pos) {
            self.parse_attrs()?
        } else {
            Vec::new()
        };

        if self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                // NOTE: OLD STATEMENT FUNCTIONS
                // Some(Token::If) => parse_if_else(tokens, pos, discovered_modules, parsers, scope),

                // NOTE: NEW ONES
                Some(Token::Function) | Some(Token::Extern) => {
                    let attrs_temp = attrs.clone();
                    attrs = Vec::new();
                    self.parse_fn_decl(attrs_temp)
                }
                Some(Token::Loop) => self.parse_for_loop(),
                Some(Token::While) => self.parse_while_loop(),
                Some(Token::Break) => {
                    self.pos += 1;
                    if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
                        return Err("Expected ';' after `break`".to_string());
                    }

                    self.pos += 1;
                    Ok(Stmt::Break)
                }
                Some(Token::Return) => self.parse_return(),
                Some(Token::Let) => self.parse_var_decl(),
                Some(Token::Import) => self.parse_import_decl(),
                Some(Token::For) => self.parse_for_loop(),
                Some(Token::Id(_)) => {
                    match self.tokens.get(self.pos + 1) {
                        Some(Token::Equal) => self.parse_var_assign(),
                        Some(Token::OpenParenthese) => {
                            let expr = self.parse_fn_call()?;
                            Ok(Stmt::Expr(expr))
                        }
                        _ => Err("Unexpected token after identifier. Expected `=` or `(`.".into())
                    }
                }
                Some(Token::OpenBracket) => {
                    #[cfg(debug_assertions)]
                    { println!("{}: encountered an opening bracket `{{`", "debug".cyan().bold()); }
                    let stmts = self.parse_block()?;
                    Ok(Stmt::Block(stmts))
                }
                _ => {
                    let start_pos = self.pos;
                    match self.parse_arithmetic(0) {
                        Some(Expr::FnCall { name, args }) if self.pos > start_pos => {
                            let typed_args = args
                                .into_iter()
                                .map(|arg| TypedArg {
                                    name: name.clone(),
                                    typ: Type::Unknown,
                                    value: Some(arg),
                                })
                                .collect();

                            Ok(Stmt::FnCall { name, args: typed_args })
                        }
                        Some(expr) if self.pos > start_pos => Ok(Stmt::Expr(expr)),
                        _ => Err(format!(
                            "Invalid statement or no progress at pos {}: {:?}",
                            self.pos,
                            self.tokens.get(self.pos)
                        )),
                    }
                }
            }
        } else {
            println!("Unexpected end of input while parsing statement.");
            Err("Unexpected end of input while parsing statement.".to_string())
        }
    }
}
