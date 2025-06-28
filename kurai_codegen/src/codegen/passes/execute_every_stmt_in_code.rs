use std::collections::HashMap;

use colored::Colorize;
use inkwell::{types::{BasicMetadataTypeEnum, BasicTypeEnum}, values::BasicMetadataValueEnum, AddressSpace};
use kurai_attr::attribute::Attribute;
use kurai_core::scope::Scope;
use kurai_expr::expr::Expr;
use kurai_parser::GroupedParsers;
use kurai_token::token::token::Token;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::{typ::Type, value::Value};

use crate::{codegen::{CodeGen, VariableInfo}, registry::registry::{AttributeHandler, AttributeRegistry}};
use kurai_stmt::stmt::Stmt;

impl<'ctx> CodeGen<'ctx> {
    pub fn execute_every_stmt_in_code(
        &mut self,
        parsed_stmt: Vec<Stmt>, 
        discovered_modules: &mut Vec<String>, 
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) {
        for stmt in parsed_stmt {
            match &stmt {
                Stmt::VarDecl { name, typ, value } => {
                    let i64_type = self.context.i64_type();
                    let alloca = self.builder.build_alloca(i64_type, &name).unwrap();

                    if let Some(Expr::Literal(Value::Int(v))) = value {
                        let init_val = i64_type.const_int(*v as u64, true);
                        self.builder.build_store(alloca, init_val).unwrap();
                        let v_pointer_val = alloca;
                        let var_info = VariableInfo {
                            ptr_value: v_pointer_val,
                            var_type: Type::I64,
                        };

                        self.variables.insert(name.to_string(), var_info);
                    }
                }
                Stmt::Assign { name, value } => {
                    #[cfg(debug_assertions)]
                    { println!("Assign value AST: {:?}", value); }
                    let var_ptr = match self.variables.get(name.as_str()) {
                        Some(ptr) => ptr.ptr_value,
                        None => {
                            panic!("{} Variable {} not found", "[ERROR]".red().bold(), name);
                        }
                    };

                    // now do mutable stuff after immutable borrow is over
                    let llvm_value = self.lower_expr_to_llvm(value).unwrap();
                    self.builder.build_store(var_ptr, llvm_value).unwrap();
                }
                Stmt::FnCall { name, args } => {
                    match name.as_str() {
                        "printf" => {
                            self.import_printf().unwrap();
                            self.printf(&args).unwrap();
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
                                for arg in args {
                                    let value = arg.value.as_ref().unwrap_or_else(||
                                        panic!("{}: Failed to compile arguments for function {}",
                                            "error".red().bold(),
                                            name.bold()));

                                    self.lower_expr_to_llvm(value)
                                        .map(|expr| compiled_args.push(expr.into()));
                                }
                                self.builder.build_call(function, &compiled_args, &name).unwrap();
                            } else {
                                println!("{} {}", "Couldnt find function named:".red(), name.red());
                            }
                        }
                    }
                }
                Stmt::FnDecl { name, args, body, attributes, ret_type } => {
                    // Map the argument types to LLVM types 
                    // remember, we need to speak LLVM IR language, not rust!
                    #[cfg(debug_assertions)]
                    {
                        println!("converting args to llvm args types");
                    }
                    let arg_types: Vec<BasicMetadataTypeEnum> = args.iter().map(|arg| {
                        match arg.typ {
                            Type::I32 => self.context.i32_type().into(),
                            Type::F32 => self.context.f32_type().into(),
                            Type::Bool => self.context.bool_type().into(),
                            Type::Str => self.context.ptr_type(AddressSpace::default()).into(),
                            _ => panic!("Unknown type: {:?}", arg.typ),
                            }
                        }).collect();

                    #[cfg(debug_assertions)]
                    {
                        println!("done");
                    }

                    {
                        #[cfg(debug_assertions)]
                        {
                            println!("Module: {:?}", self.module.lock().unwrap());
                            println!("creating function named: {}", &name);
                        }

                        let fn_type = if *ret_type == Type::Void { 
                            self.context.void_type().fn_type(&arg_types, false)
                        } else {
                            let llvm_ret_type = ret_type.to_llvm_type(self.context).unwrap();
                            #[cfg(debug_assertions)] { println!("{:?}", llvm_ret_type) }
                            // let fn_type = self.context.i32_type().fn_type(&arg_types, false);
                            match llvm_ret_type {
                                BasicTypeEnum::IntType(int_type) => int_type.fn_type(&arg_types, false),
                                BasicTypeEnum::FloatType(float_type) => float_type.fn_type(&arg_types, false),
                                BasicTypeEnum::PointerType(ptr_type) => ptr_type.fn_type(&arg_types, false),
                                _ => panic!("Unsupported return type in fn_type gen"),
                            }
                        };
                        let function = self.module.lock().unwrap().add_function(name, fn_type, None);
                        let basic_block = self.context.append_basic_block(function, "entry");
                        self.builder.position_at_end(basic_block);

                        self.attr_registry.register_all();
                        self.load_attributes(attributes, &stmt);

                        #[cfg(debug_assertions)]
                        {
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
                    }
                    self.execute_every_stmt_in_code(
                        body.to_vec(),
                        discovered_modules,
                        parsers, scope);

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
                    let tokens = Token::tokenize(&code);

                    let mut pos = 0;
                    let mut stmts = Vec::new();

                    while pos < tokens.len() {
                        match parsers.import_parser.parse_imported_file(
                            &tokens,
                            &mut pos,
                            discovered_modules,
                            parsers,
                            scope
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
                            scope
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
                Stmt::If { branches, else_body } => {
                    let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                    // let merge_block = self.context.append_basic_block(current_function, "merge");

                    let mut prev_block = self.builder.get_insert_block().unwrap();

                    // process each branch hehe
                    for (i, branch) in branches.iter().enumerate() {
                        if let Some(block) = Some(prev_block) {
                            self.builder.position_at_end(block);
                        }

                        // imagine if!! or else u die
                         self.build_conditional_branch(
                            current_function,
                            &branch.condition,
                            &branch.body.clone(),
                            &else_body,
                            discovered_modules,
                            &i.to_string(),
                            parsers,
                            scope
                        );
                    }
                    // Position builder at merge block for continuation
                    // self.builder.position_at_end(merge_block);
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
                        scope
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
                    self.lower_expr_to_llvm(expr);
                }
                Stmt::Block(stmts) => {
                    self.execute_every_stmt_in_code(
                        stmts.to_vec(),
                        discovered_modules, 
                        parsers,
                        scope
                    );
                }
                Stmt::Return(expr) => {
                    let expr = expr.as_ref().unwrap();
                    let ret_val = self.lower_expr_to_llvm(expr).unwrap();
                    self.builder.build_return(Some(&ret_val)).unwrap();
                }
            }
        }
    }
}
