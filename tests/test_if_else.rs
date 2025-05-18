use Kurai::{parse::{parse::parse_out_vec_stmt, parse_if_else::{self, parse_if_else}}, token::token::Token};

#[test]
fn test_if_empty() {
    let tokens = Token::tokenize("
    fn main() {
        if (true) {}
    }");
    println!("{:?}", tokens);
    println!("{:?}", parse_out_vec_stmt(&tokens, &mut Vec::new()));
    assert!(parse_if_else(&tokens, &mut 0, &mut Vec::new()).is_ok());
}

#[test]
fn test_if_only_return() {
    let tokens = Token::tokenize("if (false) return;");
    println!("{:?}", tokens);
    println!("{:?}", parse_out_vec_stmt(&tokens, &mut Vec::new()));
    assert!(parse_if_else(&tokens, &mut 0, &mut Vec::new()).is_ok());
}
