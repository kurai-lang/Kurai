use inkwell::{basic_block::BasicBlock, values::{BasicValue, BasicValueEnum, FunctionValue}};

use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;
use kurai_types::typ::Type;
use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn build_conditional_branch(
        &mut self,
        current_function: FunctionValue<'ctx>,
        condition_expr: &Stmt,
        then_body: &[Stmt],
        else_body: &Option<Vec<Stmt>>,
        discovered_modules: &mut Vec<String>,
        block_suffix: &str,
        expected_type: Option<&Type>,
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) -> Option<BasicValueEnum<'ctx>> 
    {
        let condition_value = self.lower_expr_to_llvm(
            condition_expr,
            Some(&Type::Bool),
            discovered_modules,
            scope,
            parsers,
        ).unwrap();

        let condition = if condition_value.is_int_value() {
            let int_val = condition_value.into_int_value();
            let zero = int_val.get_type().const_zero();

            self.builder.build_int_compare(
                inkwell::IntPredicate::NE,
                int_val,
                zero,
                "cond_tmp").unwrap()
        } else {
            panic!("condition must evaluate to an integer value");
        };

        let then_block = self.context.append_basic_block(current_function, &format!("then_{}", block_suffix));
        let else_block = self.context.append_basic_block(current_function, &format!("else_{}", block_suffix));
        let merge_block = self.context.append_basic_block(current_function, &format!("merge_{}", block_suffix));

        self.builder.build_conditional_branch(
            condition,
            then_block,
            else_block
        ).unwrap();

        // then block
        self.builder.position_at_end(then_block);

        // let mut last_val: Option<_> = None;
        let then_val = {
            let mut val = None;
            for (i, expr) in then_body.iter().enumerate() {
                let expected = if i == then_body.len() - 1 { expected_type } else { None };
                val = self.lower_expr_to_llvm(
                    expr,
                    expected,
                    discovered_modules,
                    scope,
                    parsers,
                );
            }
            val
        };
        self.builder.build_unconditional_branch(merge_block).unwrap();

        // generate else block if it exists
        self.builder.position_at_end(else_block);
        let else_val = if let Some(else_exprs) = else_body.as_ref() {
            let mut val = None;
            for (i, else_expr) in else_exprs.iter().enumerate() {
                let expected = if i == else_exprs.len() - 1 { expected_type } else { None };
                val = self.lower_expr_to_llvm(
                    else_expr,
                    expected,
                    discovered_modules,
                    scope,
                    parsers,
                );
            }

            val
        } else {
            None
        };
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(merge_block);
        if let Some(expected_type) = expected_type {
            let phi = self.builder.build_phi(
                expected_type.to_llvm_type(self.context).unwrap(),
                "if_result"
            ).unwrap();

            if let Some(val) = then_val {
                phi.add_incoming(&[(&val, then_block)]);
            }

            if let Some(val) = else_val {
                phi.add_incoming(&[(&val, else_block)]);
            }

            return Some(phi.as_basic_value().as_basic_value_enum());
        } else {
            return None;
        }
        // self.builder.build_return(None).unwrap();
    }
}
