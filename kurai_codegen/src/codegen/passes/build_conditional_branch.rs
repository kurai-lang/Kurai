use inkwell::{basic_block::BasicBlock, values::FunctionValue};

use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;
use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn build_conditional_branch(
        &mut self,
        current_function: FunctionValue<'ctx>,
        condition_expr: &Expr,
        then_body: &[Stmt],
        else_body: &Option<Vec<Stmt>>,
        discovered_modules: &mut Vec<String>,
        block_suffix: &str,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> BasicBlock<'ctx> {
        let condition = self.lower_expr_to_llvm(condition_expr).unwrap();

        let then_block = self.context.append_basic_block(current_function, &format!("then_{}", block_suffix));
        let else_block = self.context.append_basic_block(current_function, &format!("else_{}", block_suffix));
        let merge_block = self.context.append_basic_block(current_function, &format!("merge_{}", block_suffix));

        self.builder.build_conditional_branch(
            condition.into_int_value(),
            then_block,
            else_block
        ).unwrap();

        // then block
        self.builder.position_at_end(then_block);
        self.execute_every_stmt_in_code(
            then_body.to_vec(),
            discovered_modules,
            parsers,
            scope
        );
        self.builder.build_unconditional_branch(merge_block).unwrap();

        // generate else block if it exists
        self.builder.position_at_end(else_block);
        if let Some(else_stmts) = else_body.as_ref() {
            self.execute_every_stmt_in_code(
                else_stmts.to_vec(),
                discovered_modules,
                parsers,
                scope
            );
        }
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(merge_block);
        merge_block
    }
}
