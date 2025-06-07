use kurai_token::eat::eat;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;

pub fn parse_import_decl(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    if !eat(&Token::Import, tokens, pos) {
        return Err("Expected keyword `use` or `gunakan`".to_string());
    }
    println!("\n[DEBUG] parse_import called at pos={}", pos);
    println!("Current token: {:?}", tokens.get(*pos));
    println!("Next few tokens: {:?}", tokens.iter().skip(*pos).take(5).collect::<Vec<_>>());

    let mut path = Vec::new();

    loop {
        match tokens.get(*pos) {
            Some(Token::Id(name)) => {
                path.push(name.clone());
                discovered_modules.push(name.clone());
                *pos += 1;
            }
            _ => break 
        };

        // Detects `::`? then continue scanning and pushing to `path` vector variable
        if !(eat(&Token::Colon, tokens, pos) && eat(&Token::Colon, tokens, pos)) {
            break;
        }
    }

    let nickname = if eat(&Token::As, tokens, pos) {
        match tokens.get(*pos) {
            Some(Token::Id(name)) => {
                *pos += 1;
                Some(name.clone())
            },
            _ => return Err("Expected identifier after `as`".to_string()),
        }
    } else {
        None
    };

    if !eat(&Token::Semicolon, tokens, pos) {
        return Err("Expected semicolon after giving the package a nickname".to_string());
    }

    Ok(Stmt::Import {
        path,
        nickname,
    })
}
