use inkwell::{module::Linkage, values::{BasicValue, BasicValueEnum}, IntPredicate};

use kurai_binop::bin_op::BinOp;
use kurai_expr::expr::Expr;
use kurai_types::value::Value;
use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn lower_expr_to_llvm(&mut self, expr: &Expr, in_condition: bool) -> Option<BasicValueEnum<'ctx>> {
        #[cfg(debug_assertions)]
        {
            println!("Lowering expr: {:?}", expr);
        }
        match expr {
            Expr::Literal(value) => match value {
                Value::Int(v) => {
                    Some(self.context.i64_type().const_int(*v as u64, true).into())
                }
                Value::Bool(b) => {
                    Some(self.context.bool_type().const_int(*b as u64, false).into())
                }
                // Value::Str(s) => {
                //     let global_str = self.builder.build_global_string_ptr(s, &format!("str_{}_{}", 1, 1));
                //     Some(global_str.unwrap().as_basic_value_enum())
                // }
                Value::Str(s) => {
                    let id = format!("str_{}", self.string_counter);
                    self.string_counter += 1;

                    // @str_{} = constant [3 x i8] c"hi\00"
                    // let llvm_string_type = self.context.i8_type().array_type((s.len() + 1) as u32);
                    // let global_str = self.module.lock().unwrap().add_global(llvm_string_type, None, &id);

                    let cstr = format!("{}\0", s);
                    let str_bytes = cstr.as_bytes();
                    let str_len = str_bytes.len();

                    let str_type = self.context.i8_type().array_type(str_len as u32);

                    let global = self.module.lock().unwrap().add_global(str_type, None, &id);
                    global.set_initializer(&self.context.const_string(str_bytes, false));
                    global.set_constant(true);
                    global.set_linkage(Linkage::Private);

                    // GET DA POINTER TO DA START OF STRING (GEP trick)
                    let ptr = unsafe {
                        self.builder.build_gep(
                            str_type,
                            global.as_pointer_value(),
                            &[
                                self.context.i32_type().const_zero(),
                                self.context.i32_type().const_zero()
                            ],
                            format!("str_{}_ptr", self.string_counter).as_str(),
                        ).unwrap()
                    };

                    Some(ptr.as_basic_value_enum())
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
