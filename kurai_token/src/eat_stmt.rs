use kurai_stmt::stmt::Stmt;

pub fn eat_stmt(expected: &Stmt, stmts: &[Stmt], pos: &mut usize) -> bool {
    if *pos < stmts.len() && stmts.get(*pos) == Some(expected) {
        *pos += 1;
        true
    } else {
        false
    }
}
