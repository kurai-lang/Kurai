use std::{fmt::format, ops::Deref};

use colored::Colorize;
use inkwell::{basic_block::BasicBlock, values::FunctionValue, IntPredicate};

use kurai_parser::{FunctionParser, ImportParser, StmtParser};
// use kurai_core::parse::{bin_op::BinOp, expr::Expr, stmt::Stmt};
use kurai_types::value::Value;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;
use kurai_binop::bin_op::BinOp;
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
        stmt_parser: &dyn StmtParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
    ) -> BasicBlock<'ctx> {
        let condition = self.lower_expr_to_llvm(condition_expr, true).unwrap();

        // convert condition to boolean if needed
        let bool_cond = match condition_expr { // If it's already a comparison operation, use it as-is
            Expr::Binary { op: BinOp::Gt | BinOp::Lt | BinOp::Ge | BinOp::Le, left, right } => {
                // Already returns i1 boolean
                condition.into_int_value()
            }
            Expr::Literal(Value::Bool(_)) => {
                // Already returns i1 boolean
                condition.into_int_value()
            }
            // For other expressions, convert to boolean if integer
            _ => {
                println!("Other operations not found");
                if condition.is_int_value() {
                    println!("Condition is int value");
                    let zero = self.context.bool_type().const_int(0, false);
                    self.builder.build_int_compare(
                        IntPredicate::NE,
                        condition.into_int_value(),
                        zero,
                        "bool_cond"
                    ).unwrap()
                } else {
                    condition.into_int_value() // Already boolean
                }
            }
        };

        let then_block = self.context.append_basic_block(current_function, &format!("then_{}", block_suffix));
        // let else_block = else_body.as_ref().map(|_| {
        //     self.context.append_basic_block(current_function, &format!("else_{}", block_suffix))
        // });
        let else_block = self.context.append_basic_block(current_function, &format!("else_{}", block_suffix));
        let merge_block = self.context.append_basic_block(current_function, &format!("merge_{}", block_suffix));

        self.builder.build_conditional_branch(
            bool_cond,
            then_block,
            else_block
        ).unwrap();

        // then block
        self.builder.position_at_end(then_block);
        self.execute_every_stmt_in_code(then_body.to_vec(), discovered_modules, stmt_parser, fn_parser, import_parser);
        self.builder.build_unconditional_branch(merge_block).unwrap();

        // generate else block if it exists
        self.builder.position_at_end(else_block);
        if let Some(else_stmts) = else_body.as_ref() {
            self.execute_every_stmt_in_code(else_stmts.to_vec(), discovered_modules, stmt_parser, fn_parser, import_parser);
        }
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(merge_block);
        merge_block
    }
}
