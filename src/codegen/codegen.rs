use crate::{parse::{expr::Expr, stmt::Stmt}, stdlib::{self, print::{import_printf, printf}}, typedArg::TypedArg, value::Value};
use colored::Colorize;
use inkwell::{
    builder::Builder, context::Context, module::Module, types::BasicMetadataTypeEnum::{self, PointerType}, values::{BasicMetadataValueEnum, BasicValueEnum, PointerValue}, AddressSpace
};
use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Arc<Mutex<Module<'ctx>>>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = Arc::new(Mutex::new(context.create_module("main_module")));
        let variables = HashMap::new();
        Self {
            context,
            builder,
            module,
            variables,
            // context: &'ctx Context
        }
    }
    pub fn generate_code(&mut self, parsed_stmt: Vec<Stmt> /*, context: &'ctx Context, builder: &Builder, module: &mut Arc<Mutex<Module<'ctx>>> */) {
        {
            // let module = self.module.lock().unwrap();

            // main function signature
            // (i32, i8**)
            // let i32_type = self.context.i32_type();
            // let i8_ptr_ptr_type = self.context.i8_type()
            //     .ptr_type(AddressSpace::default())
            //     .ptr_type(AddressSpace::default());
            //
            // let fn_type = i32_type.fn_type(
            //     &[i32_type.into(), i8_ptr_ptr_type.into()],
            //     false,
            // );
            // let printf_type = i32_type.fn_type(
            //     &[PointerType(self.context.i8_type().ptr_type(AddressSpace::default().into()))], true);

            // module.add_function("printf", printf_type, None);

            // Declare main function
            // Add a basic block 
            // And 
            // Position builder at start of the block
            // let main_fn = module.add_function("main", fn_type, None);
            // let entry_block = self.context.append_basic_block(main_fn, "entry");
            // self.builder.position_at_end(entry_block);
            import_printf(self);
        }
        self.execute_every_stmt_in_code(parsed_stmt);

        // self.builder.build_call(
        //     printf_fn,
        //     &[format_str.as_pointer_value().into(), some_value.into()],
        //     "printf_call",
        // );
    }

    // pub context: &'ctx Context,
    // pub builder: Builder<'ctx>,
    // pub module: Arc<Mutex<Module<'ctx>>>,
    // pub variables: HashMap<String, PointerValue<'ctx>>,
    fn execute_every_stmt_in_code(&mut self, parsed_stmt: Vec<Stmt>) {
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
                    if name == "printf" {
                        stdlib::print::printf(args, self);
                        return;
                    }
                    {
                        let module = self.module.lock().unwrap();

                        println!("Module: {:?}", module);
                        let function = module.get_function(&name);

                        if let Some(function) = function {
                            let mut compiled_args: Vec<BasicMetadataValueEnum> = Vec::new();
                            for arg in &args {
                                if let Some(value) = &arg.value {
                                    if let Some(llvm_value) = self.lower_expr_to_llvm(&value) {
                                        compiled_args.push(llvm_value.into());
                                    }
                                } else {
                                    println!("{} {}", "Failed to compile arguments for function".red(), name.red());
                                }
                            }
                            self.builder.build_call(function, &compiled_args, &name);
                        } else {
                            println!("{} {}", "Couldnt find function named:".red(), name.red());
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
                        self.execute_every_stmt_in_code(body);
                        let return_value = self.context.i32_type().const_int(0 as u64, false);
                        self.builder.build_return(Some(&return_value)).unwrap();
                }
            }
        }
    }

    fn lower_value_to_llvm(&self, value: &Value) -> Option<BasicValueEnum<'ctx>> {
        match value {
            Value::Int(v) => Some(self.context.i64_type().const_int(*v as u64, true).into()),
            Value::Float(v) => Some(self.context.f64_type().const_float(*v as f64).into()),
            Value::Bool(v) => Some(self.context.bool_type().const_int(*v as u64, false).into()),
            // Value::Str(v) => Some(self.context.i8_type().const_int(*v as u64, false).into()),
            _ => None,
        }
    }

    fn lower_expr_to_llvm(&self, expr: &Expr) -> Option<BasicValueEnum<'ctx>> {
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
            Expr::Binary { op, left, right } => todo!(),
            Expr::Call { name, args } => todo!(),
        }
    }
}

#[allow(warnings)]
impl<'ctx> CodeGen<'ctx> {
    pub fn show_result(&self) -> String {
        let module = self.module.lock().unwrap();
        module.print_to_stderr();
        println!("{:#?}", *module.get_data_layout());
        return module.print_to_string().to_string();
    }
}

