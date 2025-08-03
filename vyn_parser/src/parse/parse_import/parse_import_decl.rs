use vyn_token::eat::eat;
use vyn_ast::stmt::Stmt;
use vyn_token::token::token::Token;

use crate::parse::Parser;

impl Parser {
    pub fn parse_import_decl(
        &mut self
    ) -> Result<Stmt, String> {
        if !eat(&Token::Import, &self.tokens, &mut self.pos) {
            return Err("Expected keyword `use` or `gunakan`".to_string());
        }

        let mut path = Vec::new();

        let mut is_glob = false;

        loop {
            match self.tokens.get(self.pos) {
                Some(Token::Id(name)) => {
                    path.push(name.clone());
                    self.discovered_modules.push(name.clone());
                    self.pos += 1;
                }
                _ => break 
            };

            // Detects `::`? then continue scanning and pushing to `path` vector variable
            if eat(&Token::Colon, &self.tokens, &mut self.pos) && eat(&Token::Colon, &self.tokens, &mut self.pos) {
                if eat(&Token::Star, &self.tokens, &mut self.pos) {
                    is_glob = true;
                    break;
                }
                // if let Some(Token::Id(name)) = tokens.get(*pos) {
                //     path.push(name.clone());
                //     *pos += 1;
                // } else {
                //     panic!("Expected identifier after `::`");
                // }
                match self.tokens.get(self.pos) {
                    Some(Token::Id(name)) => {
                        path.push(name.clone());
                        self.pos += 1;
                    }
                    _ => return Err("Expected identifier after `::`".to_string()),
                }
            } else {
                break;
            }
        }

        let nickname = if eat(&Token::As, &self.tokens, &mut self.pos) {
            match self.tokens.get(self.pos) {
                Some(Token::Id(name)) => {
                    self.pos += 1;
                    Some(name.clone())
                },
                _ => return Err("Expected identifier after `as`".to_string()),
            }
        } else {
            None
        };

        if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
            return Err("Expected semicolon after giving the package a nickname".to_string());
        }

        Ok(Stmt::Import {
            path,
            nickname,
            is_glob,
        })
    }
}
