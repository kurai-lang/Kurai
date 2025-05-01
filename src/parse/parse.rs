use crate::token::token::Token;
use crate::parse::stmt::Stmt;
use crate::value::Value;
use crate::eat::eat;

fn parse_fn(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if !eat(&Token::Function, tokens, pos) {
        return None;
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return None
    };

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return None;
    }

    // TODO: Add arguments passing here later 
    
    if !eat(&Token::CloseParenthese, tokens, pos) {
        return None;
    }

    let mut body = Vec::new();
    if eat(&Token::OpenBracket, tokens, pos) {
        while *pos < tokens.len() {
            if let Some(Token::CloseBracket) = tokens.get(*pos) {
                *pos += 1;
                break;
            }

            if let Some(stmt) = parse_stmt(tokens, pos) {
                body.push(stmt);
            } else {
                return None;
            }
        }
    } else {
        return None;
    }

    Some(Stmt::FnDecl { 
        name,
        args: vec![], // bro got skipped 💀
        body,
    })
}
// this function just wants to return stmt
pub fn parse_stmt(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    match tokens.get(*pos) {
        Some(Token::Function) => {
            parse_fn(tokens, pos)
        }
        // Some(Token::Int) => {
        //     // parse_var_decl(tokens, pos)
        // }
        // Some(Token::Id(_)) => {
        //     // parse_func_call(tokens, pos)
        // }
        _ => None
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        if let Some(stmt) = parse_stmt(tokens, &mut pos) {
            stmts.push(stmt)
        } else {
            panic!("Parse error at token {:?}", tokens.get(pos));
        }
    }

    println!("{:?}", tokens);
    stmts
}
