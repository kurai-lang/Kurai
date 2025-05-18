use crate::{eat::eat, token::token::Token, value::Value};

use super::{bin_op::BinOp, expr::Expr, parse::{self, parse_expr, parse_stmt}, parse_block::parse_block, stmt::{IfBranch, Stmt}};

pub fn parse_if_else(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    if !eat(&Token::If, tokens, pos) {
        return Err("Expected keyword `if`".to_string());
    }

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return Err("Expected an opening parenthesis `(` after keyword `if`".to_string());
    }
    let condition = parse_expr(tokens, pos).unwrap();
    println!("{:?}", condition);
    *pos += 2;
    if !eat(&Token::CloseParenthese, tokens, pos) {
        return Err("Expected a closing paranthesis `)` after condition".to_string());
    }

    let then_branch = parse_block(tokens, pos, discovered_modules);

    let else_branch = if eat(&Token::Else, tokens, pos) {
        Ok(parse_block(tokens, pos, discovered_modules).unwrap())
    } else {
        Err("puru".to_string())
    };
        // IfBranch {
        //     condition: Expr::Binary { 
        //         op: BinOp::Eq,
        //         left: Box::new(Expr::Literal(Value::Int(5))),
        //         right: Box::new(Expr::Literal(Value::Int(5))),
        //     }, // Wait shit, Expr is not that developed yet
        //     body,
        // }
    Ok(Stmt::If {
        branches: vec![IfBranch {
            condition,
            body: then_branch?,
        }],
        else_body: else_branch,
    })
}
