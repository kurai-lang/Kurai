use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;
use kurai_typedArg::typedArg::TypedArg;
use kurai_parser_import_decl::parse_import_decl::parse_import_decl;

pub trait FunctionParser {
    fn parse_fn_decl(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
    fn parse_fn_call(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
}

pub trait Parser {
    fn parse_var_decl(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
    fn parse_var_assign(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
    fn parse_if_else(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
}

pub fn parse_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn FunctionParser,
    parser: &dyn Parser,
) -> Result<Stmt, String> {
    match tokens.get(*pos) {
        Some(Token::Function) => fn_parser.parse_fn_decl(tokens, pos, discovered_modules),
        Some(Token::Let) => parser.parse_var_decl(tokens, pos),
        Some(Token::Import) => parse_import_decl(tokens, pos, discovered_modules),
        Some(Token::If) => parser.parse_if_else(tokens, pos, discovered_modules),
        Some(Token::Id(_)) => {
            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => fn_parser.parse_fn_call(tokens, pos),
                Some(Token::Equal) => parser.parse_var_assign(tokens, pos),
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
