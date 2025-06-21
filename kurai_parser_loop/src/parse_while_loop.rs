use kurai_core::scope::Scope;
use kurai_expr::expr::Expr;
use kurai_parser::{parse::{parse::parse_expr, parse_block::parse_block}, BlockParser, LoopParser};
use kurai_stmt::stmt::{IfBranch, Stmt};
use kurai_token::{eat::eat, token::token::Token};

pub fn parse_while_loop(
    tokens: &[kurai_token::token::token::Token],
    pos: &mut usize,
    block_parser: &dyn BlockParser,
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn kurai_parser::FunctionParser,
    import_parser: &dyn kurai_parser::ImportParser,
    loop_parser: &dyn LoopParser,
    scope: &mut Scope,
) -> Result<Stmt, String> {
    if !eat(&Token::While, tokens, pos) {    
        return Err("Expected `while`".to_string());
    }

    // if !eat(&Token::OpenParenthese, tokens, pos) {
    //     return Err("Expected `(` after `while`".to_string());
    // }

    println!("parsing conditions...");
    let condition = parse_expr(tokens, pos, true)
        .ok_or_else(|| format!("Failed to parse expression inside `while (...)` at token {}", pos))?;
    println!("parsing conditions succeed");

    // if !eat(&Token::CloseParenthese, tokens, pos) {
    //     return Err("Expected `)` after `while` condition".to_string());
    // }

    let body = parse_block(
        tokens,
        pos,
        discovered_modules,
        block_parser,
        fn_parser,
        import_parser,
        loop_parser,
        scope,
    )?;

    Ok(Stmt::Block(vec![
        Stmt::Loop { 
            body: vec![
                Stmt::If {
                    branches: vec![IfBranch {
                        condition,
                        body,
                    }],
                    else_body: Some(vec![Stmt::Break]),
                }
            ]
        }]
    ))
}
