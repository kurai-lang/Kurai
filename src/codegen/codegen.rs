use crate::{parse::{bin_op::BinOp, parse_import::parse_imported_file::parse_imported_file}, stdlib::{self, print::{import_printf, printf}}};
use crate::token::token::Token;
use crate::value::Value;
use crate::parse::{expr::Expr, stmt::Stmt};
use crate::parse::parse_import::parse_import_decl::parse_import_decl;
use colored::Colorize;
use inkwell::{
    builder::Builder, context::Context, module::Module, types::BasicMetadataTypeEnum::{self}, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue}, AddressSpace, IntPredicate
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Arc<Mutex<Module<'ctx>>>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
    pub loaded_modules: HashMap<String, Vec<Stmt>>
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
            // context: &'ctx Context
        }
    }
    pub fn generate_code(&mut self, parsed_stmt: Vec<Stmt>, exprs: Vec<Expr>, discovered_modules: &mut Vec<String> /*, context: &'ctx Context, builder: &Builder, module: &mut Arc<Mutex<Module<'ctx>>> */) {
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
        }

        import_printf(self);

        self.execute_every_stmt_in_code(parsed_stmt, discovered_modules);
        self.execute_every_expr_in_code(exprs);

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
            panic!("tried to convert non-int into bool");
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

