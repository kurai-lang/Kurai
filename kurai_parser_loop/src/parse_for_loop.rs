use kurai_binop::bin_op::BinOp;
use kurai_core::scope::Scope;
use kurai_expr::expr::Expr;
use kurai_parser::GroupedParsers;
use kurai_stmt::stmt::{IfBranch, Stmt};
use kurai_token::{eat::eat, token::token::Token};
use kurai_types::value::Value;

pub fn parse_for_loop(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::For, tokens, pos) {
        return Err("Expected `for`".to_string());
    }

    if let Some(Token::Id(var_name)) = tokens.get(*pos) {
        *pos += 1;

        if !eat(&Token::In, tokens, pos) {
            return Err(format!("Expected `in` after `{}`", var_name));
        }

        if let Some(Token::Number(starting_num)) = tokens.get(*pos) {
            *pos += 1;

            if !eat(&Token::Range, tokens, pos) {
                return Err("Expected `..` in for loop range".to_string());
            }

            if let Some(Token::Number(ending_num)) = tokens.get(*pos) {
                *pos += 1;

                let body = parsers.block_parser.parse_block(
                    tokens,
                    pos,
                    discovered_modules,
                    parsers,
                    scope,
                )?;

                let id = var_name.to_string();

                return Ok(Stmt::Block(vec![
                    // let i = <starting_num>;
                    Stmt::Let {
                        name: id.clone(),
                        typ: Some("int".to_string()),
                        value: Some(Box::new(Stmt::Literal(Value::Int(*starting_num)))),
                        then: None,
                    },
                    // loop {}
                    Stmt::Loop(
                        // vec![
                        Box::new(
                            Stmt::Block(
                                vec![
                                // if i >= ending_num { break; }
                                Stmt::If {
                                    branches: vec![IfBranch {
                                        condition: Box::new(Stmt::Binary {
                                            op: BinOp::Ge,
                                            left: Box::new(Stmt::Id(id.clone())),
                                            right: Box::new(Stmt::Literal(Value::Int(*ending_num))),
                                        }),
                                        body: Box::new(Stmt::Break),
                                    }],
                                    else_: None,
                                },
                                // body block
                                Stmt::Block(body),
                                // i = i + 1;
                                Stmt::Assign {
                                    target: Box::new(Stmt::Id(id.clone())),
                                    value: Box::new(Stmt::Binary {
                                        op: BinOp::Add,
                                        left: Box::new(Stmt::Id(id.clone())),
                                        right: Box::new(Stmt::Literal(Value::Int(1))),
                                    }),
                                },
                            ]),
                        ),
                            // ],
                    ),
                ]));
            }
        }
    }

    Err("For loop failed".to_string())
}
