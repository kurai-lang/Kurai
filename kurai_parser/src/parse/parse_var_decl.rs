use kurai_binop::bin_op::BinOp;
use kurai_core::scope::Scope;
use kurai_expr::expr::Expr;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;
use kurai_types::value::Value;
use kurai_stmt::stmt::Stmt;

use crate::parse::parse_expr::parse_arithmetic::parse_arithmetic;
use crate::parse::utils::expr_to_value::expr_to_value;

pub fn parse_var_decl(tokens: &[Token], pos: &mut usize, scope: &mut Scope) -> Result<Stmt, String> {
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
    //     Some(Token::Bool(v)) => {
    //         *pos += 1;
    //         Value::Bool(*v).into()
    //     }
    //     _ => return Err(format!("Unsupported value {:?}", tokens.get(*pos)))
    // };

    let expr = parse_arithmetic(tokens, pos, 0);
    scope.0.insert(name.clone(), expr.clone().unwrap());
    // *pos += 1;

    // No semicolon, no ending
    // no ending, no food
    // no food, ded
    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected a semicolon after expr".to_string());
    }

    // stands for... i forgot
    // oh btw typ does nothing, at least for now 
    Ok(Stmt::VarDecl {
        name,
        typ: "int".to_string(), 
        value: expr,
    })
}
