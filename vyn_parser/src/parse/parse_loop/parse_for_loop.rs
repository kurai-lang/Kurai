use vyn_ast::{expr::{Expr, IfBranch}, stmt::Stmt};
use vyn_binop::bin_op::BinOp;
use vyn_token::{eat::eat, token::token::Token};
use vyn_types::value::Value;

use crate::parse::Parser;

impl Parser {
    pub fn parse_for_loop(
        &mut self,
    ) -> Result<Stmt, String> {
        if !eat(&Token::For, &self.tokens, &mut self.pos) {
            return Err("Expected `for`".to_string());
        }

        let id = match self.tokens.get(self.pos) {
            Some(Token::Id(id)) => id.clone(),
            _ => return Err("Where identifier".to_string())
        };
        self.pos += 1;

        if !eat(&Token::In, &self.tokens, &mut self.pos) {
            return Err(format!("Expected `in` after `{id}`"));
        }

        let starting_num  = match self.tokens.get(self.pos) {
            Some(Token::Number(v)) => *v,
            _ => return Err("Where starting number".to_string())
        };
        self.pos += 1;

        if !eat(&Token::Range, &self.tokens, &mut self.pos) {
            return Err("Expected `..` in for loop range".to_string());
        }

        let ending_num = match self.tokens.get(self.pos) {
            Some(Token::Number(v)) => *v,
            _ => return Err("Where ending number".to_string())
        };
        self.pos += 1;

        let body = self.parse_block()?;

        return Ok(Stmt::Block(vec![
            // let i = <starting_num>;
            Stmt::VarDecl {
                name: id.clone(),
                typ: Some("i128".to_string()),
                value: Some(Expr::Literal(Value::Int(starting_num))),
            },
            // loop {}
            Stmt::Loop {
                body: vec![
                    // if i >= ending_num { break; }
                    Stmt::Expr(Expr::If {
                        branches: vec![IfBranch {
                            condition: Expr::Binary {
                                op: BinOp::Ge,
                                left: Box::new(Expr::Id(id.clone())),
                                right: Box::new(Expr::Literal(Value::Int(ending_num))),
                            },
                            body: vec![Expr::Block {
                                stmts: vec![Stmt::Break],
                                final_expr: None,
                            }],
                        }],
                        else_body: None,
                    }),
                    // body block
                    Stmt::Block(body),
                    // i = i + 1;
                    Stmt::Assign {
                        name: id.clone(),
                        value: Expr::Binary {
                            op: BinOp::Add,
                            left: Box::new(Expr::Id(id.clone())),
                            right: Box::new(Expr::Literal(Value::Int(1))),
                        },
                    },
                ],
            },
        ]));
    }
}
