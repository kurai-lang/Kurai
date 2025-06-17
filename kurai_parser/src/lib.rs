use kurai_core::scope::Scope;
use kurai_stmt::stmt::Stmt;
use kurai_token::token::token::Token;

pub mod parse;

pub trait ImportParser {
    fn parse_import_decl(&self, tokens: &[Token], pos: &mut usize, discovered_modules: &mut Vec<String>) -> Result<Stmt, String>;
    fn parse_imported_file(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        stmt_parser: &dyn StmtParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<kurai_stmt::stmt::Stmt, String>;
}

pub trait StmtParser {
    fn parse_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<Stmt, String>;
}

pub trait BlockParser {
    fn parse_block(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        block_parser: &dyn BlockParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<Vec<Stmt>, String>;
}

pub trait LoopParser {
    fn parse_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        block_parser: &dyn BlockParser,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<Stmt, String>;
}

pub trait FunctionParser {
    fn parse_fn_decl(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser,
        scope: &Scope,
    ) -> Result<Stmt, String>;
    fn parse_fn_call(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
}
