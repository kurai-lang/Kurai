use inkwell::{types::BasicTypeEnum, values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue}};
use vyn_ast::stmt::Stmt;
use vyn_types::typ::Type;
use crate::codegen::CodeGen;

#[macro_export]
macro_rules! print_error {
    ($($msg:tt)*) => {
        eprintln!("{}: {}", "error".red().bold(), format!($($msg)*))
    };
}

#[macro_export]
macro_rules! print_hint {
    ($($msg:tt)*) => {
        eprintln!("{}: {}", "hint".cyan().bold(), format!($($msg)*))
    };
}

#[macro_export]
macro_rules! vyn_panic {
    () => {
        #[cfg(debug_assertions)]
        panic!("Compilation terminated due to previous error");

        #[allow(unreachable_code)] {
            eprintln!("Compilation terminated due to previous error");
            std::process::exit(1);
        }
    };
}


pub fn basic_value_enum_to_string(value: &BasicValueEnum) -> String {
    let res = match *value {
        BasicValueEnum::IntValue(_) => "IntValue",
        BasicValueEnum::FloatValue(_) => "FloatValue",
        BasicValueEnum::PointerValue(_) => "PointerValue",
        BasicValueEnum::StructValue(_) => "StructValue",
        BasicValueEnum::ArrayValue(_) => "ArrayValue",
        BasicValueEnum::VectorValue(_) => "VectorValue",
        BasicValueEnum::ScalableVectorValue(_) => "ScalableVectorValue",
    };
    res.to_string()
}

pub fn basic_type_enum_to_string(typ: &BasicTypeEnum) -> String {
    let res = match *typ {
        BasicTypeEnum::ArrayType(_) => "ArrayType",
        BasicTypeEnum::FloatType(_) => "FloatType",
        BasicTypeEnum::IntType(_) => "IntType",
        BasicTypeEnum::PointerType(_) => "PointerType",
        BasicTypeEnum::StructType(_) => "StructType",
        BasicTypeEnum::VectorType(_) => "VectorType",
        BasicTypeEnum::ScalableVectorType(_) => "ScalableVectorType",
    };
    res.to_string()
}

impl<'ctx> CodeGen<'ctx> {
    pub fn get_or_compile_function(
        &mut self,
        name: &str,
    ) -> Option<FunctionValue<'ctx>> {
        if let Some((modname, funcname)) = Self::split_module_function_name(name) {
        // found modname and funcname? compile.
            self.get_function_from_module(
                modname,
                funcname
            )
        } else {
            // already compiled? ok reuse it
            self.module.lock().unwrap().get_function(name)
        }
    }

    pub fn split_module_function_name(name: &str) -> Option<(&str, &str)> {
        let mut parts = name.split("::");
        let modname = parts.next().unwrap();
        let funcname = parts.next().unwrap();

        Some((modname, funcname))
    }

    pub fn get_function_from_module(
        &mut self,
        modname: &str,
        funcname: &str,
    ) -> Option<FunctionValue<'ctx>> {
        let mod_stmts = self.loaded_modules.get(modname)?;
        let already_compiled: Option<FunctionValue<'ctx>> = self.module.lock().unwrap().get_function(funcname);

        if let Some(func) = already_compiled {
            return Some(func);
        }

        let maybe_stmt = mod_stmts.iter().find(|stmt| {
            matches!(stmt, Stmt::FnDecl { name, .. } if name == funcname)
        });

        if let Some(stmt) = maybe_stmt {
            #[cfg(debug_assertions)]
            {
                use colored::Colorize;

                println!("
                    {} `{}` from `{}` is now being compiled", "Compiling function".green(),
                    funcname, modname);
            }

            self.generate_code(
                vec![stmt.clone()], 
                
            );
        }
        // try again after compiling
        self.module.lock().unwrap().get_function(funcname)
    } 
}

pub struct TypeInfer;

impl TypeInfer {
    pub fn infer_int_type(&self, value: i64) -> Type {
        let min_i8 = i8::MIN as i64;
        let max_i8 = i8::MAX as i64;
        let min_i16 = i16::MIN as i64;
        let max_i16 = i16::MAX as i64;
        let min_i32 = i32::MIN as i64;
        let max_i32 = i32::MAX as i64;

        if (value >= min_i8) && (value <= max_i8)   { return Type::I8;  }
        if (value >= min_i16) && (value <= max_i16) { return Type::I16; }
        if (value >= min_i32) && (value <= max_i32) { Type::I32         }
        else { Type::I64 }
    }

    pub fn infer_float_type(&self, value: f64) -> Type {
        let min_f32 = f32::MIN as f64;
        let max_f32 = f32::MAX as f64;
        // let min_f64 = f64::MIN as f64;
        // let max_f64 = f64::MAX as f64;

        if (value >= min_f32) && (value <= max_f32) { Type::F32 }
        else { Type::F64 }
    }

    pub fn infer_alignment_int_type(&self, value: i64) -> i64 {
        let min_i8 = i8::MIN as i64;
        let max_i8 = i8::MAX as i64;
        let min_i16 = i16::MIN as i64;
        let max_i16 = i16::MAX as i64;
        let min_i32 = i32::MIN as i64;
        let max_i32 = i32::MAX as i64;

        if (value >= min_i8) && (value <= max_i8)   { return 1; }
        if (value >= min_i16) && (value <= max_i16) { return 2; }
        if (value >= min_i32) && (value <= max_i32) { 4         }
        else { 8 }
    }

    pub fn infer_alignment_float_type(&self, value: f64) -> i64 {
        let min_f32 = f32::MIN as f64;
        let max_f32 = f32::MAX as f64;
        // let min_f64 = f64::MIN as f64;
        // let max_f64 = f64::MAX as f64;

        if (value >= min_f32) && (value <= max_f32) { 4 }
        else { 8 }
    }

    pub fn store_with_alignment<F, G, T>(
        &self, 
        codegen: &CodeGen,
        var_ptr: PointerValue,
        llvm_value: BasicValueEnum,
        try_get_const: F,
        infer_alignment: G,
    ) 
    where
        F: Fn(BasicValueEnum) -> Option<T>,
        G: Fn(T) -> i64,
    {
        if let Some(raw_val) = try_get_const(llvm_value) {
            let alignment_val = infer_alignment(raw_val);

            codegen.builder.build_store(var_ptr, llvm_value).unwrap()
                .set_alignment(alignment_val.try_into().unwrap()).unwrap();
        } else {
            codegen.builder.build_store(var_ptr, llvm_value).unwrap();
        }
    }
}

