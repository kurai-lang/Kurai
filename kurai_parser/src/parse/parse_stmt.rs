use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;
use kurai_typedArg::typedArg::TypedArg;

use crate::{parse::{parse::parse_expr, parse_if_else::parse_if_else, parse_var_assign::parse_var_assign, parse_var_decl::parse_var_decl}, FunctionParser, ImportParser};

pub fn parse_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
) -> Result<Stmt, String> {
    match tokens.get(*pos) {
        Some(Token::Function) => fn_parser.parse_fn_decl(tokens, pos, discovered_modules),
        Some(Token::Let) => parse_var_decl(tokens, pos),
        Some(Token::Import) => import_parser.parse_import_decl(tokens, pos, discovered_modules),
        Some(Token::If) => parse_if_else(tokens, pos, discovered_modules, fn_parser, import_parser),
        Some(Token::Id(_)) => {
            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => fn_parser.parse_fn_call(tokens, pos),
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
