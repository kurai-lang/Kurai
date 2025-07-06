use colored::Colorize;
use kurai_core::scope::Scope;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;
use kurai_ast::stmt::Stmt;

use crate::parse::parse_expr::parse_arithmetic::parse_arithmetic;
use crate::GroupedParsers;

pub fn parse_var_decl(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>, parsers: &GroupedParsers, scope: &mut Scope) -> Result<Stmt, String> {
    if !eat(&Token::Let, tokens, pos) {
        return Err("Expected keyword `let`".to_string());
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return Err("Expected an identifier name after keyword `let`".to_string()),
    };

    if !eat(&Token::Equal, tokens, pos) {
        return Err(format!("Expected an equal sign after `{}`", name));
    }

    #[cfg(debug_assertions)]
    println!("{}: parsing expressions", "debug".cyan().bold());
    let expr = parse_arithmetic(tokens, pos, 0, discovered_modules, parsers, scope);
    #[cfg(debug_assertions)]
    println!("{}: parsing expressions successful", "debug".cyan().bold());
    // let expr = parse_expr(tokens, pos, true, discovered_modules, parsers, scope);
    scope.0.insert(name.clone(), expr.clone().unwrap());
    // *pos += 1;

    // No semicolon, no ending
    // no ending, no food
    // no food, ded
    #[cfg(debug_assertions)]
    println!("{}: current token is {:?}", "debug".cyan().bold(), tokens.get(*pos));
    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected a semicolon after expr".to_string());
    }

    // stands for... i forgot
    // oh btw typ does nothing, at least for now 
    Ok(Stmt::VarDecl {
        name,
        typ: None, 
        value: expr,
    })
}
