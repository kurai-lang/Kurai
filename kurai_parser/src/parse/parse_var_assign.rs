use colored::Colorize;
use kurai_core::scope::Scope;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use kurai_ast::stmt::Stmt;

use crate::parse::parse_expr::parse_arithmetic::parse_arithmetic;
use crate::GroupedParsers;

pub fn parse_var_assign(
    tokens: &[Token], 
    pos: &mut usize, 
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers, 
    scope: &mut Scope,
    src: &str,
) -> Result<Stmt, String> {
    if let Some(Token::Id(id)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::Equal, tokens, pos) {
            return Err(format!("Expected an equal sign `=` after `{}`", id));
        }

        // let value: Option<Value> = match tokens.get(*pos) {
        //     Some(Token::Number(v)) => {
        //         *pos += 1;
        //         // Lets use .into() cuz im too lazy to use Some()
        //         Value::Int(*v).into()
        //     }
        //     Some(Token::Float(v)) => {
        //         *pos += 1;
        //         Value::Float(*v as f64).into()
        //     }
        //     Some(Token::Id(id)) => {
        //         *pos += 1;
        //         Value::Str(id.clone()).into()
        //     }
        //     _ => return Err(format!("Unsupported value: {:?}", tokens.get(*pos)))
        // };

        let expr = parse_arithmetic(tokens, pos, 0, discovered_modules, parsers, scope, src);
        let value = &expr.unwrap();

        #[cfg(debug_assertions)]
        { println!("{} name = {}, value = {:?}", "[parse_var_assign()]".green().bold(), id.clone(), value); }

        if !eat(&Token::Semicolon, tokens, pos) {
            return Err(format!("Expected a semicolon `;` after `{:?}`", value));
        }

        Ok(Stmt::Assign {
            name: id.to_string(),
            value: value.clone(),
        })
    } else {
        return Err("Where identifier".to_string());
    }
}
