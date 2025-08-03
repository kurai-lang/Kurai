use vyn_core::scope::Scope;
use vyn_token::token::{spanned_token::SpannedToken, token::Token};

pub mod utils;
pub mod parse;
pub mod instr;
pub mod parse_var_decl;
pub mod parse_var_assign;
pub mod parse_if_else;
pub mod parse_expr;
pub mod parse_block;
pub mod parse_stmt;
pub mod parse_return;
pub mod parse_function;
pub mod parse_loop;
pub mod parse_import;

#[derive(Default, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub spanned_tokens: Vec<SpannedToken>,
    pub pos: usize,

    pub scope: Scope,
    pub src: String,

    pub discovered_modules: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            spanned_tokens: Vec::new(),
            pos: 0,

            scope: Scope::new(),
            src: String::new(),

            discovered_modules: Vec::new(),
        }
    }

    pub fn with_tokens(&self, src: &str) -> Self {
        let (tokens, spanned_tokens) = Token::tokenize(src);
        Self {
            tokens,
            spanned_tokens,
            src: src.to_string(),
            ..Default::default()
        }
    }
}
