use kurai_parser_function::parse::parse_fn_decl::parse_fn_decl;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;
use kurai_typedArg::typedArg::TypedArg;

pub fn parse_stmt(tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String> {
    match tokens.get(*pos) {
        Some(Token::Function) => parse_fn_decl(tokens, pos, discovered_modules),
        Some(Token::Let) => parse_var_decl(tokens, pos),
        Some(Token::Import) => parse_import_decl(tokens, pos, discovered_modules),
        Some(Token::If) => parse_if_else(tokens, pos, discovered_modules),
        Some(Token::Id(_)) => {
            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => parse_fn_call(tokens, pos),
                Some(Token::Equal) => parse_var_assign(tokens, pos),
                _ => Err("Identifier expected, is this supposed to be a function call or variable assignment?".to_string())
            }
        }
        _ => match parse_expr(tokens, pos, false) {
            Some(Expr::FnCall { name, args }) => {
                let typed_args = args.into_iter().map(|arg|
                    TypedArg {
                        name: name.clone(),
                        typ: "any".to_string(),
                        value: Some(arg),
                    }).collect();

                Ok(Stmt::FnCall { name, args: typed_args })
            }
            Some(expr) => Err(format!("Standalone expressions not allowed: {:?}", expr)),
            None => Err(format!("Invalid statement: {:?}", tokens.get(*pos)))
        }
    }
}
