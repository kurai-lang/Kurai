use crate::{eat::eat, parse::{expr::Expr, parse::parse, stmt::{self, Stmt}}, token::token::Token, typedArg::TypedArg, value::Value};
use inkwell::{
    builder::Builder, context::Context, module::Module, types::{BasicMetadataTypeEnum::{self, PointerType}, IntType}, values::{BasicMetadataValueEnum, BasicValueEnum, GenericValue, IntValue}, AddressSpace
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Arc<Mutex<Module<'ctx>>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = Arc::new(Mutex::new(context.create_module("main_module")));
        Self {
            context,
            builder,
            module,
            // context: &'ctx Context
        }
    }
    pub fn generate_code(&self, parsed_stmt: Vec<Stmt> /*, context: &'ctx Context, builder: &Builder, module: &mut Arc<Mutex<Module<'ctx>>> */) {
        // main function signature
        // (i32, i8**)
        let i32_type = self.context.i32_type();
        let i8_ptr_ptr_type = self.context.i8_type()
            .ptr_type(AddressSpace::default())
            .ptr_type(AddressSpace::default());

        let fn_type = i32_type.fn_type(
            &[i32_type.into(), i8_ptr_ptr_type.into()],
            false,
        );
        let printf_type = i32_type.fn_type(
            &[PointerType(self.context.i8_type().ptr_type(AddressSpace::default().into()))], true);

        let module = self.module.lock().unwrap();

        module.add_function("printf", printf_type, None);

        // Declare main function
        // Add a basic block 
        // And 
        // Position builder at start of the block
        let main_fn = module.add_function("main", fn_type, None);
        let entry_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_block);

        let return_val = i32_type.const_int(0, false);
        self.builder.build_return(Some(&return_val));

        for stmt in parsed_stmt {
            match stmt {
                Stmt::VarDecl { name, typ, value } => {           
                    self.handle_var_decl(name, value.clone(), &self.builder, &self.context);

                    // if let Some(Value::Int(v)) = value {
                    //     some_value = self.context.i64_type().const_int(v as u64, true);
                    //
                    //     self.builder.build_call(
                    //         printf_fn,
                    //         &[format_str.as_pointer_value().into(), some_value.into()],
                    //         "printf_call"
                    //     );
                    // }
                    
                    if let Some(Value::Int(v)) = value {
                        let value = self.context.i64_type().const_int(v as u64, false);
                    }
                }
                Stmt::Assign { name, value } => {
                    todo!();
                }
                Stmt::FnCall { name, args } => {
                    let function = module.get_function(&name).unwrap();

                    // TODO: Im continuing this tomorrow
                    // let mut compiled_args = Vec::new();
                    // for arg in args {
                    //     let value = self.
                    // }
                }
                Stmt::FnDecl { name, args, body } => {
                    // Map the argument types to LLVM types 
                    // remember, we need to speak LLVM IR language, not rust!
                    dbg!("converting args to llvm args types");
                    let arg_types: Vec<BasicMetadataTypeEnum> = args.iter().map(|arg| {
                        match arg.typ.as_str() {
                            "int" => self.context.i32_type().into(),
                            "float" => self.context.f32_type().into(),
                            "bool" => self.context.bool_type().into(),
                            "str" => self.context.i8_type().ptr_type(AddressSpace::default()).into(),
                            _ => panic!("Unknown type: {}", arg.typ),
                        }
                    }).collect();
                    dbg!("done");

                    // WARNING: pls dont uncommonet this
                    // dbg!("locking module");
                    // let module = self.module.lock().unwrap();

                    dbg!("creating function named: {}", &name);
                    let fn_type = self.context.i32_type().fn_type(&arg_types, false);
                    let function = module.add_function(&name, fn_type, None);
                    let basic_block = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(basic_block);
                    dbg!("done");
                }
            }
        }

        // self.builder.build_call(
        //     printf_fn,
        //     &[format_str.as_pointer_value().into(), some_value.into()],
        //     "printf_call",
        // );

        let return_val = i32_type.const_int(0, false);
        self.builder.build_return(Some(&return_val));
    }
    
    // TODO: this too, i need to masturbate at exactly 11:40
    fn lower_expr_to_llvm(&self, expr: &Expr) -> Result<BasicValueEnum, String> {
        match expr {
            Expr::Int(v) => {
                Ok(self.context.i64_type().const_int(*v as u64, false).into())
            }
            _ => Err("mwa".to_string())
        }
    }

    // pub context: &'ctx Context,
    // pub builder: Builder<'ctx>,
    // pub module: Arc<Mutex<Module<'ctx>>>,
    // fn handle_fn_decl(&self, name: String, args: Vec<TypedArg>, body: Vec<Stmt>, module: &Module<'ctx>, builder: &Builder<'ctx>, context: &Context) {
    // }

    // fn allocate_i64(context: &mut Context, builder: &mut Builder) {
    //     let i64_type = context.i64_type();
    //     let ptr = builder.build_alloca(i64_type, "i64_type");
    // }

    // First we must take the outputted VarDecl
    // because thats the parsed variable
    // we can extract some values from it, and use it to our 
    // llvm ir code, and inkwell 
    // fn parse_to_llvm_ir(datatype: IntType, builder: &mut Builder) {
    //     let tokens = Token::tokenize("int x = 5;");
    //     let parsed_stmt = parse(&tokens);
    //
    //     match parsed_stmt.unwrap() {
    //         Stmt::VarDecl { name, typ, value } => {
    //             let alloca = builder.build_alloca(datatype, &name).unwrap();
    //
    //             if let Some(Value::Int(v)) = value {
    //                 let init_val = datatype.const_int(v as u64, false);
    //                 builder.build_store(alloca, init_val);
    //             }
    //         }
    //         Stmt::Assign { name, value } => {
    //             todo!()
    //         }
    //     }
    // }

    fn handle_var_decl<'a>(&self, name: String, value: Option<Value>, builder: &Builder<'a>, context: &Context) {
        let i64_type = context.i64_type();
        let alloca = builder.build_alloca(i64_type, &name).unwrap();

        if let Some(Value::Int(v)) = value {
            let init_val = i64_type.const_int(v as u64, true);
            builder.build_store(alloca, init_val).unwrap();
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn show_result(&self) -> String {
        let module = self.module.lock().unwrap();
        module.print_to_stderr();
        println!("{:#?}", *module.get_data_layout());
        return module.print_to_string().to_string();
    }
}

