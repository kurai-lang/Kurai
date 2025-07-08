use inkwell::{types::BasicTypeEnum, values::{BasicValueEnum, FunctionValue}};
use kurai_core::scope::Scope;

use kurai_ast::stmt::Stmt;

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
macro_rules! kurai_panic {
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
        discovered_modules: &mut Vec<String>,
        
        scope: &mut Scope,
    ) -> Option<FunctionValue<'ctx>> {
        if let Some((modname, funcname)) = Self::split_module_function_name(name) {
        // found modname and funcname? compile.
            self.get_function_from_module(
                modname,
                funcname,
                discovered_modules,
                parsers,
                scope
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
        discovered_modules: &mut Vec<String>,
        
        scope: &mut Scope,
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
                vec![],
                discovered_modules,
                parsers,
                scope
            );
        }
        // try again after compiling
        self.module.lock().unwrap().get_function(funcname)
    } 
}
