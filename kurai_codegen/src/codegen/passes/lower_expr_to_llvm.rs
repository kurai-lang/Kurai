use colored::Colorize;
use inkwell::{module::Linkage, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum}, IntPredicate};

use kurai_ast::expr::Expr;
use kurai_binop::bin_op::BinOp;
use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_types::{typ::Type, value::Value};
use crate::{codegen::CodeGen, kurai_panic, print_error};

impl<'ctx> CodeGen<'ctx> {
    pub fn lower_expr_to_llvm(&mut self, expr: &Expr, expected_type: Option<&Type>, discovered_modules: &mut Vec<String>, parsers: &GroupedParsers, scope: &mut Scope) -> Option<(BasicValueEnum<'ctx>, Type)> {
        #[cfg(debug_assertions)]
        {
            println!("Lowering expr: {:?}", expr);
        }
        match expr {
            Expr::Literal(value) => match value {
                Value::Int(v) => {
                    match expected_type {
                        Some(Type::I8) => Some((
                            self.context.i8_type().const_int(*v as u64, true).into(),
                            Type::I8)),
                        Some(Type::I16) => Some((
                            self.context.i16_type().const_int(*v as u64, true).into(),
                            Type::I16)),
                        Some(Type::I32) => Some((
                            self.context.i32_type().const_int(*v as u64, true).into(),
                            Type::I32)),
                        Some(Type::I64) => Some((
                            self.context.i64_type().const_int(*v as u64, true).into(),
                            Type::I64)),
                        Some(Type::I128) => Some((
                            self.context.i128_type().const_int(*v as u64, true).into(),
                            Type::I128)),
                        _ => Some((
                            self.context.i64_type().const_int(*v as u64, true).into(),
                            Type::I64)), // <- DEFAULT TYPE
                        // _ => panic!("invalid expected type for int literal")
                    }
                }
                Value::Float(v) => {
                    match expected_type {
                        Some(Type::F16) => Some((
                            self.context.f16_type().const_float(*v).into(),
                            Type::F16)),
                        Some(Type::F32) => Some((
                            self.context.f32_type().const_float(*v).into(),
                            Type::F32)),
                        Some(Type::F64) => Some((
                            self.context.f64_type().const_float(*v).into(),
                            Type::F64)),
                        Some(Type::F128) => Some((
                            self.context.f128_type().const_float(*v).into(),
                            Type::F128)),
                        _ => Some((
                            self.context.f64_type().const_float(*v).into(),
                            Type::F64)), // <- DEFAULT TYPE
                    }
                }
                Value::Bool(b) => {
                    Some((
                        self.context.bool_type().const_int(*b as u64, false).into(),
                        Type::Bool))
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

                    Some((ptr.as_basic_value_enum(), Type::Str))
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
                    Some((loaded.unwrap(), ptr.var_type.clone()))
                } else {
                    println!("Variable {} not found!", name);
                    None
                }
            }
            Expr::Binary { op, left, right } => {
                #[cfg(debug_assertions)]
                { println!("{:?}", op);
                println!("{} Entering Expr::Binary case", "[lower_expr_to_llvm()]".green().bold()); }
                let left_val = self.lower_expr_to_llvm(left, Some(&Type::I32), discovered_modules, parsers, scope)?;
                let right_val = self.lower_expr_to_llvm(right, Some(&Type::I32), discovered_modules, parsers, scope)?;

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

                        Some((self.builder.build_int_compare(
                            predicate,
                            left_val.0.into_int_value(),
                            right_val.0.into_int_value(),
                            "cmp"
                        ).unwrap().as_basic_value_enum(), Type::Bool))
                    }

                    // Arithmetic'ing time
                    BinOp::Add => {
                        let sum = self.builder.build_int_add(
                            left_val.0.into_int_value(),
                            right_val.0.into_int_value(),
                            "addtmp",
                        ).unwrap();

                        Some((sum.as_basic_value_enum(), Type::Bool))
                    }
                    BinOp::Sub => {
                        let diff = self.builder.build_int_sub(
                            left_val.0.into_int_value(),
                            right_val.0.into_int_value(),
                            "subtmp",
                        ).unwrap();

                        Some((diff.as_basic_value_enum(), Type::Bool))
                    }
                    BinOp::Mul => {
                        let product = self.builder.build_int_mul(
                            left_val.0.into_int_value(),
                            right_val.0.into_int_value(),
                            "multmp",
                        ).unwrap();

                        Some((product.as_basic_value_enum(), Type::Bool))
                    }
                    BinOp::Div => {
                        let div = self.builder.build_int_signed_div(
                            left_val.0.into_int_value(),
                            right_val.0.into_int_value(),
                            "divtmp",
                        ).unwrap();

                        Some((div.as_basic_value_enum(), Type::Bool))
                    }
                    _ => panic!("Operator {:?} not supported yet", op),
                }
            }
            Expr::FnCall { name, args } => {
                match name.as_str() {
                    "printf" => {
                        self.import_printf().unwrap();
                        self.printf(&args, expected_type, discovered_modules, parsers, scope).unwrap();
                        // return Some((self.context.i32_type().const_zero().into(), Type::Void));
                        None
                    }
                    _ => {
                        let function = if name.contains("::") {
                            self.get_or_compile_function(
                                name.as_str(),
                                discovered_modules,
                                parsers,
                                scope
                            )
                        } else {
                            self.module.lock().unwrap().get_function(&name)
                        };

                        if let Some(function) = function {
                            let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
                            for expr in args {
                                let (compiled, _) = self.lower_expr_to_llvm(expr, expected_type, discovered_modules, parsers, scope)
                                    .unwrap_or_else(|| { 
                                        print_error!("Failed to compile argument in call to function: {}", name.bold());
                                        kurai_panic!();
                                    });
                                compiled_args.push(compiled.into());
                            }
                            let call = self.builder.build_call(function, &compiled_args, &name).unwrap();

                            Some((call.try_as_basic_value().unwrap_left().as_basic_value_enum(), Type::Void))
                        } else {
                            print_error!("Couldnt find function named {}", name.bold());
                            kurai_panic!();
                        }
                    }
                }
            }
            Expr::If { branches, else_body } => {
                // NOTE: FOR REFERENCE ONLY, THIS IS LEGACY CODE
                // Stmt::If { branches, else_body } => {
                //     let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                //     // let merge_block = self.context.append_basic_block(current_function, "merge");
                //
                //     let mut prev_block = self.builder.get_insert_block().unwrap();
                //
                //     // process each branch hehe
                //     for (i, branch) in branches.iter().enumerate() {
                //         if let Some(block) = Some(prev_block) {
                //             self.builder.position_at_end(block);
                //         }
                //
                //         // imagine if!! or else u die
                //          self.build_conditional_branch(
                //             current_function,
                //             &branch.condition,
                //             &branch.body.clone(),
                //             &else_body,
                //             discovered_modules,
                //             &i.to_string(),
                //             parsers,
                //             scope
                //         );
                //     }
                //     // Position builder at merge block for continuation
                //     // self.builder.position_at_end(merge_block);
                // }
                #[cfg(debug_assertions)]
                println!("Its if'ing time");

                let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();

                let merge_block = self.context.append_basic_block(current_function, "merge");
                let mut result_phi = None;

                let mut prev_block = self.builder.get_insert_block().unwrap();

                let mut branch_blocks = Vec::new();
                let mut branch_values = Vec::new();

                for (i, branch) in branches.iter().enumerate() {
                    let then_block = self.context.append_basic_block(current_function, &format!("then_{}", i));
                    branch_blocks.push(then_block);

                    self.builder.position_at_end(prev_block);

                    // evaluate condition
                    let cond_val = self.lower_expr_to_llvm(
                        &branch.condition,
                        Some(&Type::Bool),
                        discovered_modules,
                        parsers,
                        scope
                    ).unwrap();

                    self.builder.build_conditional_branch(
                        cond_val.0.into_int_value(), 
                        then_block, 
                        merge_block, // Supposed to be else_block, but its temporary here
                    ).unwrap();

                    self.builder.position_at_end(then_block);

                    let mut last_expr_value = None;

                    for expr in &branch.body {
                        last_expr_value = self.lower_expr_to_llvm(
                            expr,
                            expected_type,
                            discovered_modules,
                            parsers,
                            scope
                        );
                    }

                    self.builder.build_unconditional_branch(merge_block).unwrap(); // jumps
                                                                                             // to
                                                                                             // merge
                                                                                             // lol
                    if let Some(val) = last_expr_value {
                        branch_values.push(val);
                    }

                    prev_block = then_block;
                }

                if let Some(else_exprs) = else_body {
                    let else_block = self.context.append_basic_block(current_function, "else");

                    self.builder.position_at_end(prev_block);
                    self.builder.build_unconditional_branch(else_block).unwrap();

                    self.builder.position_at_end(else_block);

                    let mut last_expr_value = None;

                    for expr in else_exprs {
                        last_expr_value = self.lower_expr_to_llvm(
                            expr,
                            expected_type, 
                            discovered_modules,
                            parsers,
                            scope
                        );
                    }

                    self.builder.build_unconditional_branch(merge_block).unwrap();

                    if let Some(val) = last_expr_value {
                        branch_values.push(val);
                    }

                    prev_block = else_block;
                }

                self.builder.position_at_end(merge_block);

                if !branch_values.is_empty() {
                    let phi = self.builder.build_phi(branch_values[0].0.get_type(), "if_result").unwrap();

                    for (i, val) in branch_values.iter().enumerate() {
                        phi.add_incoming(&[(&val.0, branch_blocks[i])]);
                    }

                    result_phi = Some((phi.as_basic_value(), branch_values[0].1.clone()));
                }

                result_phi
            }
            Expr::Block { stmts, final_expr } => {
                self.execute_every_stmt_in_code(stmts.to_vec(), discovered_modules, parsers, scope);

                if let Some(expr) = final_expr {
                    self.lower_expr_to_llvm(expr, expected_type, discovered_modules, parsers, scope)
                } else {
                    // It returns nothing here lol
                    // it found no expr here remember?
                    let void_val = self.context.i32_type().const_zero();
                    Some((void_val.as_basic_value_enum(), Type::Void))
                }
            },
        }
    }
}
