use std::collections::HashMap;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

use inkwell::types::BasicType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, PointerValue};
use inkwell::AddressSpace;
use inkwell::types::BasicMetadataTypeEnum::PointerType;

use kurai_codegen::codegen::codegen::CodeGen;
use kurai_typedArg::typedArg::TypedArg;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;

static GLOBAL_STRING_ID: AtomicUsize = AtomicUsize::new(0);

impl<'ctx> CodeGen<'ctx> {
    pub fn printf_format(&self, args: &Vec<TypedArg>, id: usize) -> Vec<BasicValueEnum<'ctx>> {
        args.iter()
            .enumerate()
            .filter_map(|(i, arg)| {
                match arg.typ.to_string().as_str() {
                    "int" => self.compile_int(arg),
                    "str" => self.compile_str(arg, id, i),
                    "id" => self.compile_id(arg),
                    _ => None
                }
            })
        .collect()
    }

    pub fn compile_int(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        match &arg.value {
            Some(Expr::Literal(Value::Int(v))) => Some(self.context.i64_type().const_int(*v as u64, true).into()),
            _ => None
        }
    }

    pub fn compile_str(&self, arg: &TypedArg, id: usize, index: usize) -> Option<BasicValueEnum<'ctx>> {
        match &arg.value {
            Some(Expr::Literal(Value::Str(s))) => {
                let global_str = self.builder.build_global_string_ptr(s, &format!("str_{}_{}", id, index));
                Some(global_str.unwrap().as_basic_value_enum())
            }
            _ => None
        }
    }

    pub fn compile_id(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
        if let Some(var_ptr) = self.variables.get(&arg.name) {
            let ptr_type = var_ptr.get_type();
            let loaded_val = self.builder.build_load(ptr_type.as_basic_type_enum(), *var_ptr, "loaded_id");
            loaded_val.ok()
        } else {
            None
        }
    }

    pub fn printf(/* env: &mut IRContext, */args: &Vec<TypedArg>, codegen: &mut CodeGen) -> Result<(), String>{
        let id = GLOBAL_STRING_ID.fetch_add(1, Ordering::Relaxed);

        let mut format = String::new();
        for arg in args.iter() {
            match arg.typ.to_string().as_str() {
                "int" => format.push_str("%d"),
                "str" => format.push_str("%s"),
                "id" => {
                    // if let Some(var) = codegen.variables.get(&arg.name) {
                    //     let loaded_val = codegen.builder.build_load(var.get_type(), *var, "load_id").unwrap();
                    //
                    //     match loaded_val.get_type().to_string().as_str() {
                    //         "i64" => format.push_str("%ld"),
                    //         "i32" => format.push_str("%d"),
                    //         "i8*" => format.push_str("%s"),
                    //         // _ => panic!("UNKNOWN IDENTIFIER VAR TYPE FOR PRINTF"),
                    //         _ => format.push_str("%s")
                    //     }
                    // }
                }
                _ => panic!("UNSUPPORTED PRINTF ARG TYPE")
            }
        }
        format.push('\n');

        let format_str = codegen.builder
            .build_global_string_ptr(&format, &format!("fmt_{}", id))
            .map_err(|e| format!("Error building global string pointer: {:?}", e))?
            .as_pointer_value()
            .as_basic_value_enum();

        // let mut final_args: Vec<BasicMetadataValueEnum> = Vec::new();
        let mut final_args: Vec<BasicMetadataValueEnum> = vec![format_str.into()];
        {
            let compiled_args = codegen.printf_format(&args, id);
            final_args.extend(
                compiled_args
                    .clone()
                    .into_iter()
                    .map(|arg| Into::<BasicMetadataValueEnum>::into(arg))
            );

            println!("Compiled args: {:?}", compiled_args.len());
        }

        let module = codegen.module.lock().unwrap();

        let printf_fn = module.get_function("printf").expect("printf isnt defined. Did you mean to import printf?");
        codegen.builder.build_call(printf_fn, &final_args, &format!("printf_call_{}", id));   

        Ok(())
    }

    pub fn import_printf(codegen: &mut CodeGen) -> Result<(), String> {
        let module = codegen.module.lock().unwrap();

        let printf_type = codegen.context.i32_type().fn_type(
            &[PointerType(codegen.context.i8_type().ptr_type(AddressSpace::default().into()))], true);
        module.add_function("printf", printf_type, None);

        Ok(())
    }
}
