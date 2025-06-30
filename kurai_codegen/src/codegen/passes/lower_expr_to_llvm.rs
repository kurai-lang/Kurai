use colored::Colorize;
use inkwell::{module::Linkage, values::{BasicValue, BasicValueEnum}, IntPredicate};

use kurai_binop::bin_op::BinOp;
use kurai_expr::expr::Expr;
use kurai_types::{typ::Type, value::Value};
use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn lower_expr_to_llvm(&mut self, expr: &Expr, expected_type: Option<&Type>) -> Option<BasicValueEnum<'ctx>> {
        #[cfg(debug_assertions)]
        {
            println!("Lowering expr: {:?}", expr);
        }
        match expr {
            Expr::Literal(value) => match value {
                Value::Int(v) => {
                    match expected_type {
                        Some(Type::I8) => Some(self.context.i8_type().const_int(*v as u64, true).into()),
                        Some(Type::I16) => Some(self.context.i16_type().const_int(*v as u64, true).into()),
                        Some(Type::I32) => Some(self.context.i32_type().const_int(*v as u64, true).into()),
                        Some(Type::I64) => Some(self.context.i64_type().const_int(*v as u64, true).into()),
                        Some(Type::I128) => Some(self.context.i128_type().const_int(*v as u64, true).into()),
                        _ => Some(self.context.i64_type().const_int(*v as u64, true).into()), // <- DEFAULT TYPE
                        // _ => panic!("invalid expected type for int literal")
                    }
                }
                Value::Float(v) => {
                    match expected_type {
                        Some(Type::F16) => Some(self.context.f16_type().const_float(*v).into()),
                        Some(Type::F32) => Some(self.context.f32_type().const_float(*v).into()),
                        Some(Type::F64) => Some(self.context.f64_type().const_float(*v).into()),
                        Some(Type::F128) => Some(self.context.f128_type().const_float(*v).into()),
                        _ => Some(self.context.f64_type().const_float(*v).into()), // <- DEFAULT TYPE
                    }
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
            }
            Expr::Id(name) => {
                if let Some(ptr) = self.variables.get(name) {
                    let loaded = self.builder.build_load(
                        self.context.i64_type(),
                        ptr.ptr_value,
                        &format!("load_{}", name)
                    );
                    Some(loaded.unwrap())
                } else {
                    println!("Variable {} not found!", name);
                    None
                }
            }
            Expr::Binary { op, left, right } => {
                #[cfg(debug_assertions)]
                { println!("{:?}", op);
                println!("{} Entering Expr::Binary case", "[lower_expr_to_llvm()]".green().bold()); }
                let left_val = self.lower_expr_to_llvm(left, Some(&Type::I32))?;
                let right_val = self.lower_expr_to_llvm(right, Some(&Type::I32))?;

                #[cfg(debug_assertions)]
                { println!("{} left_val:{:?}\nright_val:{:?}", "[lower_expr_to_llvm()]".green().bold(), left_val, right_val); }

                match op {
                    BinOp::Lt | BinOp::Le | BinOp::Eq | BinOp::Ge | BinOp::Gt | BinOp::Ne => {
                        let predicate = match op {
                            BinOp::Lt => IntPredicate::SLT,
                            BinOp::Le => IntPredicate::SLE,
                            BinOp::Eq => IntPredicate::EQ,
                            BinOp::Ge => IntPredicate::SGE,
                            BinOp::Gt => IntPredicate::SGT,
                            BinOp::Ne => IntPredicate::NE,
                            _ => {
                                panic!("Operator {:?} not supported", op);
                                // return None;
                            }
                        };

                        Some(self.builder.build_int_compare(
                            predicate,
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "cmp"
                        ).unwrap().as_basic_value_enum())
                    }

                    // Arithmetic'ing time
                    BinOp::Add => {
                        let sum = self.builder.build_int_add(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "addtmp",
                        ).unwrap();

                        Some(sum.as_basic_value_enum())
                    }
                    BinOp::Sub => {
                        let diff = self.builder.build_int_sub(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "subtmp",
                        ).unwrap();

                        Some(diff.as_basic_value_enum())
                    }
                    BinOp::Mul => {
                        let product = self.builder.build_int_mul(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "multmp",
                        ).unwrap();

                        Some(product.as_basic_value_enum())
                    }
                    BinOp::Div => {
                        let div = self.builder.build_int_signed_div(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "divtmp",
                        ).unwrap();

                        Some(div.as_basic_value_enum())
                    }
                    _ => panic!("Operator {:?} not supported yet", op),
                }
            }
            Expr::FnCall { name, args } => todo!(),
        }
    }
}
