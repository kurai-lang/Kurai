use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use crate::{BlockParser, FunctionParser, ImportParser, LoopParser};

use super::parse::parse_expr;
use super::parse_block::parse_block;
use kurai_stmt::stmt::{ IfBranch, Stmt };

pub fn parse_if_else(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    block_parser: &dyn BlockParser,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
    loop_parser: &dyn LoopParser
) -> Result<Stmt, String> {
    if !eat(&Token::If, tokens, pos) {
        return Err("Expected keyword `if`".to_string());
    }

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return Err("Expected an opening parenthesis `(` after keyword `if`".to_string());
    }
    let condition = parse_expr(tokens, pos, true).unwrap();
    println!("{:?}", condition);

    if !eat(&Token::CloseParenthese, tokens, pos) {
        return Err("Expected a closing paranthesis `)` after condition".to_string());
    }

    let then_branch = block_parser.parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser);

    let else_branch = if eat(&Token::Else, tokens, pos) {
        Some(parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser).unwrap())
    } else {
        None 
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
