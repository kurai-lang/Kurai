use colored::Colorize;
use kurai_attr::attribute::Attribute;
use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_token::eat::eat;
use kurai_token::token::token::Token;
use kurai_ast::stmt::Stmt;
use kurai_ast::typedArg::TypedArg;
use kurai_types::typ::Type;

pub fn parse_fn_decl(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
    attrs: Vec<Attribute>,
) -> Result<Stmt, String> {
    let mut is_extern = false;

    if eat(&Token::Extern, tokens, pos) {
        is_extern = true;
    }

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
        while let Some(token) = tokens.get(*pos) {
            match token {
                Token::CloseParenthese => {
                    *pos += 1;
                    break;
                }
                // Some(Token::If) => {
                //     let expr = parse_if_else(tokens, pos, discovered_modules, parsers, scope)?;
                //     return Ok(Stmt::Expr(expr)); // wrap it in a Stmt!
                // }
                Token::Id(arg_name) => {
                    let name = arg_name.clone();
                    *pos += 1;

                    if !eat(&Token::Colon, tokens, pos) {
                        return Err(format!("Expected `:` after argument name {}", name));
                    }

                    let typ = match tokens.get(*pos) {
                        // Some(Token::Id(type_name)) => {
                        //     *pos += 1;
                        //     type_name.clone()
                        // }
                        Some(Token::Type(typ)) => {
                            *pos += 1;
                            typ.clone()
                        }
                        _ => return Err(format!("Expected a type name after `:` in argument {}", name))
                    };

                    // name: type
                    args.push(TypedArg {
                        name,
                        typ,
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

    let mut star_count = 0;
    while let Some(Token::Star) = tokens.get(*pos) {
        *pos += 1;
        star_count += 1;
        // NOTE: not yet bro
        // ret_type = Type::Ptr(Box::new((ret_type)))
    }
    #[cfg(debug_assertions)]
    println!("{}: parsing return type", "debug".cyan().bold());
    let mut ret_type = match tokens.get(*pos) {
        Some(Token::Type(typ)) => {
            *pos += 1;
            typ.clone()
        },
        _ => Type::Void
    };

    for _ in 0..star_count {
        ret_type = Type::Ptr(Box::new(ret_type));
    }
    #[cfg(debug_assertions)]
    println!("{}: return type of function {} is {:?}", "debug".cyan().bold(), name, ret_type);

    let mut body = Vec::new();

    if is_extern {
        if !eat(&Token::Semicolon, tokens, pos) {
            return Err("Expected `;` after extern function declaration".to_string());
        }
    } else if eat(&Token::OpenBracket, tokens, pos) {
        while *pos < tokens.len() {
            while let Some(token) = tokens.get(*pos) {
                match token {
                    Token::CloseBracket => {
                        #[cfg(debug_assertions)]
                        println!("{}: end of function block", "debug".cyan().bold());
                        *pos += 1;

                        break;
                    }
                    _ => {
                    let old_pos = *pos;
                    match parsers.stmt_parser.parse_stmt(
                        tokens, 
                        pos,
                        discovered_modules, 
                        parsers,
                        scope
                    ) {
                        Ok(stmt) => {
                            if *pos == old_pos {
                                return Err(format!(
                                    "{}: parse_stmt() made no progress at pos {}: {:?}", "debug".cyan().bold(),
                                    *pos,
                                    tokens.get(*pos),
                                ));
                            }
                            body.push(stmt);
                        }
                        Err(e) => {
                            println!(
                                "{}: statement failed at pos {} → token: {:?}",
                                "debug".cyan().bold(),
                                *pos,
                                tokens.get(*pos)
                            );

                            return Err(format!(
                                "Error parsing statement in function `{}` at pos {} (token: {:?}):\n{}",
                                name,
                                *pos,
                                tokens.get(*pos),
                                e
                            ));
                        }
                    }
                }
                }
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
        ret_type,
        is_extern
    })
}
