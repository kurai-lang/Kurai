use colored::Colorize;
use inkwell::{basic_block::BasicBlock, module::Linkage, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum}, IntPredicate};

use kurai_ast::expr::Expr;
use kurai_binop::bin_op::BinOp;

use kurai_parser::parse::Parser;
use kurai_types::{typ::Type, value::Value};
use crate::{codegen::{passes::utils::{basic_type_enum_to_string, basic_value_enum_to_string}, CodeGen}, kurai_panic, print_error};

impl<'ctx> CodeGen<'ctx> {
    pub fn lower_expr_to_llvm(
        &mut self,
        expr: &Expr,
        expected_type: Option<&Type>,
        parser: &mut Parser,
        jump_from: Option<BasicBlock>
    ) -> Option<(BasicValueEnum<'ctx>, Type)> {
        #[cfg(debug_assertions)]
        println!("Lowering expr: {:?}", expr);

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
                let mut left_val = self.lower_expr_to_llvm(left, Some(&Type::I32), parser, None)?;
                let mut right_val = self.lower_expr_to_llvm(right, Some(&Type::I32), parser, None)?;

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

                        let l_debug = basic_value_enum_to_string(&left_val.0);
                        let r_debug = basic_value_enum_to_string(&right_val.0);
                        let l_type_debug = basic_type_enum_to_string(&left_val.0.get_type());
                        let r_type_debug = basic_type_enum_to_string(&right_val.0.get_type());

                        if left_val.0.get_type() != right_val.0.get_type() {
                            #[cfg(debug_assertions)]
                            println!("{}: {:?}'s bit width isn't equal to {:?}", "debug".cyan().bold(), l_debug, r_debug);

                            if left_val.0.get_type().into_int_type().get_bit_width() <
                            right_val.0.get_type().into_int_type().get_bit_width() {
                                #[cfg(debug_assertions)] {
                                    println!("{}: beginning conversion. {:?}: {:?} to {:?}: {:?}", "debug".cyan().bold(), 
                                        l_debug, l_type_debug,
                                        r_debug, r_type_debug);
                                }

                                left_val.0 = self.builder.build_int_cast(
                                    left_val.0.into_int_value(),
                                    right_val.0.get_type().into_int_type(),
                                    "cast_l")
                                    .unwrap().as_basic_value_enum();
                            } else {
                                #[cfg(debug_assertions)] {
                                    println!("{}: beginning conversion. {:?}: {:?} to {:?}: {:?}", "debug".cyan().bold(), 
                                        l_debug, l_type_debug,
                                        r_debug, r_type_debug);
                                }

                                right_val.0 = self.builder.build_int_cast(
                                    right_val.0.into_int_value(),
                                    left_val.0.get_type().into_int_type(),
                                    "cast_r")
                                    .unwrap().as_basic_value_enum();
                            }
                        }

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
                        self.printf(args, expected_type, parser).unwrap();
                        None
                    }
                    _ => {
                        let function = if name.contains("::") {
                            self.get_or_compile_function(
                                name.as_str(),
                                &mut parser.discovered_modules,
                                &mut parser.scope
                            )
                        } else {
                            self.module.lock().unwrap().get_function(&name)
                        };

                        if let Some(function) = function {
                            let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
                            for expr in args {
                                let (compiled, _) = self.lower_expr_to_llvm(expr, expected_type, parser, None)
                                    .unwrap_or_else(|| { 
                                        print_error!("Failed to compile argument in call to function: {}", name.bold());
                                        kurai_panic!();
                                    });
                                compiled_args.push(compiled.into());
                            }
                            let call = self.builder.build_call(function, &compiled_args, &name).unwrap();

                            let ret_val = match call.try_as_basic_value() {
                                inkwell::Either::Left(val) => Some(val),
                                inkwell::Either::Right(_inst) => None,
                            }?;
                            Some((ret_val, Type::Void))
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

                let mut branch_blocks = Vec::new();
                let mut branch_values = Vec::new();

                let else_block = else_body.as_ref().map(|_| {
                    self.context.append_basic_block(current_function, "else")
                });

                // tbh i dont even understand what is get_insert_block() lmfao
                // nayways this is for chaining condition branches
                let mut next_check_block = self.builder.get_insert_block().unwrap();
                let mut else_entry_block = None;
                // let mut last_check_block = None;

                for (i, branch) in branches.iter().enumerate() {
                    let then_block = self.context.append_basic_block(current_function, &format!("then_{}", i));
                    // branch_blocks.push(then_block);

                    let is_last_branch = i == branches.len() - 1;
                    let has_else = else_body.is_some();

                    let check_next_block = if !is_last_branch || has_else {
                        let check_block = self.context.append_basic_block(current_function, &format!("check_next_{}", i));
                        if is_last_branch && has_else {
                            // this is the last branch, we would have an else by that time
                            else_entry_block = Some(check_block);
                        }
                        check_block
                    } else {
                        // OLD CODE: merge_block
                        // compared to this old code, we aint jumping straight to merge_block.
                        // just make a last check
                        let fallback_block = self.context.append_basic_block(current_function, &format!("final_check_{}", i));
                        self.final_check_blocks.push(fallback_block);
                        fallback_block
                    };

                    if i == 0 {
                        if let Some(jump_from_block) = jump_from {
                            self.builder.position_at_end(jump_from_block);
                            self.builder.build_unconditional_branch(check_next_block).unwrap();
                        }
                    }

                    // position to current check block
                    // but we gotta check if the check_next_block isnt equal to merge_block or not
                    // this a guard to make sure random shit doesnt get thrown in merge_block
                    if !self.terminated_blocks.contains(&check_next_block)
                    && check_next_block != merge_block {
                        self.builder.position_at_end(check_next_block);

                        // evaluate condition
                        let cond_val = self.lower_expr_to_llvm(
                            &branch.condition,
                            Some(&Type::Bool),
                            parser,
                            None
                        ).unwrap();

                        if is_last_branch && has_else {
                            // last one gets else lol
                            self.builder.build_conditional_branch(
                                cond_val.0.into_int_value(), 
                                then_block, 
                                else_block.unwrap()
                            ).unwrap();

                            // this way, its guaranteed to not be overwriting blocks with multiple
                            // branches
                        } else {
                            self.builder.build_conditional_branch(
                                cond_val.0.into_int_value(),
                                then_block,
                            check_next_block
                            ).unwrap();
                        }

                        self.terminated_blocks.insert(check_next_block);
                    }

                    self.builder.position_at_end(then_block);

                    let mut last_expr_value = None;
                    for expr in &branch.body {
                        last_expr_value = self.lower_expr_to_llvm(
                            expr,
                            expected_type,
                            parser,
                            None
                        );
                    }

                    self.builder.build_unconditional_branch(merge_block).unwrap(); // jumps
                                                                                             // to
                                                                                             // merge
                                                                                             // lol
                    if let Some(val) = last_expr_value {
                        branch_blocks.push(then_block);
                        branch_values.push(val);
                    }

                    next_check_block = check_next_block;
                }

                if let (Some(else_exprs), Some(else_block)) = (else_body, else_block) {
                    if let Some(else_jump_from_block) = else_entry_block {
                        if !self.terminated_blocks.contains(&else_jump_from_block) {
                            self.builder.position_at_end(else_jump_from_block);
                            self.builder.build_unconditional_branch(else_block).unwrap();
                            self.terminated_blocks.insert(else_jump_from_block);
                        }
                    }

                    self.builder.position_at_end(else_block);

                    let mut last_expr_value = None;
                    for expr in else_exprs {
                        last_expr_value = self.lower_expr_to_llvm(
                            expr,
                            expected_type,
                            parser,
                            None
                        );
                    }

                    self.builder.build_unconditional_branch(merge_block).unwrap();

                    if let Some(val) = last_expr_value {
                        branch_blocks.push(else_block);
                        branch_values.push(val);
                    }
                } else {
                    // for when theres no else
                    for final_block in self.final_check_blocks.drain(..) {
                        self.builder.position_at_end(final_block);
                        self.builder.build_unconditional_branch(merge_block).unwrap();
                    }
                }

                self.builder.position_at_end(merge_block);

                if !branch_values.is_empty() {
                    let phi = self.builder.build_phi(branch_values[0].0.get_type(), "if_result").unwrap();

                    for (i, val) in branch_values.iter().enumerate() {
                        phi.add_incoming(&[(&val.0, branch_blocks[i])]);
                    }

                    Some((phi.as_basic_value(), branch_values[0].1.clone()))
                } else {
                    None
                }
            }
            Expr::Block { stmts, final_expr } => {
                self.execute_every_stmt_in_code(stmts.to_vec(), parser, None);

                if let Some(expr) = final_expr {
                    self.lower_expr_to_llvm(expr, expected_type, parser, None)
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
