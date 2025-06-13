use colored::Colorize;
use inkwell::{types::BasicMetadataTypeEnum, values::{BasicMetadataValueEnum, IntValue}, AddressSpace, IntPredicate};

use crate::codegen::CodeGen;
use kurai_parser_import_file::parse_imported_file::parse_imported_file;
use kurai_token::token::token::Token;
use kurai_types::value::Value;
use kurai_stmt::stmt::Stmt;

impl<'ctx> CodeGen<'ctx> {
    pub fn execute_every_stmt_in_code(&mut self, parsed_stmt: Vec<Stmt>, discovered_modules: &mut Vec<String>) {
        for stmt in parsed_stmt {
            match stmt {
                Stmt::VarDecl { name, typ, value } => {
                    let i64_type = self.context.i64_type();
                    let alloca = self.builder.build_alloca(i64_type, &name).unwrap();

                    if let Some(Value::Int(v)) = value {
                        let init_val = i64_type.const_int(v as u64, true);
                        self.builder.build_store(alloca, init_val).unwrap();
                        let v_pointer_val = alloca;

                        self.variables.insert(name.to_string(), v_pointer_val);
                    }
                }
                Stmt::Assign { name, value } => {
                    if let Some(var_ptr) = self.variables.get(&name) {
                        let llvm_value = self.lower_value_to_llvm(&value).unwrap();
                        self.builder.build_store(*var_ptr, llvm_value);
                    } else {
                        println!("Variable {} could not be found!", name);
                    }
                }
                Stmt::FnCall { name, args } => {
                    match name.as_str() {
                        "printf" => {
                            // FIXME: Yes
                            // stdlib::print::printf(&args, self).unwrap();
                        }
                        _ => {
                            let module = self.module.lock().unwrap();

                            println!("Module: {:?}", module);
                            let function = module.get_function(&name);

                            if let Some(function) = function {
                                let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
                                for arg in &args {
                                    if let Some(value) = &arg.value {
                                        if let Some(llvm_value) = self.lower_expr_to_llvm(&value, false) {
                                            compiled_args.push(llvm_value.into());
                                        }
                                    } else {
                                        println!("{} {}", "Failed to compile arguments for function".red(), name.red());
                                    }
                                }
                                self.builder.build_call(function, &compiled_args, &name).unwrap();
                            } else {
                                println!("{} {}", "Couldnt find function named:".red(), name.red());
                            }
                        }
                    }
                }
                Stmt::FnDecl { name, args, body } => {
                    // Map the argument types to LLVM types 
                    // remember, we need to speak LLVM IR language, not rust!
                    dbg!("converting args to llvm args types");
                    let arg_types: Vec<BasicMetadataTypeEnum> = args.iter().map(|arg| {
                        match arg.typ.to_string().as_str() {
                            "int" => self.context.i32_type().into(),
                            "float" => self.context.f32_type().into(),
                            "bool" => self.context.bool_type().into(),
                            "str" => self.context.i8_type().ptr_type(AddressSpace::default()).into(),
                            _ => panic!("Unknown type: {:?}", arg.typ),
                        }
                    }).collect();
                    dbg!("done");

                    {
                        let module = self.module.lock().unwrap();

                        if name == "main" && module.get_function("main").is_some() {
                            let fn_type = self.context.i32_type().fn_type(&arg_types, false);
                            let function = module.add_function(&name, fn_type, None);
                            let basic_block = self.context.append_basic_block(function, "entry");
                            self.builder.position_at_end(basic_block);


                            for (i, arg) in args.iter().enumerate() {
                                let llvm_arg = function.get_nth_param(i as u32).unwrap().into_pointer_value();

                                let alloca = self.builder.build_alloca(
                                    llvm_arg.get_type(),
                                    &arg.name,
                                ).unwrap();

                                self.builder.build_store(alloca, llvm_arg).unwrap();
                                self.variables.insert(arg.name.clone(), alloca);
                            }
                        }
                    }

                    {
                        let module = self.module.lock().unwrap();
                        println!("Module: {:?}", module);

                        dbg!("creating function named: {}", &name);
                        let fn_type = self.context.i32_type().fn_type(&arg_types, false);
                        let function = module.add_function(&name, fn_type, None);
                        let basic_block = self.context.append_basic_block(function, "entry");
                        self.builder.position_at_end(basic_block);
                        dbg!("done");

                        dbg!("parsing the function's body");
                        for (i, arg) in args.iter().enumerate() {
                            let llvm_arg = function.get_nth_param(i as u32).unwrap().into_pointer_value();

                            let alloca = self.builder.build_alloca(
                                llvm_arg.get_type(),
                                &arg.name,
                            ).unwrap();

                            self.builder.build_store(alloca, llvm_arg).unwrap();
                            self.variables.insert(arg.name.clone(), alloca);
                        }
                    }
                        self.execute_every_stmt_in_code(body, discovered_modules);
                        let return_value = self.context.i32_type().const_int(0 as u64, false);
                        self.builder.build_return(Some(&return_value)).unwrap();
                }
                Stmt::Import { path, nickname } => {
                    let key = path.join("/");
                    if self.loaded_modules.contains_key(&key) {
                        eprintln!("Module `{}` already loaded", key);
                        return;
                    }

                    let path_str = format!("{}.kurai", path.join("/"));
                    let code = std::fs::read_to_string(&path_str).expect(&format!("Failed to load module {}", path_str));

                    let tokens = Token::tokenize(&code);
                    let mut pos = 0;
                    let mut stmts = Vec::new();

                    while pos < tokens.len() {
                        match parse_imported_file(&tokens, &mut pos, discovered_modules) {
                            Ok(stmt) => stmts.push(stmt),
                            Err(e) => panic!("Failed to parse stmt at pos: {}\nError: {}", pos, e)
                        }
                    }

                    self.loaded_modules.insert(key.clone(), stmts.clone());
                    self.execute_every_stmt_in_code(stmts, discovered_modules);

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

                        let next_block = Some(self.build_conditional_branch(
                            current_function,
                            &branch.condition,
                            &branch.body.clone(),
                            &else_body,
                            discovered_modules,
                            &i.to_string()));

                        // prev_block = next_block.unwrap();
                    }

                    // restore builder position if needed
                    // or move to merge block 
                    // if self.builder.get_insert_block().is_some() {
                    //     self.builder.position_at_end(merge_block);
                    // } else {
                    //     self.builder.position_at_end(prev_block);
                    // }
                }
            }
        }
    }
}
