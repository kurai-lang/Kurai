use colored::Colorize;
use inkwell::{values::{BasicValue, BasicValueEnum}, IntPredicate};

use crate::codegen::CodeGen;
use kurai_expr::expr::Expr;
use kurai_binop::bin_op::BinOp;
use kurai_types::value::Value;

// impl<'ctx> CodeGen<'ctx> {
//     // The reason why this function returns something and execute_every_expr_in_code doesnt
//     // is because expr returns a value meanwhile stmt doesnt 
//     // go learn about expr and stmt if youre confused xD
//     pub fn execute_every_expr_in_code(&mut self, exprs: Vec<Expr>) -> Result<BasicValueEnum<'ctx>, String> {
//         let mut result = Err("Empty expression list".to_string());
//
//         for expr in exprs {
//             dbg!(&expr);
//
//             result = match expr {
//                 Expr::Literal(Value::Int(v)) => {
//                     Ok(self.context.i64_type().const_int(v as u64, true).as_basic_value_enum())
//                 }
//                 Expr::Binary { op, left, right } => {
//                     println!("{} Entering Expr::Binary case", "[execute_every_expr_in_code]".green().bold());
//                     let left_val = self.execute_every_expr_in_code(vec![*left])?;
//                     let right_val = self.execute_every_expr_in_code(vec![*right])?;
//                     println!("{} left_val:{:?}\nright_val:{:?}", "[execute_every_expr_in_code()]".green().bold(), left_val, right_val);
//
//                     match op {
//                         BinOp::Lt | BinOp::Le | BinOp::Eq | BinOp::Ge | BinOp::Gt | BinOp::Ne => {
//                             let op: Result<IntPredicate, String> = match op {
//                                 BinOp::Lt => Ok(IntPredicate::SLT),
//                                 BinOp::Le => Ok(IntPredicate::SLE),
//                                 BinOp::Eq => Ok(IntPredicate::EQ),
//                                 BinOp::Ne => Ok(IntPredicate::NE),
//                                 BinOp::Gt => Ok(IntPredicate::SGT),
//                                 BinOp::Ge => Ok(IntPredicate::SGE),
//                                 // let cmp = if left_val.is_int_value() {
//                                 //     self.builder.build_int_compare(
//                                 //         IntPredicate::EQ,
//                                 //         left_val.into_int_value(),
//                                 //         right_val.into_int_value(),
//                                 //         "eq"
//                                 //     )
//                                 _ => Err("Unsupported operator".to_string())
//                             };
//
//                             Ok(self.builder.build_int_compare(
//                                 op.unwrap(),
//                                 left_val.into_int_value(),
//                                 right_val.into_int_value(),
//                                 "cmp"
//                             ).unwrap().as_basic_value_enum())
//                         }
//
//
//                         // Arithmetic'ing time
//                         BinOp::Add => {
//                             let sum = self.builder.build_int_add(
//                                 left_val.into_int_value(),
//                                 right_val.into_int_value(),
//                                 "addtmp",
//                             ).unwrap();
//
//                             Ok(sum.as_basic_value_enum())
//                         }
//                         BinOp::Sub => {
//                             let diff = self.builder.build_int_sub(
//                                 left_val.into_int_value(),
//                                 right_val.into_int_value(),
//                                 "subtmp",
//                             ).unwrap();
//
//                             Ok(diff.as_basic_value_enum())
//                         }
//                         BinOp::Mul => {
//                             let product = self.builder.build_int_mul(
//                                 left_val.into_int_value(),
//                                 right_val.into_int_value(),
//                                 "multmp",
//                             ).unwrap();
//
//                             Ok(product.as_basic_value_enum())
//                         }
//                         BinOp::Div => {
//                             let div = self.builder.build_int_signed_div(
//                                 left_val.into_int_value(),
//                                 right_val.into_int_value(),
//                                 "divtmp",
//                             ).unwrap();
//
//                             Ok(div.as_basic_value_enum())
//                         }
//                     }
//                 }
//                 _ => Err("Unsupported expression".to_string())
//             }
//         }
//
//         result
//     }
// }
