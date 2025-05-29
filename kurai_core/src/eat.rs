use crate::token::token::Token;

pub fn eat(expected: &Token, tokens: &[Token], pos: &mut usize) -> bool {
    if *pos < tokens.len() && tokens.get(*pos) == Some(expected) {
        *pos += 1;
        true
    } else {
        false
    }
}
