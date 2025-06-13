use inkwell::{values::{BasicValue, BasicValueEnum}, IntPredicate};

use kurai_binop::bin_op::BinOp;
use kurai_expr::expr::Expr;
use kurai_types::value::Value;
use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn lower_expr_to_llvm(&self, expr: &Expr, in_condition: bool) -> Option<BasicValueEnum<'ctx>> {
        println!("Lowering expr: {:?}", expr);
        match expr {
            Expr::Literal(value) => match value {
                Value::Int(v) => {
                    Some(self.context.i64_type().const_int(*v as u64, true).into())
                }
                Value::Bool(b) => {
                    Some(self.context.bool_type().const_int(*b as u64, false).into())
                }
                _ => None
            },
            Expr::Var(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    let loaded = self.builder.build_load(self.context.i64_type(), *ptr, &format!("load_{}", name));
                    Some(loaded.unwrap())
                } else {
                    println!("Variable {} not found!", name);
                    None
                }
            }
            Expr::Id(_) => todo!(),
            Expr::Binary { op, left, right } => {
                let left_val = self.lower_expr_to_llvm(left, false)?;
                let right_val = self.lower_expr_to_llvm(right, false)?;

                let predicate = match op {
                    BinOp::Lt => IntPredicate::SLT,
                    BinOp::Le => IntPredicate::SLE,
                    BinOp::Eq => IntPredicate::EQ,
                    BinOp::Ge => IntPredicate::SGE,
                    BinOp::Gt => IntPredicate::SGT,
                    _ => {
                        panic!("Operator {:?} not supported", op);
                        return None;
                    }
                };

                let cmp_result = self.builder.build_int_compare(
                    predicate,
                    left_val.into_int_value(),
                    right_val.into_int_value(),
                    "cmp"
                ).unwrap();
                Some(cmp_result.as_basic_value_enum())
            }
            Expr::FnCall { name, args } => todo!(),
        }
    }
}
