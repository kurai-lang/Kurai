use colored::Colorize;
use vyn_error::error::{report_error, Error};
use vyn_error::error_kind::{ErrorKind, ParseErrorKind};
use vyn_error::span::Span;
use vyn_token::token::token::Token;
use vyn_token::eat::eat;
use vyn_ast::stmt::Stmt;

use crate::parse::Parser;

impl Parser {
    pub fn parse_var_decl(
        &mut self,
    ) -> Result<Stmt, String> {
        if !eat(&Token::Let, &self.tokens, &mut self.pos) {
            return Err("Expected keyword `let`".to_string());
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Id(name)) => {
                self.pos += 1;
                name.clone()
            }
            _ => return Err("Expected an identifier name after keyword `let`".to_string()),
        };

        if !eat(&Token::Equal, &self.tokens, &mut self.pos) {
            // return Err(format!("Expected an equal sign after `{}`", name));
            let span = Span::new("dummy.kurai").with_range(2).with_width(3).with_line_column(15, 5);
            let src_line = self.src.lines().nth(span.line - 1).unwrap_or("");
            let err = Error::new(ErrorKind::Parse {
                kind: ParseErrorKind::ExpectedToken(format!("Expected an equal sign `=` after `{name}`")),
                span
            });
            report_error(&err, src_line);
        }

        #[cfg(debug_assertions)]
        println!("{}: parsing expressions", "debug".cyan().bold());
        let expr = self.parse_arithmetic(0).unwrap();
        #[cfg(debug_assertions)]
        println!("{}: parsing expressions successful", "debug".cyan().bold());
        // let expr = parse_expr(tokens, pos, true, discovered_modules, parsers, scope);
        self.scope.0.insert(name.clone(), expr.clone());
        // *pos += 1;

        // No semicolon, no ending
        // no ending, no food
        // no food, ded
        #[cfg(debug_assertions)]
        println!("{}: current token is {:?}", "debug".cyan().bold(), self.tokens.get(self.pos));
        if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
            return Err("Expected a semicolon after expr".to_string());
        }

        // typ is automatically inferred in codegen process lol
        Ok(Stmt::VarDecl {
            name,
            typ: None,
            value: Some(expr),
        })
    }
}
