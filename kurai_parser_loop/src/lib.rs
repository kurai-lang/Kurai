use kurai_core::scope::Scope;
use kurai_parser::{parse::parse_block::{parse_block, parse_block_stmt}, BlockParser, FunctionParser, GroupedParsers, ImportParser, LoopParser};
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
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Vec<kurai_stmt::stmt::Stmt>, String> {
        parse_block(tokens, pos, discovered_modules, parsers, scope)
    }

    fn parse_block_stmt(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_block_stmt(tokens, pos, discovered_modules, parsers, scope)
    }
}

pub struct LoopParserStruct;
impl LoopParser for LoopParserStruct {
    fn parse_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_loop(tokens, pos, discovered_modules, parsers, scope)
    }

    fn parse_for_loop(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_for_loop(tokens, pos, discovered_modules, parsers, scope)
    }

    fn parse_while_loop(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String> {
        parse_while_loop(tokens, pos, discovered_modules, parsers, scope)
    }
}
