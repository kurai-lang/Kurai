use colored::Colorize;
use vyn_attr::attribute::Attribute;
use vyn_core::scope::Scope;
use vyn_token::eat::eat;
use vyn_token::token::token::Token;
use vyn_ast::stmt::Stmt;
use vyn_ast::typedArg::TypedArg;
use vyn_types::typ::Type;

use crate::parse::Parser;

impl Parser {
    pub fn parse_fn_decl(
        &mut self,
        attrs: Vec<Attribute>,
    ) -> Result<Stmt, String> {
        let mut is_extern = false;

        if eat(&Token::Extern, &self.tokens, &mut self.pos) {
            is_extern = true;
        }

        if !eat(&Token::Function, &self.tokens, &mut self.pos) {
            return Err("Expected keyword `fn`".to_string());
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Id(name)) => {
                self.pos += 1;
                name.clone()
            }
            _ => return Err("Expected an identifier name after keyword `fn`".to_string())
        };

        if !eat(&Token::OpenParenthese, &self.tokens, &mut self.pos) {
            return Err(format!("Expected an opening paranthese `(` after `{}`", name));
        }

        // NOTE: arguments parsing time
        let mut args = Vec::new();
        if let Some(&Token::CloseParenthese) = self.tokens.get(self.pos) {
            self.pos += 1;
        } else {
            while let Some(token) = self.tokens.get(self.pos) {
                match token {
                    Token::CloseParenthese => {
                        self.pos += 1;
                        break;
                    }
                    // Some(Token::If) => {
                    //     let expr = parse_if_else(tokens, pos, discovered_modules, parsers, scope)?;
                    //     return Ok(Stmt::Expr(expr)); // wrap it in a Stmt!
                    // }
                    Token::Id(arg_name) => {
                        let name = arg_name.clone();
                        self.pos += 1;

                        if !eat(&Token::Colon, &self.tokens, &mut self.pos) {
                            return Err(format!("Expected `:` after argument name {}", name));
                        }

                        let typ = match self.tokens.get(self.pos) {
                            // Some(Token::Id(type_name)) => {
                            //     *pos += 1;
                            //     type_name.clone()
                            // }
                            Some(Token::Type(typ)) => {
                                self.pos += 1;
                                typ.clone()
                            }
                            _ => return Err(format!("Expected a type name after `:` in argument {}", name))
                        };

                        // name: type
                        args.push(TypedArg {
                            name,
                            typ,
                            value: None,
                        });

                        if let Some(Token::Comma) = self.tokens.get(self.pos) {
                            self.pos += 1;
                        }
                    }
                    _ => return Err("Invalid argument syntax inside function declaration".to_string())
                    // _ => None
                }
            }
        }

        let mut star_count = 0;
        while let Some(Token::Star) = self.tokens.get(self.pos) {
            self.pos += 1;
            star_count += 1;
            // NOTE: not yet bro
            // ret_type = Type::Ptr(Box::new((ret_type)))
        }
        #[cfg(debug_assertions)]
        println!("{}: parsing return type", "debug".cyan().bold());
        let mut ret_type = match self.tokens.get(self.pos) {
            Some(Token::Type(typ)) => {
                self.pos += 1;
                typ.clone()
            },
            _ => Type::Void
        };

        for _ in 0..star_count {
            ret_type = Type::Ptr(Box::new(ret_type));
        }
        #[cfg(debug_assertions)]
        println!("{}: return type of function {} is {:?}", "debug".cyan().bold(), name, ret_type);

        let mut body = Vec::new();

        if is_extern {
            if !eat(&Token::Semicolon, &self.tokens, &mut self.pos) {
                return Err("Expected `;` after extern function declaration".to_string());
            }
        } else if eat(&Token::OpenBracket, &self.tokens, &mut self.pos) {
            while self.pos < self.tokens.len() {
                while let Some(token) = self.tokens.get(self.pos) {
                    match token {
                        Token::CloseBracket => {
                            #[cfg(debug_assertions)]
                            println!("{}: end of function block", "debug".cyan().bold());
                            self.pos += 1;

                            break;
                        }
                        _ => {
                        let old_pos = self.pos;
                        match self.parse_stmt() {
                            Ok(stmt) => {
                                if self.pos == old_pos {
                                    return Err(format!(
                                        "{}: parse_stmt() made no progress at pos {}: {:?}", "debug".cyan().bold(),
                                        self.pos,
                                        self.tokens.get(self.pos),
                                    ));
                                }
                                body.push(stmt);
                            }
                            Err(e) => {
                                println!(
                                    "{}: statement failed at pos {} → token: {:?}",
                                    "debug".cyan().bold(),
                                    self.pos,
                                    self.tokens.get(self.pos)
                                );

                                return Err(format!(
                                    "Error parsing statement in function `{}` at pos {} (token: {:?}):\n{}",
                                    name,
                                    self.pos,
                                    self.tokens.get(self.pos),
                                    e
                                ));
                            }
                        }
                    }
                    }
                }
            }
        } else {
            return Err("Expected an opening bracket".to_string());
        }
        Ok(Stmt::FnDecl { 
            name,
            args,
            body,
            attributes: attrs,
            ret_type,
            is_extern
        })
    }
}
