use colored::Colorize;
use kurai_attr::attribute::Attribute;
use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use kurai_stmt::stmt::Stmt;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::typ::Type;

pub fn parse_fn_decl(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    attrs: Vec<Attribute>,
) -> Result<Stmt, String> {
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

    // NOTE: arguments parsing time
    let mut args = Vec::new();
    if let Some(&Token::CloseParenthese) = tokens.get(*pos) {
        *pos += 1;
    } else {
        while *pos < tokens.len() {
            match tokens.get(*pos) {
                Some(Token::CloseParenthese) => {
                    *pos += 1;
                    break;
                }
                Some(Token::Id(arg_name)) => {
                    let name = arg_name.clone();
                    *pos += 1;

                    if !eat(&Token::Colon, tokens, pos) {
                        return Err(format!("Expected `:` after argument name {}", name));
                    }

                    let typ = match tokens.get(*pos) {
                        Some(Token::Id(type_name)) => {
                            *pos += 1;
                            type_name.clone()
                        }
                        _ => return Err(format!("Expected a type name after `:` in argument {}", name))
                    };

                    // name: type
                    args.push(TypedArg {
                        name,
                        typ: Type::from_str(&typ).unwrap(),
                        value: None,
                    });

                    if let Some(Token::Comma) = tokens.get(*pos) {
                        *pos += 1;
                    }
                }
                _ => return Err("Invalid argument syntax inside function declaration".to_string())
                // _ => None
            }
        }
    }

    #[cfg(debug_assertions)]
    println!("{}: parsing return type", "debug".cyan().bold());
    let ret_type = if let Some(Token::Type(typ)) = tokens.get(*pos) {
        *pos += 1;
        typ.clone()
    } else {
        Type::Void
    };
    #[cfg(debug_assertions)]
    println!("{}: return type of function {} is {:?}", "debug".cyan().bold(), name, ret_type);

    let mut body = Vec::new();
    if eat(&Token::OpenBracket, tokens, pos) {
        while *pos < tokens.len() {
            if let Some(Token::CloseBracket) = tokens.get(*pos) {
                *pos += 1;
                break;
            }

            match parsers.stmt_parser.parse_stmt(
                tokens, 
                pos,
                discovered_modules, 
                parsers,
                scope
            ) {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(format!("Couldnt work on the body\nREASON: {}", e))
            }
        }
    } else {
        return Err("Expected an opening bracket".to_string());
    }

    Ok(Stmt::FnDecl { 
        name,
        args,
        body,
        attributes: attrs,
        ret_type
    })
}
