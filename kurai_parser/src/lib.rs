use kurai_attr::attribute::Attribute;
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
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<kurai_stmt::stmt::Stmt, String>;
}

pub trait StmtParser {
    fn parse_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String>;
}

pub trait BlockParser {
    fn parse_block(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Vec<Stmt>, String>;

    fn parse_block_stmt(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String>;
}

pub trait LoopParser {
    fn parse_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String>;

    fn parse_for_loop(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String>;

    fn parse_while_loop(
        &self,
        tokens: &[kurai_token::token::token::Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Result<Stmt, String>;
}

pub trait FunctionParser {
    fn parse_fn_decl(
        &self,
        tokens: &[Token],
        pos: &mut usize,
        discovered_modules: &mut Vec<String>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
        attrs: Vec<Attribute>,
    ) -> Result<Stmt, String>;
    fn parse_fn_call(&self, tokens: &[Token], pos: &mut usize) -> Result<Stmt, String>;
    fn parse_attrs(&self, tokens: &[Token], pos: &mut usize) -> Result<Vec<Attribute>, String>;
}

pub struct GroupedParsers<'a> {
    pub stmt_parser: &'a dyn StmtParser,
    pub fn_parser: &'a dyn FunctionParser,
    pub import_parser: &'a dyn ImportParser,
    pub block_parser: &'a dyn BlockParser,
    pub loop_parser: &'a dyn LoopParser,
}

impl<'a> GroupedParsers<'a> {
    pub fn new(
        stmt_parser: &'a dyn StmtParser,
        fn_parser: &'a dyn FunctionParser,
        import_parser: &'a dyn ImportParser,
        block_parser: &'a dyn BlockParser,
        loop_parser: &'a dyn LoopParser,
    ) -> Self {
        GroupedParsers {
            stmt_parser,
            fn_parser,
            import_parser,
            block_parser,
            loop_parser,
        }
    }
}
