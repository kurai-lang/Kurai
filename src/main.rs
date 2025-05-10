use Kurai::{parse::parse::parse, scope::Scope, token::token::Token};
// use std::io::{self, Write};

fn main() {
    let scope = Scope::new();
    let code = "printf(\"yo\")";
    let tokens = Token::tokenize(code);
    let parsed_stmt = parse(&tokens);
 
    println!("{:?}", parsed_stmt);
    // if let Some(stmt) = parsed_stmt {
    //     stmt.execute(&mut scope);
    // }

    println!("{:?}", scope);
    println!("{:?}", tokens);
}
