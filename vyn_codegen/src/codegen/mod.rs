// ERROR: use crate::{parse::{bin_op::BinOp, parse_import::parse_imported_file::parse_imported_file}, stdlib::{self, print::{import_printf, printf}}}; use crate::token::token::Token;
// ERROR: use crate::parse::parse_import::parse_import_decl::parse_import_decl;
pub mod traits;
pub mod passes;
pub mod value;

use colored::Colorize;
use vyn_core::scope::Scope;

use vyn_parser::parse::Parser;
use vyn_types::{typ::Type, value::Value};
use vyn_ast::expr::Expr;
use vyn_ast::stmt::Stmt;
use vyn_ast::typedArg::TypedArg;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module, values::{AnyValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, IntValue, PointerValue}, AddressSpace, IntPredicate
};
use std::{cell::{Ref, RefCell}, collections::{HashMap, HashSet}, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};
use std::sync::{Arc, Mutex};

use crate::{codegen::passes::lower_expr_to_llvm, registry::registry::AttributeRegistry};

static GLOBAL_STRING_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct VariableInfo<'ctx> {
    pub ptr_value: PointerValue<'ctx>,
    pub var_type: Type,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub ret_type: Type,
    pub args: Vec<TypedArg>,
    pub is_extern: bool,
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

    pub functions: HashMap<String, FunctionInfo>,
    pub inline_fns: HashSet<String>,
    pub current_fn_ret_type: Type,

    pub final_check_blocks: Vec<BasicBlock<'ctx>>,
    terminated_blocks: HashSet<BasicBlock<'ctx>>,

    pub src: &'ctx str,

    pub parser: Rc<RefCell<Parser>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, src: &'ctx str, parser: Parser) -> Self {
        let builder = context.create_builder();
        let module = Arc::new(Mutex::new(context.create_module("main_module")));
        let variables = HashMap::new();
        let loaded_modules = HashMap::new();
        let attr_registry = AttributeRegistry {
            handlers: HashMap::new(),
            parser: parser.clone(),
        };
        // dang.. this straight up looks like attribute has its own parser
        // and this main codegen has its own parser too

        let refcell_parser = RefCell::new(parser);
        let parser = Rc::new(refcell_parser);

        Self {
            context,
            builder,
            module,
            variables,
            loaded_modules,
            string_counter: 0,
            loop_counter: 0,
            loop_exit_stack: Vec::new(),
            attr_registry,

            functions: HashMap::new(),
            inline_fns: HashSet::new(),
            current_fn_ret_type: Type::Void,

            final_check_blocks: Vec::new(),
            terminated_blocks: HashSet::new(),

            src,
            parser
            // context: &'ctx Context
        }
    }

    pub fn init(mut self) -> Self {
        self.import_printf().unwrap();
        self
    }

    pub fn generate_code(
        &mut self,
        parsed_stmt: Vec<Stmt>,
        exprs: Vec<Expr>,
    ) {
        // nothing lol, just for fun
        // self.import_printf().expect("Couldnt import printf for unknown reasons");
        let mut parser = Rc::clone(&self.parser);
        self.execute_every_stmt_in_code(
            parsed_stmt,
            &mut parser.borrow_mut(),
            None
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

    fn printf_format(&mut self, args: &[Expr], parser: &mut Parser, format: &mut String) -> Vec<BasicValueEnum<'ctx>> {
        let mut values = Vec::new();

        args.iter().for_each(|arg| {
            self.lower_expr_to_llvm(
                arg,
                None,
                parser,
                None
            ).map(|(val, ty)| {
                #[cfg(debug_assertions)]
                println!("{} value: {:?}", "[printf_format()]".green().bold(), val);

                let llvm_value = val.print_to_string().to_string();
                #[cfg(debug_assertions)]
                println!("{} llvm_value: {}", "[printf_format()]".green().bold(), llvm_value);

                // handling the \ lol
                let raw_str = self.extract_string_from_llvm_decl(llvm_value);
                #[cfg(debug_assertions)]
                println!("{} raw_str: {}", "[printf_format()]".green().bold(), raw_str);

                let parsed = self.parse_escape_sequences(raw_str);
                #[cfg(debug_assertions)]
                println!("{} parsed: {}", "[printf_format()]".green().bold(), parsed);

                values.push(val);
                match ty {
                    Type::I64 => format.push_str("%ld"),
                    Type::Str | Type::Var | Type::Void => format.push_str("%s"),
                    _ => panic!("unsupported print arg type {:?}", ty)
                }
            });
        });

        values
    }

    fn parse_escape_sequences(&self, content: String) -> String {
        let mut out = String::new();
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.clone().peek() {
                    Some('n') => {
                        chars.next();
                        out.push('\n');
                    }
                    Some('t') => {
                        chars.next();
                        out.push('\t');
                    }
                    Some('r') => {
                        chars.next();
                        out.push('\r');
                    }
                    Some('0'..='7') => {
                        let mut octal = String::new();

                        for _ in 0..3 {
                            if let Some(digit) = chars.clone().peek() {
                                if digit.is_digit(8) {
                                    chars.next();
                                    octal.push(*digit);
                                } else {
                                    break;
                                }
                            }
                        }

                        if let Ok(val) = u8::from_str_radix(&octal, 8) {
                            out.push(val as char);
                        }
                    }
                    Some(other) => {
                        chars.next();
                        out.push(*other);
                    }
                    _ => out.push('\\'),
                }
            } else {
                // no `\`? then store it as itself, the usual lol
                out.push(ch);
            }
        }

        out
    }

    fn extract_string_from_llvm_decl(&self, content: String) -> String {
        let start = content.find("c\"").unwrap() + 2;
        let end = content[start..].find('"').unwrap() + start;
        content[start..end].to_string()
    }

    pub fn printf(&mut self, args: &[Expr], expected_type: Option<&Type>, parser: &mut Parser) -> Result<(), String>{
        let id = GLOBAL_STRING_ID.fetch_add(1, Ordering::Relaxed);

        let mut format = String::new();
        let mut final_args: Vec<BasicMetadataValueEnum> = Vec::new();

        let compiled_args = self.printf_format(args, parser, &mut format);
        #[cfg(debug_assertions)]
        println!("Compiled args: {:?}", compiled_args.len());

        let format_str = self.builder
            .build_global_string_ptr(&format, &format!("fmt_{id}"))
            .map_err(|e| format!("Error building global string pointer: {:?}", e))?
            .as_pointer_value()
            .as_basic_value_enum();

        let module = self.module
            .lock()
            .unwrap();

        // the format
        final_args.insert(0, format_str.into());

        final_args.extend(
            compiled_args
                .into_iter()
                .map(|arg| Into::<BasicMetadataValueEnum>::into(arg))
        );

        let printf_fn = module
            .get_function("printf")
            .expect("printf isnt defined. Did you mean to import printf?");

        #[cfg(debug_assertions)]
        println!("Final args: {:?}", final_args);
        self.builder.
            build_call(printf_fn, &final_args, &format!("printf_call_{}", id))
            .unwrap();

        Ok(())
    }

    pub fn import_printf(&mut self) -> Result<(), String> {
        #[cfg(debug_assertions)]
        println!("printf imported!");

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

