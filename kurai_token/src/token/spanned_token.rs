use crate::token::token::Token;

#[derive(Debug)]
pub struct SpannedToken {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub width: usize,
}
