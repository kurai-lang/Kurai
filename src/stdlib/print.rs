use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
use inkwell::AddressSpace;
use inkwell::types::BasicMetadataTypeEnum::PointerType;

use crate::codegen::codegen::CodeGen;
use crate::parse::expr::Expr;
use crate::typedArg::TypedArg;
use crate::value::Value;

impl<'ctx> CodeGen<'ctx> {
    pub fn printf_format(&self, args: Vec<TypedArg>) -> Vec<BasicValueEnum<'ctx>> {
        let mut compiled_args: Vec<BasicValueEnum> = Vec::new();

        for arg in args {
            if let Some(expr) = arg.value {
                match (arg.typ.to_string().as_str(), expr) {
                    ("int", Expr::Literal(Value::Int(v))) => {
                        let int = self.context.i64_type().const_int(v as u64, true).into();
                        compiled_args.push(int);
                    }
                    ("str", Expr::Literal(Value::Str(s))) => {
                        let str_global = self.builder.
                            build_global_string_ptr(&s, "str")
                            .unwrap();
                        compiled_args.push(str_global.as_pointer_value().into());
                    }
                    _ => panic!("Unsupported type in printf_format")
                }
            }
        }
        compiled_args
    }
}

pub fn printf(/* env: &mut IRContext, */args: Vec<TypedArg>, codegen: &mut CodeGen) -> Result<(), String>{
    let format = match args.get(0) {
        Some(TypedArg { typ, .. }) if typ == "str" => "%s\n",
        _ => "%d\n"
    };

    let format_str = codegen.builder
        .build_global_string_ptr(format, "fmt")
        .map_err(|e| format!("Error building global string pointer: {:?}", e))?
        .as_pointer_value();

    let compiled_args = codegen.printf_format(args);

    let mut final_args: Vec<BasicMetadataValueEnum> = vec![format_str.into()];
    final_args.extend(
        compiled_args
            .clone()
            .into_iter()
            .map(|arg| Into::<BasicMetadataValueEnum>::into(arg))
    );
    
    let module = codegen.module.lock().unwrap();

    let printf_fn = module.get_function("printf").unwrap();
    codegen.builder.build_call(printf_fn, &final_args, "printf_call");

    
    println!("Compiled args: {:?}", compiled_args.len());

    Ok(())
}

pub fn import_printf(codegen: &mut CodeGen) -> Result<(), String> {
    let module = codegen.module.lock().unwrap();

    let printf_type = codegen.context.i32_type().fn_type(
        &[PointerType(codegen.context.i8_type().ptr_type(AddressSpace::default().into()))], true);
    module.add_function("printf", printf_type, None);

    Ok(())
}
