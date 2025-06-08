use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use kurai_parser_stmt::parse_stmt::parse_stmt;
use kurai_stmt::stmt::Stmt;

pub fn parse_fn_decl(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    if !eat(&Token::Function, tokens, pos) {
        return Err("Expected keyword `fn`".to_string());
    }

    let name = match tokens.get(*pos) {
        Some(Token::Id(name)) => {
            *pos += 1;
            name.clone()
        }
        _ => return Err("Expected an identifier name after keyword `fn`".to_string())
    };

    if !eat(&Token::OpenParenthese, tokens, pos) {
        return Err(format!("Expected an opening paranthese `(` after `{}`", name));
    }

    // TODO: Add arguments passing here later

    if !eat(&Token::CloseParenthese, tokens, pos) {
        return Err("Expected a closing paranthese `)` after passing in arguments".to_string());
    }

    let mut body = Vec::new();
    if eat(&Token::OpenBracket, tokens, pos) {
        while *pos < tokens.len() {
            if let Some(Token::CloseBracket) = tokens.get(*pos) {
                *pos += 1;
                break;
            }

            match parse_stmt(tokens, pos, discovered_modules) {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(format!("Couldnt work on the body\nREASON: {}", e))
            }
        }
    } else {
        return Err("Expected an opening bracket".to_string());
    }

    Ok(Stmt::FnDecl { 
        name,
        args: vec![], // bro got skipped 💀
        body,
    })
}
