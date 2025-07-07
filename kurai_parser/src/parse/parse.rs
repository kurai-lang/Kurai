use kurai_types::value::Value;
use kurai_ast::stmt::Stmt;
use kurai_ast::expr::Expr;
use kurai_binop::bin_op::BinOp;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;

use crate::parse::Parser;

impl Parser {
    pub fn parse_expr(
        &mut self,
        in_condition: bool,
    ) -> Option<Expr> {
        // parse_equal(tokens, pos)
        // match tokens.get(*pos) {
        //     Some(Token::If) => parse_if_else(tokens, pos, discovered_modules, parsers, scope).ok(),
        //     _ => None
        // };
        let mut left = match self.tokens.get(self.pos)? {
            Token::If => self.parse_if_else().ok(),
            Token::Number(v) => {
                self.pos += 1;
                Some(Expr::Literal(Value::Int(*v)))
            }
            Token::Float(v) => {
                self.pos += 1;
                Some(Expr::Literal(Value::Float(*v)))
            }
            Token::StringLiteral(v) => {
                self.pos += 1;
                let v = v.clone();
                Some(Expr::Literal(Value::Str(v)))
            }
            Token::Bool(v) => {
                self.pos += 1;
                Some(Expr::Literal(Value::Bool(*v)))
            }
            Token::Id(id) => {
                let name = id.clone();
                self.pos += 1;

                if eat(&Token::OpenParenthese, &self.tokens, &mut self.pos) {
                    let mut args = Vec::new();
                    while !eat(&Token::CloseParenthese, &self.tokens, &mut self.pos) {
                        if let Some(arg) = self.parse_arithmetic(0) {
                            args.push(arg);
                            eat(&Token::Comma, &self.tokens, &mut self.pos);
                        } else {
                            return None;
                        }
                    }
                    Some(Expr::FnCall { 
                        name,
                        args
                    })
                } else {
                    Some(Expr::Id(name))
                }
            }
            Token::OpenParenthese => {
                println!("yay");
                self.pos += 1;
                let expr = match self.parse_arithmetic(0) {
                    Some(e) => e,
                    None => {
                        panic!("Failed to parse expression inside parentheses at pos {}", self.pos);
                    }
                };

                if !eat(&Token::CloseParenthese, &self.tokens, &mut self.pos) {
                    return None;
                }

                Some(expr)
            }
            // Token::CloseBracket => {
            //     // A standalone block is a valid statement in some languages, or maybe error here
            //     println!("Unexpected `}}` without a control structure");
            //     None
            // }
            _ => {
                // NOTE: this is def correct
                self.pos += 1;
                None
            }
        }?;

        if in_condition {
            while let Some(token) = &self.tokens.get(self.pos) {
                let op = match token {
                    Token::Less => BinOp::Lt,
                    Token::LessEqual => BinOp::Le,
                    Token::EqualEqual => BinOp::Eq,
                    Token::Greater => BinOp::Gt,
                    Token::GreaterEqual => BinOp::Ge,
                    // Token::Plus => BinOp::Add,
                    // Token::Dash => BinOp::Sub,
                    // Token::Star => BinOp::Mul,
                    // Token::Slash => BinOp::Div,
                    _ => break,
                };

                self.pos += 1;

                let right_start = self.pos;
                let right = self.parse_arithmetic(0)?;
                if self.pos == right_start {
                    return None;
                }

                left = Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right)
                }
            }
        }
        Some(left)
    }

    // pub fn parse_typed_arg(tokens: &[Token], pos: &mut usize) -> Option<TypedArg> {
    //     todo!()
    // }

    pub fn parse_out_vec_expr(
        &mut self,
    ) -> Result<Vec<Expr>, String> {
        let mut pos = 0;
        let mut exprs = Vec::new();

        while pos < self.tokens.len() {
            if let Some(expr) = self.parse_expr(false) {
                exprs.push(expr);
                if eat(&Token::Comma, &self.tokens, &mut pos) { continue; }
            }
        }

        Ok(exprs)
    }

    pub fn parse_out_vec_stmt(
        &mut self,
    ) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        let tokens = self.tokens.as_slice();

        self.pos = 0;
        let mut pos = self.pos;

        while let Some(token) = tokens.get(pos) {
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => panic!("Parse error at token {:?}: {}\n {:?}", token, e, tokens)
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("TOKENS: {:?}", tokens);

            for stmt in &stmts {
                println!("Parsed stmt: {:?}", stmt);
            }
        }
        stmts
    }
}
