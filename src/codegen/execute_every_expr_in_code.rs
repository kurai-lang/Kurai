use inkwell::{values::{BasicValue, BasicValueEnum}, IntPredicate};

use crate::{parse::{bin_op::BinOp, expr::Expr}, value::Value};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    // The reason why this function returns something and execute_every_expr_in_code doesnt
    // is because expr returns a value meanwhile stmt doesnt 
    // go learn about expr and stmt if youre confused xD
    pub fn execute_every_expr_in_code(&mut self, exprs: Vec<Expr>) -> Result<BasicValueEnum<'ctx>, String> {
        let mut result = Err("Empty expression list".to_string());

        for expr in exprs {
            result = match expr {
                Expr::Literal(Value::Int(v)) => {
                    Ok(self.context.i64_type().const_int(v as u64, true).as_basic_value_enum())
                }
                Expr::Binary { op, left, right } => {
                    let left_val = self.execute_every_expr_in_code(vec![*left])?;
                    let right_val = self.execute_every_expr_in_code(vec![*right])?;

                    let op: Result<IntPredicate, String> = match op {
                        BinOp::Eq => Ok(IntPredicate::EQ),
                        BinOp::Le => Ok(IntPredicate::SLE),
                        // let cmp = if left_val.is_int_value() {
                        //     self.builder.build_int_compare(
                        //         IntPredicate::EQ,
                        //         left_val.into_int_value(),
                        //         right_val.into_int_value(),
                        //         "eq"
                        //     )
                        _ => Err("Unsupported operator".to_string())
                    };

                    Ok(self.builder.build_int_compare(
                        op.unwrap(),
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "cmp"
                    ).unwrap().as_basic_value_enum())
                }
                _ => Err("Unsupported expression".to_string())
            }
        }

        result
    }
}
