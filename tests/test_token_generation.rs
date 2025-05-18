use Kurai::token::token::Token;

#[test]
fn test_token_generation() {
    let tokens = Token::tokenize("use std::io;");
    assert_eq!(tokens, vec![
        Token::Import,
        Token::Id("std".into()),
        Token::Colon,
        Token::Colon,
        Token::Id("io".into()),
        Token::Semicolon
    ]);
}
