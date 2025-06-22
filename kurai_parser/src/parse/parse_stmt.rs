use kurai_core::scope::Scope;
use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use kurai_token::{eat::eat, token::token::Token};
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::typ::Type;

use crate::{parse::{parse::parse_expr, parse_expr::parse_arithmetic::parse_arithmetic, parse_if_else::parse_if_else, parse_var_assign::parse_var_assign, parse_var_decl::parse_var_decl}, BlockParser, FunctionParser, GroupedParsers, ImportParser, LoopParser, StmtParser};

pub struct StmtParserStruct;
impl StmtParser for StmtParserStruct {
    fn parse_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_stmt(tokens, pos, discovered_modules, parsers, scope)
    }
}

pub fn parse_stmt(
    tokens: &[Token],
    pos: &mut usize,
    discovered_modules: &mut Vec<String>,
    parsers: &GroupedParsers,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    // println!("[parse_stmt] Entering at pos = {}, token = {:?}", *pos, tokens.get(*pos));

    match tokens.get(*pos) {
        Some(Token::Function) => parsers.fn_parser.parse_fn_decl(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope
        ),
        Some(Token::Loop) => parsers.loop_parser.parse_for_loop(
            tokens,
            pos,
            discovered_modules,
            parsers,
            scope
        ),
        Some(Token::While) => parsers.loop_parser.parse_while_loop(tokens, pos, discovered_modules, parsers, scope),
        Some(Token::Break) => {
            *pos += 1;
            if !eat(&Token::Semicolon, tokens, pos) {
                return Err("Expected ';' after `break`".to_string());
            }

            *pos += 1;
            Ok(Stmt::Break)
        }
        Some(Token::Let) => parse_var_decl(tokens, pos, scope),
        Some(Token::Import) => parsers.import_parser.parse_import_decl(tokens, pos, discovered_modules),
        Some(Token::If) => parse_if_else(tokens, pos, discovered_modules, parsers, scope),
        Some(Token::For) => parsers.loop_parser.parse_for_loop(tokens, pos, discovered_modules, parsers, scope),
        Some(Token::Id(_)) => {
                // For functions from modules. like foo::bar()
                if let (Some(Token::Colon), Some(Token::Colon)) =
                    (tokens.get(*pos + 1), tokens.get(*pos + 2))
                {
                    return parsers.fn_parser.parse_fn_call(tokens, pos);
                }

            match tokens.get(*pos + 1) {
                Some(Token::OpenParenthese) => parsers.fn_parser.parse_fn_call(tokens, pos),
                Some(Token::Equal) => parse_var_assign(tokens, pos, scope),
                _ => Err("Identifier expected, is this supposed to be a function call or variable assignment?".to_string())
            }
        }
        Some(Token::OpenBracket) => {
            #[cfg(debug_assertions)]
            { println!("Some(Token::OpenBracket)"); }
            let stmts = parsers.block_parser.parse_block(tokens, pos, discovered_modules, parsers, scope)?;
            Ok(Stmt::Block(stmts))
        }
        _ => {
            let start_pos = *pos;
            match parse_arithmetic(tokens, pos, 0) {
                Some(Expr::FnCall { name, args }) if *pos > start_pos => {
                    let typed_args = args
                        .into_iter()
                        .map(|arg| TypedArg {
                            name: name.clone(),
                            typ: Type::Unknown,
                            value: Some(arg),
                        })
                        .collect();

                    Ok(Stmt::FnCall { name, args: typed_args })
                }
                Some(expr) if *pos > start_pos => Ok(Stmt::Expr(expr)),
                _ => Err(format!(
                    "Invalid statement or no progress at pos {}: {:?}",
                    pos,
                    tokens.get(*pos)
                )),
            }
        }
    }
}
