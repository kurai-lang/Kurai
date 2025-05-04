use Kurai::{parse::{parse::parse, stmt::Stmt}, scope::Scope, token::token::Token, value::Value};
// use std::io::{self, Write};

fn main() {
    let mut scope = Scope::new();
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
