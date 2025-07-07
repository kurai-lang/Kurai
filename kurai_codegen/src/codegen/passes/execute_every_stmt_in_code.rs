
use colored::Colorize;
use inkwell::{basic_block::{self, BasicBlock}, module::Linkage, types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum}, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum}, AddressSpace};
use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_token::token::token::Token;
use kurai_types::{typ::Type, value::Value};

use crate::{codegen::{passes::utils::basic_value_enum_to_string, CodeGen, FunctionInfo, VariableInfo}, kurai_panic, print_error, print_hint};
use kurai_ast::stmt::Stmt;
use kurai_ast::expr::Expr;

impl<'ctx> CodeGen<'ctx> {
    pub fn execute_every_stmt_in_code(
        &mut self,
        parsed_stmt: Vec<Stmt>, 
        discovered_modules: &mut Vec<String>, 
        parsers: &GroupedParsers,
        scope: &mut Scope,
        jump_from: Option<BasicBlock>,
    ) {
        for stmt in parsed_stmt {
            match &stmt {
                Stmt::VarDecl { name, typ, value } => {
                    let parsed_type = match typ {
                        Some(t) => Type::from_str(t).unwrap_or_else(|| panic!("Invalid type {:?}", typ)),
                        None => {
                            match value {
                                Some(Expr::Literal(Value::Int(_))) => Type::I64,
                                Some(Expr::Literal(Value::Bool(_))) => Type::Bool,
                                Some(Expr::FnCall { name, .. }) => {
                                    let fn_info = self.functions.get(name).expect("Function not found");
                                    fn_info.ret_type.clone()
                                }
                                _ => panic!("Can't infer type of expr: {:?}", value),
                            }
                        }
                    };

                    let llvm_type = parsed_type.to_llvm_type(self.context) 
                        .unwrap_or_else(|| {
                            eprintln!("{}: falling back to i64 for unsupported type {:?}", "warn".yellow().bold(), parsed_type);
                            self.context.i64_type().as_basic_type_enum()
                        });

                    let alloca = self.builder.build_alloca(
                            llvm_type,
                            name
                        )
                        .unwrap();

                    if let Some(expr) = value {
                        let (val, _) = self.lower_expr_to_llvm(
                            expr,
                            Some(&parsed_type),
                            discovered_modules,
                            parsers,
                            scope,
                            None,
                        ).unwrap_or_else(|| {
                            eprintln!(
                                "warn: failed to lower expr for '{}': falling back to default value",
                                name
                            );

                            let fallback_val = parsed_type.to_llvm_value(self.context);
                            (fallback_val, parsed_type.clone())
                        });

                        self.builder.build_store(alloca, val).unwrap();
                    }

                    let variable_info = VariableInfo {
                        ptr_value: alloca,
                        var_type: parsed_type,
                    };

                    self.variables.insert(
                        name.to_string(),
                        variable_info,
                    );
                }
                Stmt::Assign { name, value } => {
                    #[cfg(debug_assertions)]
                    { println!("Assign value AST: {:?}", value); }
                    let var_ptr = match self.variables.get(name.as_str()) {
                        Some(ptr) => ptr.ptr_value,
                        None => {
                            panic!("{}: variable {} not found", "error".red().bold(), name);
                        }
                    };

                    let (llvm_value, _) = self.lower_expr_to_llvm(
                        value,
                        None,
                        discovered_modules,
                        parsers,
                        scope,
                        None
                    ).unwrap_or_else(||
                        panic!("{}: tried to lower an invalid expression `{:?}` into LLVM IR",
                        "internal error".red().bold(), value));

                    self.builder.build_store(var_ptr, llvm_value).unwrap();
                }
                Stmt::FnCall { name, args } => {
                    // match name.as_str() {
                    //     "printf" => {
                    //         self.import_printf().unwrap();
                    //         self.printf(&args).unwrap();
                    //     }
                    //     _ => {
                    //         let function = if name.contains("::") {
                    //             self.get_or_compile_function(
                    //                 name.as_str(),
                    //                 discovered_modules,
                    //                 parsers,
                    //                 scope
                    //             )
                    //         } else {
                    //             self.module.lock().unwrap().get_function(&name)
                    //         };
                    //
                    //         if let Some(function) = function {
                    //             let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
                    //             for arg in args {
                    //                 let value = arg.value.as_ref().unwrap_or_else(||
                    //                     panic!("{}: Failed to compile arguments for function {}",
                    //                         "error".red().bold(),
                    //                         name.bold()));
                    //
                    //                 self.lower_expr_to_llvm(value, None, parsers, scope)
                    //                     .map(|expr| compiled_args.push(expr.into()));
                    //             }
                    //             self.builder.build_call(function, &compiled_args, &name).unwrap();
                    //         } else {
                    //             println!("{} {}", "Couldnt find function named:".red(), name.red());
                    //         }
                    //     }
                    // }
                    #[cfg(debug_assertions)]
                    println!("broken lol (nah i just commented legacy code)");
                }
                Stmt::FnDecl { name, args, body, attributes, ret_type, is_extern } => {
                    // Map the argument types to LLVM types 
                    // remember, we need to speak LLVM IR language, not rust!
                    #[cfg(debug_assertions)]
                    {
                        println!("converting args to llvm args types");
                    }
                    let arg_types: Vec<BasicMetadataTypeEnum> = args.iter().map(|arg| {
                        match arg.typ {
                            Type::I8 | Type::U8 => self.context.i8_type().into(),
                            Type::I16 | Type::U16 => self.context.i16_type().into(),
                            Type::I32 | Type::U32 => self.context.i32_type().into(),
                            Type::I64 | Type::U64 => self.context.i64_type().into(),
                            Type::I128 | Type::U128 => self.context.i128_type().into(),

                            Type::F16 => self.context.f16_type().into(),
                            Type::F32 => self.context.f32_type().into(),
                            Type::F64 => self.context.f64_type().into(),
                            Type::F128 => self.context.f128_type().into(),

                            Type::Bool => self.context.bool_type().into(),

                            Type::Str => self.context.ptr_type(AddressSpace::default()).into(),
                            Type::Ptr(ref inner) => {
                                let inner_ty = inner.to_llvm_type(self.context).unwrap();
                                inner_ty.ptr_type(AddressSpace::default()).into()
                            }
                            _ => panic!("Unknown type: {:?}", arg.typ),
                            }
                        }).collect();

                    #[cfg(debug_assertions)]
                    {
                        println!("done");
                    }

                    #[cfg(debug_assertions)]
                    {
                        println!("Module: {:?}", self.module.lock().unwrap());
                        println!("creating function named: {}", &name);
                        println!("name={}, is_extern={}", name, is_extern);
                    }

                    let fn_type = if *ret_type == Type::Void { 
                        self.context.void_type().fn_type(&arg_types, false)
                    } else {
                        let llvm_ret_type = ret_type.to_llvm_type(self.context)
                            .unwrap_or_else(|| 
                                panic!("{}: failed to lower return type to LLVM IR type", "internal error".red().bold()));

                        #[cfg(debug_assertions)] { println!("{:?}", llvm_ret_type) }
                        // let fn_type = self.context.i32_type().fn_type(&arg_types, false);
                        match llvm_ret_type {
                            BasicTypeEnum::IntType(int_type) => int_type.fn_type(&arg_types, false),
                            BasicTypeEnum::FloatType(float_type) => float_type.fn_type(&arg_types, false),
                            BasicTypeEnum::PointerType(ptr_type) => ptr_type.fn_type(&arg_types, false),
                            _ => panic!("unsupported return type `{:?}` in fn_type gen", llvm_ret_type),
                        }
                    };

                    println!("{}: parsed fn: {}, is_extern: {}", "debug".cyan().bold(), name, is_extern);
                    let function = self.module.lock().unwrap().add_function(
                        name,
                        fn_type,
                        Some(Linkage::External));

                    let function_info = FunctionInfo {
                        ret_type: ret_type.clone(),
                        args: args.to_vec(),
                        is_extern: *is_extern,
                    };
                    self.functions.insert(name.to_string(), function_info);

                    if *is_extern {
                        #[cfg(debug_assertions)]
                        println!("{}: skipping codegen for extern fn", "debug".cyan().bold());
                        continue;
                    }

                    // everything below? non-extern functions
                    let entry = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(entry);

                    self.current_fn_ret_type = ret_type.clone();

                    self.attr_registry.register_all(Some(ret_type), discovered_modules, parsers);
                    self.load_attributes(attributes, &stmt);

                    #[cfg(debug_assertions)] {
                        println!("done");
                        println!("parsing the function's body");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let llvm_arg = function.get_nth_param(i as u32).unwrap();//.into_pointer_value();
                        let pointee_type = llvm_arg.get_type();
                        let var_type = Type::from_llvm_type(pointee_type).unwrap_or(Type::Unknown);

                        println!("{:?}", pointee_type);
                        println!("{:?}", var_type);
                        println!("{:?}", llvm_arg.get_type());

                        let alloca = self.builder.build_alloca(
                            llvm_arg.get_type(),
                            &arg.name,
                        ).unwrap();

                        self.builder.build_store(alloca, llvm_arg).unwrap();

                        let var_info = VariableInfo {
                            ptr_value: alloca,
                            var_type,
                        };
                        self.variables.insert(arg.name.clone(), var_info);
                    }

                    self.execute_every_stmt_in_code(
                        body.to_vec(),
                        discovered_modules,
                        parsers,
                        scope,
                        Some(entry)
                    );

                    #[cfg(debug_assertions)] { println!("self.current_fn_ret_type = {:?}", self.current_fn_ret_type); }
                    if *ret_type == Type::Void {
                        #[cfg(debug_assertions)] {
                            println!("Auto-inserting `ret void` for void-returning fn: {}", name);
                        }

                        self.builder.build_return(None).unwrap();
                    }
                }
                Stmt::Import { path, nickname, is_glob} => {
                    let key = path.join("/");
                    // let modname = nickname.unwrap_or_else(|| path.last().unwrap().clone());
                    let modname = path[0].clone();

                    if self.loaded_modules.contains_key(&modname) {
                        eprintln!("Module `{}` already loaded", modname);
                        return;
                    }

                    let path_str = format!("{}.kurai", key);
                    let code = std::fs::read_to_string(&path_str)
                        .unwrap_or_else(|_| panic!("Failed to load module {}", path_str));
                    let (tokens, _) = Token::tokenize(&code);

                    let mut pos = 0;
                    let mut stmts = Vec::new();

                    while pos < tokens.len() {
                        match parsers.import_parser.parse_imported_file(
                            &tokens,
                            &mut pos,
                            discovered_modules,
                            parsers,
                            scope,
                            self.src
                        ) {
                            Ok(stmt) => stmts.push(stmt),
                            Err(e) => panic!("Failed to parse stmt at pos: {}\nError: {}", pos, e)
                        }
                    }

                    self.loaded_modules.insert(modname.clone(), stmts.clone());

                    if *is_glob {
                        self.execute_every_stmt_in_code(
                            stmts,
                            discovered_modules,
                            parsers,
                            scope,
                            None
                        );
                    } else {
                        #[cfg(debug_assertions)]
                        {
                            println!("{}: Not global import. Just a testing", "testing".cyan().bold());
                        }

                        self.generate_code(
                            stmts,
                            vec![], 
                            discovered_modules, 
                            parsers,
                            scope
                        );
                    }

                    // NOTE: Later
                    // if let Some(nick) = nickname {
                    //     self.imports.insert(nick.clone(), name.clone()); // Make sure `imports` is in your struct
                    // } else {
                    //     self.imports.insert(name.clone(), name.clone());
                    // }
                }
                Stmt::Loop { body } => {
                    let function = self.builder.get_insert_block()
                        .expect("Not inside a block")
                        .get_parent()
                        .expect("Block has no parent function");

                    let loop_bb = self.context.append_basic_block(function, format!("loop_{}", self.loop_counter).as_str());
                    let after_bb = self.context.append_basic_block(function, format!("after_loop_{}", self.loop_counter).as_str());
                    self.loop_counter += 1;

                    // memory
                    self.loop_exit_stack.push(after_bb);

                    // starts the loop
                    self.builder.build_unconditional_branch(loop_bb).unwrap();
                    self.builder.position_at_end(loop_bb);

                    self.execute_every_stmt_in_code(
                        body.to_vec(),
                        discovered_modules, 
                        parsers,
                        scope,
                        None
                    );

                    if self.builder.get_insert_block()
                        .map(|bb| bb.get_terminator().is_none())
                        .unwrap_or(true)
                    {
                        self.builder.build_unconditional_branch(loop_bb).unwrap();
                    }
                    self.loop_exit_stack.pop();

                    self.builder.position_at_end(after_bb);
                }
                Stmt::Break => {
                    let target = self.loop_exit_stack.last().ok_or("Break outside of loop").unwrap();
                    self.builder.build_unconditional_branch(*target).unwrap();

                    // safety net: prevent building more instructions in the same block
                    // let unreachable_block = self.context.append_basic_block(function, "unreachable");
                    // self.builder.position_at_end(unreachable_block);
                }
                Stmt::Expr(expr) => {
                    if let Some(val) = self.lower_expr_to_llvm(
                        expr,
                        None,
                        discovered_modules,
                        parsers,
                        scope,
                        jump_from
                    ) {
                        #[cfg(debug_assertions)]
                        println!("Expression result (ignored): {:?}", val);
                    }
                }
                Stmt::Block(stmts) => {
                    self.execute_every_stmt_in_code(
                        stmts.to_vec(),
                        discovered_modules, 
                        parsers,
                        scope,
                        None
                    );
                }
                Stmt::Return(expr) => {
                    let ret_type = self.current_fn_ret_type.clone();
                    #[cfg(debug_assertions)] {
                        println!("{}: Current function return type is {:?}", "debug".cyan().bold(), ret_type);
                        println!("{}: Current expression is {:?}", "debug".cyan().bold(), expr);
                    }
                    let (raw_val, _) = self.lower_expr_to_llvm(expr.as_ref().unwrap(), Some(&ret_type), discovered_modules,parsers, scope, None).unwrap();

                    let final_val = match ret_type {
                        Type::I32 => {
                            let val = match raw_val {
                                BasicValueEnum::IntValue(v) => v,
                                other => { 
                                    let variant = basic_value_enum_to_string(&other);
                                    print_error!(
                                        "Expected an `IntValue` for return type `i32`, but got `{}` instead.",
                                        variant,
                                    );

                                    print_hint!("Maybe try checking your return statement, and your function return type");
                                    kurai_panic!();
                                }
                            };
                            let val = if val.get_type() != self.context.i32_type() {
                                self.builder.build_int_cast(
                                    val, self.context.i32_type(),
                                    "ret_cast"
                                ).unwrap()
                            } else { val };
                            val.as_basic_value_enum()
                        }
                        Type::I64 => {
                            let val = raw_val.into_int_value();
                            let val = if val.get_type() != self.context.i64_type() {
                                self.builder.build_int_cast(
                                    val, self.context.i64_type(),
                                    "ret_cast"
                                ).unwrap()
                            } else { val };
                            val.as_basic_value_enum()
                        }
                        Type::F32 => {
                            let val = raw_val.into_float_value();
                            let val = if val.get_type() != self.context.f32_type() {
                                self.builder.build_float_cast(
                                    val, self.context.f32_type(),
                                    "ret_cast"
                                ).unwrap()
                            } else { val };
                            val.as_basic_value_enum()
                        }
                        Type::F64 => {
                            let val = raw_val.into_float_value();
                            let val = if val.get_type() != self.context.f64_type() {
                                self.builder.build_float_cast(
                                    val, self.context.f64_type(),
                                    "ret_cast"
                                ).unwrap()
                            } else { val };
                            val.as_basic_value_enum()
                        }
                        Type::Void => {
                            panic!("Tried to return a value from a function that returns void");
                        }
                        _ => {
                            panic!("Unsupported return type: {:?}", ret_type);
                        }
                    };

                    self.builder.build_return(Some(&final_val)).unwrap();
                }
            }

            if let Stmt::FnDecl { ref name, is_extern, .. } = stmt {
            }
        }
    }
}
