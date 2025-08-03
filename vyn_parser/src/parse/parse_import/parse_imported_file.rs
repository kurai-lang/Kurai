use vyn_ast::stmt::Stmt;

use crate::parse::Parser;

impl Parser {
    pub fn parse_imported_file(
        &mut self,
    ) -> Result<Stmt, String> {
        self.parse_stmt()
            .map_err(|_| "Failed to parse imported file content".to_string())
    }
}
