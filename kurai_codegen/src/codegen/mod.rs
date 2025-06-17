// ERROR: use crate::{parse::{bin_op::BinOp, parse_import::parse_imported_file::parse_imported_file}, stdlib::{self, print::{import_printf, printf}}}; use crate::token::token::Token;
// ERROR: use crate::parse::parse_import::parse_import_decl::parse_import_decl;
pub mod traits;
pub mod passes;
pub mod value;

use colored::Colorize;
use kurai_parser::{BlockParser, FunctionParser, ImportParser, LoopParser, StmtParser};
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::value::Value;
use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::BasicType, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue}, AddressSpace, IntPredicate
};
use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};
use std::sync::{Arc, Mutex};

static GLOBAL_STRING_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Arc<Mutex<Module<'ctx>>>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
    pub loaded_modules: HashMap<String, Vec<Stmt>>,
    pub string_counter: usize,
    pub loop_counter: usize,
    pub loop_exit_stack: Vec<BasicBlock<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = Arc::new(Mutex::new(context.create_module("main_module")));
        let variables = HashMap::new();
        let loaded_modules = HashMap::new();
        Self {
            context,
            builder,
            module,
            variables,
            loaded_modules,
            string_counter: 0,
            loop_counter: 0,
            loop_exit_stack: vec![]
            // context: &'ctx Context
        }
    }
    pub fn generate_code(
        &mut self,
        parsed_stmt: Vec<Stmt>,
        exprs: Vec<Expr>, 
        discovered_modules: &mut Vec<String>, 
        stmt_parser: &dyn StmtParser, 
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser
    ) {
        // WARNING: nothing lol ,just for fun
        // self.import_printf().expect("Couldnt import printf for unknown reasons");

        self.execute_every_stmt_in_code(parsed_stmt, discovered_modules, stmt_parser, fn_parser, import_parser, block_parser, loop_parser);

        if !exprs.is_empty() {
            #[cfg(debug_assertions)]
            if let Err(e) = self.execute_every_expr_in_code(exprs) {
                eprintln!("Expression evaluation failed: {}", e);
            }
        }

        // self.builder.build_call(
        //     printf_fn,
        //     &[format_str.as_pointer_value().into(), some_value.into()],
        //     "printf_call",
        // );
    }

    pub fn lower_value_to_llvm(&self, value: &Value) -> Option<BasicValueEnum<'ctx>> {
        match value {
            Value::Int(v) => Some(self.context.i64_type().const_int(*v as u64, true).into()),
            Value::Float(v) => Some(self.context.f64_type().const_float(*v).into()),
            Value::Bool(v) => Some(self.context.bool_type().const_int(*v as u64, false).into()),
            // Value::Str(v) => Some(self.context.i8_type().const_int(*v as u64, false).into()),
            _ => None,
        }
    }

    pub fn to_bool(&self, val: BasicValueEnum<'ctx>) -> IntValue<'ctx> {
        if val.is_int_value() && val.into_int_value().get_type().get_bit_width() != 1 {
            // Cast non-bool to i1 (bool)
            let zero = self.context.i64_type().const_int(0, false);
            self.builder.build_int_compare(
                IntPredicate::NE,
                val.into_int_value(),
                zero,
                "to_i1"
            ).unwrap()
        } else if val.is_int_value() {
            val.into_int_value()
        } else {
            panic!("{}: tried to convert non-int into bool", "error".red().bold());
        }
    }

    fn printf_format(&mut self, args: &Vec<TypedArg>) -> Vec<BasicValueEnum<'ctx>> {
        args.iter()
            .filter_map(|arg| {
                match arg.typ.as_str() {
                    "int" => self.compile_int(arg),
                    "str" => self.compile_str(arg),
                    "id" => self.compile_id(arg),
                    _ => {
                        panic!("{}: Unknown typ: {:?}", "error".red().bold(), arg.typ.as_str());
                        None
                    }
                }
            })
        .collect()
    }

    fn compile_int(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        match &arg.value {
            Some(Expr::Literal(Value::Int(v))) => Some(self.context.i64_type().const_int(*v as u64, true).into()),
            _ => None
        }
    }

    fn compile_str(&mut self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        match &arg.value {
            Some(Expr::Literal(Value::Str(s))) => {
                let global_str = self
                    .builder.build_global_string_ptr(s, &format!("str_{}", self.string_counter));

                self.string_counter += 1;
                Some(global_str.unwrap().as_basic_value_enum())
            }
            _ => {
                None
            }
        }
    }

    fn compile_id(&mut self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        if let Some(var_ptr) = self.variables.get(&arg.name) {
            let ptr_type = var_ptr.get_type();
            let loaded_val = self.builder.build_load(
                ptr_type.as_basic_type_enum(),
                *var_ptr,
                "loaded_id").unwrap();

            // let gep = unsafe {
            //     self.builder.build_gep(
            //         ptr_type.as_basic_type_enum(),
            //         *var_ptr,
            //         &[self.context.i32_type().const_zero()],
            //         format!("str_{}_gep", self.string_counter).as_str(),
            //     ).unwrap()
            // };
            
            Some(loaded_val.as_basic_value_enum())
        } else {
            None
        }
    }

    fn printf(&mut self, args: &Vec<TypedArg>) -> Result<(), String>{
        let id = GLOBAL_STRING_ID.fetch_add(1, Ordering::Relaxed);

        let mut format = String::new();
        for arg in args.iter() {
            match arg.typ.to_string().as_str() {
                "int" => format.push_str("%d"),
                "str" => format.push_str("%s"),
                "id" => {
                    if let Some(var) = self.variables.get(&arg.name) {
                        let loaded_val = self.builder.build_load(var.get_type(), *var, "load_id").unwrap();

                        match loaded_val.get_type().to_string().as_str() {
                            "i64" => format.push_str("%ld"),
                            "i32" => format.push_str("%d"),
                            "i8*" => format.push_str("%s"),
                            // _ => panic!("UNKNOWN IDENTIFIER VAR TYPE FOR PRINTF"),
                            _ => format.push_str("%s")
                        }
                    }
                }
                _ => panic!("UNSUPPORTED PRINTF ARG TYPE")
            }
        }
        format.push('\n');

        let format_str = self.builder
            .build_global_string_ptr(&format, &format!("fmt_{}", id))
            .map_err(|e| format!("Error building global string pointer: {:?}", e))?
            .as_pointer_value()
            .as_basic_value_enum();

        // let mut final_args: Vec<BasicMetadataValueEnum> = Vec::new();
        let mut final_args: Vec<BasicMetadataValueEnum> = vec![format_str.into()];
        {
            let compiled_args = self.printf_format(&args);
            final_args.extend(
                compiled_args
                    .clone()
                    .into_iter()
                    .map(|arg| Into::<BasicMetadataValueEnum>::into(arg))
            );

            #[cfg(debug_assertions)]
            {
                println!("Compiled args: {:?}", compiled_args.len());
            }
        }

        let module = self.module.lock().unwrap();

        let printf_fn = module.get_function("printf").expect("printf isnt defined. Did you mean to import printf?");
        self.builder.build_call(printf_fn,
            &final_args, &format!("printf_call_{}", id)).unwrap();

        Ok(())
    }

    fn import_printf(&mut self) -> Result<(), String> {
        #[cfg(debug_assertions)]
        {
            println!("printf imported!");
        }
        let module = self.module.lock().unwrap();

        let printf_type = self.context.i32_type().fn_type(
            &[self.context.i8_type().ptr_type(AddressSpace::default()).into()], 
            true
        );
        module.add_function("printf", printf_type, None);

        Ok(())
    }
}

#[allow(warnings)]
impl<'ctx> CodeGen<'ctx> {
    pub fn show_result(&self) -> String {
        let module = self.module.lock().unwrap();
        #[cfg(debug_assertions)]
        {
            module.print_to_stderr();
            println!("{:#?}", *module.get_data_layout());
        }
        return module.print_to_string().to_string();
    }
}

