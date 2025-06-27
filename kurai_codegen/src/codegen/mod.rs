// ERROR: use crate::{parse::{bin_op::BinOp, parse_import::parse_imported_file::parse_imported_file}, stdlib::{self, print::{import_printf, printf}}}; use crate::token::token::Token;
// ERROR: use crate::parse::parse_import::parse_import_decl::parse_import_decl;
pub mod traits;
pub mod passes;
pub mod value;

use colored::Colorize;
use kurai_core::scope::Scope;
use kurai_parser::GroupedParsers;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::{typ::Type, value::Value};
use kurai_expr::expr::Expr;
use kurai_stmt::stmt::Stmt;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue}, AddressSpace, IntPredicate
};
use std::{collections::{HashMap, HashSet}, sync::atomic::{AtomicUsize, Ordering}};
use std::sync::{Arc, Mutex};

use crate::registry::registry::AttributeRegistry;

static GLOBAL_STRING_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct VariableInfo<'ctx> {
    pub ptr_value: PointerValue<'ctx>,
    pub var_type: Type,
}

// #[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Arc<Mutex<Module<'ctx>>>,
    pub variables: HashMap<String, VariableInfo<'ctx>>,
    pub loaded_modules: HashMap<String, Vec<Stmt>>,
    pub string_counter: usize,
    pub loop_counter: usize,
    pub loop_exit_stack: Vec<BasicBlock<'ctx>>,
    pub attr_registry: AttributeRegistry,

    pub inline_fns: HashSet<String>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        let module = Arc::new(Mutex::new(context.create_module("main_module")));
        let variables = HashMap::new();
        let loaded_modules = HashMap::new();
        let attr_registry = AttributeRegistry {
            handlers: HashMap::new(),
        };
        Self {
            context,
            builder,
            module,
            variables,
            loaded_modules,
            string_counter: 0,
            loop_counter: 0,
            loop_exit_stack: vec![],
            attr_registry,

            inline_fns: HashSet::new(),
            // context: &'ctx Context
        }
    }
    pub fn generate_code(
        &mut self,
        parsed_stmt: Vec<Stmt>,
        exprs: Vec<Expr>, 
        discovered_modules: &mut Vec<String>, 
        parsers: &GroupedParsers,
        scope: &mut Scope,
    ) {
        // WARNING: nothing lol ,just for fun
        // self.import_printf().expect("Couldnt import printf for unknown reasons");

        self.execute_every_stmt_in_code(
            parsed_stmt,
            discovered_modules,
            parsers,
            scope
        );

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
                match arg.typ {
                    Type::Int => self.compile_int(arg),
                    Type::Str => self.compile_str(arg),
                    Type::Var => self.compile_id(arg),
                    _ => {
                        panic!("{}: Unknown typ: {:?}", "error".red().bold(), arg.typ);
                    }
                }
            })
        .collect()
    }

    fn compile_int(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        // let global = self.module.lock().unwrap().
        //     add_global(self.context.i8_type().array_type(5), None, "int_fmt");
        // global.set_initializer(&self.context.const_string(b"%ld\n\0", false));
        // global.set_linkage(inkwell::module::Linkage::Private);
        // global.set_constant(true);
        //
        // let gep = unsafe {
        //     self.builder.build_gep(
        //         self.context.i8_type(),
        //         global.as_pointer_value(), 
        //         &[self.context.i32_type().const_zero(), self.context.i32_type().const_zero()],
        //         "fmt_ptr")
        //         .unwrap()
        // };

        match &arg.value {
            Some(Expr::Literal(Value::Int(v))) => {
                // Some(self.context.i64_type().const_int(*v as u64, true).into())
                // let print_fn = self.module.lock().unwrap().get_function("printf");
                let int_val = self.context.i64_type().const_int(*v as u64, true);

                // self.builder.build_call(
                //     print_fn?,
                //     &[gep.into(), int_val.into()], 
                //     "printf_call_int"
                // ).unwrap();

                Some(int_val.into())
            }
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
        if let Some(var_info) = self.variables.get(&arg.name) {
            let ptr_type = match var_info.var_type.to_llvm_type(self.context) {
                Some(ty) => ty,
                None => {
                    eprintln!("Type of variable `{}` is unknown at compile time.", arg.name);
                    return None;
                }
            };
            let loaded_val = self.builder.build_load(
                ptr_type,
                var_info.ptr_value,
                "loaded_id"
            ).unwrap();

            println!("{:?}", loaded_val.get_type());

            // WARNING: THIS IS UNUSED, MIGHT BE REMOVED IN THE FUTURE
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

    pub fn printf(&mut self, args: &Vec<TypedArg>) -> Result<(), String>{
        let id = GLOBAL_STRING_ID.fetch_add(1, Ordering::Relaxed);

        let mut format = String::new();
        let mut final_args: Vec<BasicMetadataValueEnum> = Vec::new();

        for arg in args.iter() {
            match arg.typ {
                Type::Int => {
                    format.push_str("%ld");

                    // if let Some(val) = self.compile_int(arg) {
                    //     final_args.push(val.into());
                    // } else {
                    //     return Err("Failed to compile int".to_string());
                    // }
                }
                Type::Str => {
                    format.push_str("%s");

                    // if let Some(val) = self.compile_str(arg) {
                    //     final_args.push(val.into());
                    // } else {
                    //     return Err("Failed to compile str".to_string());
                    // }
                }
                Type::Var => {
                    format.push_str("%s");

                    if let Some(val) = self.compile_id(arg) {
                        final_args.push(val.into());
                    } else {
                        return Err("Failed to compile var".to_string());
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

        final_args.insert(0, format_str.into());

        let printf_fn = module.get_function("printf").expect("printf isnt defined. Did you mean to import printf?");
        self.builder.
            build_call(printf_fn, &final_args, &format!("printf_call_{}", id))
            .unwrap();

        Ok(())
    }

    pub fn import_printf(&mut self) -> Result<(), String> {
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

