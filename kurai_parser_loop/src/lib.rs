use kurai_core::scope::Scope;
use kurai_parser::{parse::parse_block::{parse_block, parse_block_stmt}, BlockParser, FunctionParser, ImportParser, LoopParser};
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;

use crate::parse_for_loop::parse_for_loop;
use crate::parse_loop::parse_loop;
use crate::parse_while_loop::parse_while_loop;

pub mod parse_loop;
pub mod parse_for_loop;
pub mod parse_while_loop;

pub struct BlockParserStruct;
impl BlockParser for BlockParserStruct {
    fn parse_block(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn kurai_parser::FunctionParser,
        import_parser: &dyn kurai_parser::ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Vec<kurai_stmt::stmt::Stmt>, String> {
        parse_block(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }

    fn parse_block_stmt(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn kurai_parser::FunctionParser,
        import_parser: &dyn kurai_parser::ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_block_stmt(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }
}

pub struct LoopParserStruct;
impl LoopParser for LoopParserStruct {
    fn parse_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        block_parser: &dyn BlockParser,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_loop(tokens, pos, block_parser, discovered_modules, fn_parser, import_parser, loop_parser, scope)
    }

    fn parse_for_loop(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        block_parser: &dyn BlockParser,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn kurai_parser::FunctionParser,
        import_parser: &dyn kurai_parser::ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_for_loop(tokens, pos, discovered_modules, block_parser, fn_parser, import_parser, loop_parser, scope)
    }

    fn parse_while_loop(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        block_parser: &dyn BlockParser,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn kurai_parser::FunctionParser,
        import_parser: &dyn kurai_parser::ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_while_loop(tokens, pos, block_parser, discovered_modules, fn_parser, import_parser, loop_parser, scope)
    }
}
